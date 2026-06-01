import {
  PetriNet,
  AutonomicController,
  AlignmentSolver,
  EWMACalculator
} from './autonomic.js';

// ==========================================================================
// 1. POSITION MAP FOR GEOMETRICAL RENDERING
// ==========================================================================

const nodePositions = {
  // Places (Circles)
  "p_start": { x: 50, y: 180, type: "place" },
  "p_ingested": { x: 170, y: 180, type: "place" },
  "p_checked": { x: 300, y: 100, type: "place" },
  "p_auth_pending": { x: 460, y: 100, type: "place" },
  "p_checked_hardened": { x: 300, y: 100, type: "place" },
  "p_escalated_auth": { x: 460, y: 100, type: "place" },
  "p_repaired": { x: 380, y: 260, type: "place" },
  "p_settled": { x: 580, y: 180, type: "place" },
  "p_decommissioned": { x: 720, y: 180, type: "place" },

  // Transitions (Rectangles)
  "t_ingest": { x: 110, y: 180, type: "trans" },
  "t_verify": { x: 230, y: 100, type: "trans" },
  "t_authorize": { x: 380, y: 100, type: "trans" },
  "t_verify_hardened": { x: 230, y: 100, type: "trans" },
  "t_escalate_auth": { x: 380, y: 100, type: "trans" },
  "t_bypass_verify": { x: 270, y: 260, type: "trans" },
  "t_settle": { x: 520, y: 180, type: "trans" },
  "t_decom": { x: 650, y: 180, type: "trans" },
  "t_anomaly": { x: 300, y: 340, type: "trans" } // Hidden/un-arc'd transition for anomaly injection
};

// ==========================================================================
// 2. DASHBOARD ORCHESTRATION ENGINE
// ==========================================================================

class SimulationDashboard {
  constructor() {
    this.net = new PetriNet();
    this.controller = new AutonomicController();
    
    // UI Elements
    this.svg = document.getElementById("petri-net-svg");
    this.btnPlayPause = document.getElementById("btn-play-pause");
    this.btnReset = document.getElementById("btn-reset");
    this.btnManualSwap = document.getElementById("btn-manual-swap");
    this.btnTriggerViolation = document.getElementById("btn-trigger-violation");
    
    this.sliderLoad = document.getElementById("slider-load");
    this.sliderDeviation = document.getElementById("slider-deviation");
    this.sliderSpeed = document.getElementById("slider-speed");
    
    this.loadVal = document.getElementById("load-val");
    this.deviationVal = document.getElementById("deviation-val");
    this.speedVal = document.getElementById("speed-val");
    
    this.valThroughput = document.getElementById("val-throughput");
    this.valFitness = document.getElementById("val-fitness");
    this.valThrottle = document.getElementById("val-throttle");
    this.valBlocks = document.getElementById("val-blocks");
    
    this.statusText = document.getElementById("system-status-text");
    this.statusDot = document.getElementById("system-status-dot");
    
    this.solverList = document.getElementById("solver-list");
    this.blockchainList = document.getElementById("blockchain-list");
    this.driftAlertBanner = document.getElementById("drift-alert-banner");
    
    // Simulation state
    this.isPlaying = true;
    this.networkLoad = 10;       // 10% base load
    this.deviationRate = 0;      // 0% base deviation injection
    this.tickSpeedMs = 1000;     // baseline speed
    this.lastTickTime = 0;
    
    this.cases = [];
    this.nextCaseId = 1;
    this.visualTokens = [];      // For rendering smooth token movements
    this.throughputCount = 0;
    this.throughputInterval = null;

    // EWMA Historical Data for Plotting
    this.fitnessHistory = Array(40).fill(1.0);
    this.loadHistory = Array(40).fill(0.10);
    this.initChart();
    
    // Initialize Net
    this.buildInitialNet();
    this.renderNetStructure();
    
    // Wire UI events
    this.setupEventListeners();
    
    // Start loop
    this.lastTime = performance.now();
    this.animateLoop(this.lastTime);
    
    // Start throughput counter reset every second
    this.throughputInterval = setInterval(() => {
      this.valThroughput.textContent = this.throughputCount;
      this.throughputCount = 0;
    }, 1000);

    // Initial render of ledger
    this.updateLedgerUI();
  }

  buildInitialNet() {
    this.net = new PetriNet();
    
    // Places
    this.net.addPlace("p_start", "Start Queue", 1);
    this.net.addPlace("p_ingested", "Transaction Ingested", 0);
    this.net.addPlace("p_checked", "Transaction Checked", 0);
    this.net.addPlace("p_auth_pending", "Authorization Pending", 0);
    this.net.addPlace("p_repaired", "Bypass Cleared", 0);
    this.net.addPlace("p_settled", "Funds Settled", 0);
    this.net.addPlace("p_decommissioned", "Ledger Archive", 0);

    // Transitions
    this.net.addTransition("t_ingest", "Ingest Tx", "t_ingest");
    this.net.addTransition("t_verify", "Standard Verify", "t_verify");
    this.net.addTransition("t_authorize", "Authorize Funds", "t_authorize");
    this.net.addTransition("t_bypass_verify", "Autonomic Bypass Path", "t_bypass_verify");
    this.net.addTransition("t_settle", "Settle Block", "t_settle");
    this.net.addTransition("t_decom", "Decommission Instance", "t_decom");

    // Arcs
    this.net.addArc("a1", "p_start", "t_ingest");
    this.net.addArc("a2", "t_ingest", "p_ingested");
    
    // Normal S-Component Arcs
    this.net.addArc("a3", "p_ingested", "t_verify");
    this.net.addArc("a4", "t_verify", "p_checked");
    this.net.addArc("a5", "p_checked", "t_authorize");
    this.net.addArc("a6", "t_authorize", "p_auth_pending");
    this.net.addArc("a7", "p_auth_pending", "t_settle");

    // Autonomic Bypass Arcs
    this.net.addArc("ab1", "p_ingested", "t_bypass_verify");
    this.net.addArc("ab2", "t_bypass_verify", "p_repaired");
    this.net.addArc("ab3", "p_repaired", "t_settle");

    // Exit Arcs
    this.net.addArc("a8", "t_settle", "p_settled");
    this.net.addArc("a9", "p_settled", "t_decom");
    this.net.addArc("a10", "t_decom", "p_decommissioned");

    // Register S-Component
    this.net.addSComponent("S_Verify_Auth", "Standard Transaction Verification & Auth", 
      ["p_checked", "p_auth_pending"], 
      ["t_verify", "t_authorize"]
    );

    // Lock standard bypass to disabled initially
    const tBypass = this.net.transitions.get("t_bypass_verify");
    tBypass.guard = () => false; 
    
    this.net.checkEnabledTransitions();
  }

  // ==========================================================================
  // 3. SVG DYNAMIC Petri Net RENDERING
  // ==========================================================================

  renderNetStructure() {
    const arcsGroup = document.getElementById("arcs-group");
    const placesGroup = document.getElementById("places-group");
    const transitionsGroup = document.getElementById("transitions-group");

    // Clear old elements
    arcsGroup.innerHTML = "";
    placesGroup.innerHTML = "";
    transitionsGroup.innerHTML = "";

    // Draw Arcs
    for (const arc of this.net.arcs) {
      const srcNode = nodePositions[arc.sourceId];
      const tgtNode = nodePositions[arc.targetId];
      if (!srcNode || !tgtNode) continue;

      const pathData = this.calculateArcLine(srcNode, tgtNode);
      const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
      path.setAttribute("d", pathData);
      path.setAttribute("id", `arc-${arc.id}`);
      path.setAttribute("class", "net-arc");
      
      // Style bypass arcs uniquely
      if (arc.id.startsWith("ab")) {
        path.style.strokeDasharray = "4,4";
      }

      arcsGroup.appendChild(path);
    }

    // Draw Places
    for (const place of this.net.places.values()) {
      const pos = nodePositions[place.id];
      if (!pos) continue;

      // Group
      const g = document.createElementNS("http://www.w3.org/2000/svg", "g");
      g.setAttribute("transform", `translate(${pos.x}, ${pos.y})`);
      g.setAttribute("id", `g-place-${place.id}`);

      // Circle
      const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
      circle.setAttribute("r", "20");
      circle.setAttribute("class", "net-place");
      
      // Color coding for S-Components
      if (place.id.includes("hardened") || place.id.includes("escalated")) {
        circle.style.stroke = "var(--amber-orange)";
      } else if (place.id === "p_checked" || place.id === "p_auth_pending") {
        circle.style.stroke = "var(--neon-cyan)";
      }

      // Label
      const text = document.createElementNS("http://www.w3.org/2000/svg", "text");
      text.setAttribute("y", "32");
      text.setAttribute("text-anchor", "middle");
      text.setAttribute("fill", "var(--text-secondary)");
      text.setAttribute("font-size", "10px");
      text.textContent = place.name;

      // Token count text
      const tokenText = document.createElementNS("http://www.w3.org/2000/svg", "text");
      tokenText.setAttribute("id", `tokens-txt-${place.id}`);
      tokenText.setAttribute("text-anchor", "middle");
      tokenText.setAttribute("dy", "4");
      tokenText.setAttribute("fill", "var(--neon-cyan)");
      tokenText.setAttribute("font-size", "12px");
      tokenText.setAttribute("font-weight", "700");
      tokenText.textContent = place.tokens > 0 ? place.tokens : "";

      g.appendChild(circle);
      g.appendChild(text);
      g.appendChild(tokenText);
      placesGroup.appendChild(g);
    }

    // Draw Transitions
    for (const trans of this.net.transitions.values()) {
      const pos = nodePositions[trans.id];
      if (!pos) continue;

      const g = document.createElementNS("http://www.w3.org/2000/svg", "g");
      g.setAttribute("transform", `translate(${pos.x}, ${pos.y})`);
      g.setAttribute("id", `g-trans-${trans.id}`);

      // Rectangle
      const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
      rect.setAttribute("x", "-12");
      rect.setAttribute("y", "-18");
      rect.setAttribute("width", "24");
      rect.setAttribute("height", "36");
      rect.setAttribute("rx", "4");
      rect.setAttribute("class", "net-transition");

      if (trans.id.includes("hardened") || trans.id.includes("escalate")) {
        rect.style.stroke = "var(--amber-orange)";
      } else if (trans.id === "t_verify" || trans.id === "t_authorize") {
        rect.style.stroke = "var(--neon-cyan)";
      } else if (trans.id === "t_bypass_verify") {
        rect.style.stroke = "var(--crimson-red)";
      }

      // Label
      const text = document.createElementNS("http://www.w3.org/2000/svg", "text");
      text.setAttribute("y", "-24");
      text.setAttribute("text-anchor", "middle");
      text.setAttribute("fill", "var(--text-secondary)");
      text.setAttribute("font-size", "10px");
      text.textContent = trans.name;

      g.appendChild(rect);
      g.appendChild(text);
      transitionsGroup.appendChild(g);
    }
  }

  calculateArcLine(source, target) {
    const dx = target.x - source.x;
    const dy = target.y - source.y;
    const len = Math.sqrt(dx * dx + dy * dy);
    if (len === 0) return "";
    
    // Node radius boundaries
    const sourceR = source.type === "place" ? 22 : 18;
    const targetR = target.type === "place" ? 22 : 18;
    
    const sx = source.x + (dx / len) * sourceR;
    const sy = source.y + (dy / len) * sourceR;
    const tx = target.x - (dx / len) * targetR;
    const ty = target.y - (dy / len) * targetR;
    
    return `M ${sx} ${sy} L ${tx} ${ty}`;
  }

  updatePlaceTokensUI(placeId, tokens) {
    const txt = document.getElementById(`tokens-txt-${placeId}`);
    if (txt) {
      txt.textContent = tokens > 0 ? tokens : "";
    }
  }

  flashTransition(transId) {
    const el = document.querySelector(`#g-trans-${transId} rect`);
    if (el) {
      el.classList.add("firing");
      setTimeout(() => {
        el.classList.remove("firing");
      }, 250);
    }
  }

  // ==========================================================================
  // 4. SMOOTH TOKEN GAME ANIMATIONS
  // ==========================================================================

  spawnVisualToken(sourceId, targetId, color = "var(--neon-cyan)") {
    const src = nodePositions[sourceId];
    const tgt = nodePositions[targetId];
    if (!src || !tgt) return;

    const tokenEl = document.createElementNS("http://www.w3.org/2000/svg", "circle");
    tokenEl.setAttribute("r", "5");
    tokenEl.setAttribute("fill", color);
    tokenEl.setAttribute("class", "net-token");
    document.getElementById("tokens-group").appendChild(tokenEl);

    this.visualTokens.push({
      element: tokenEl,
      startX: src.x,
      startY: src.y,
      endX: tgt.x,
      endY: tgt.y,
      progress: 0,
      speed: 0.05 * (parseFloat(this.sliderSpeed.value) / 10) // adapt to tick speed
    });
  }

  // ==========================================================================
  // 5. SIMULATED EXECUTION PIPELINE
  // ==========================================================================

  triggerManualViolation() {
    this.deviationRate = 100;
    this.sliderDeviation.value = 100;
    this.deviationVal.textContent = "100%";
    
    // Queue immediate violation event block
    this.controller.ledger.addEvent("VIOLATION_INJECTED", {
      injectedBy: "Operator Manual Override",
      timestamp: Date.now()
    });
    this.updateLedgerUI();
    this.throughputCount++;
    
    // Inject anomalous case immediately
    this.spawnCase(true);
  }

  spawnCase(forceViolation = false) {
    const isViolated = forceViolation || (Math.random() * 100 < this.deviationRate);
    const newCase = {
      id: this.nextCaseId++,
      currentPlace: "p_start",
      trace: [],
      isCompleted: false,
      hasViolation: isViolated,
      tokenColor: isViolated ? "var(--crimson-red)" : "var(--neon-cyan)"
    };
    
    this.cases.push(newCase);
    
    // Set token in source place
    this.net.places.get("p_start").tokens++;
    this.updatePlaceTokensUI("p_start", this.net.places.get("p_start").tokens);
  }

  stepSimulation() {
    if (!this.isPlaying) return;

    // 1. Ingestion check based on Network Load and Autonomic throttling
    const spawnChance = (this.networkLoad / 100) * this.controller.throttledRate;
    if (Math.random() < spawnChance && this.cases.length < 15) {
      this.spawnCase();
    }

    // 2. Propagate all active cases
    for (let i = this.cases.length - 1; i >= 0; i--) {
      const c = this.cases[i];
      if (c.isCompleted) continue;

      const currentPlace = c.currentPlace;
      const presets = this.net.getPostset(currentPlace); // outgoing transitions

      if (presets.length === 0) {
        // Safe check for sink places
        if (currentPlace === "p_decommissioned") {
          c.isCompleted = true;
          this.finalizeCase(c);
          this.cases.splice(i, 1);
        }
        continue;
      }

      // Selection logic for branches
      let selectedTransition = null;

      if (presets.length === 1) {
        selectedTransition = presets[0];
      } else {
        // Multi-routing branch (Place: p_ingested)
        const isBypassEnabled = this.net.transitions.get("t_bypass_verify").isEnabled;
        
        if (c.hasViolation) {
          // If violated, we trigger conformance anomalies randomly
          const rng = Math.random();
          if (rng < 0.3) {
            // SKIP VERIFY ENTIRELY: jump directly to authorization output p_checked
            c.trace.push("t_skip_verify_anomaly");
            this.moveCaseDirectly(c, "p_ingested", "p_checked");
            continue;
          } else if (rng < 0.6) {
            // OUT OF ORDER EXECUTION: authorize before verify
            c.trace.push("t_authorize");
            c.trace.push("t_verify");
            this.moveCaseDirectly(c, "p_ingested", "p_auth_pending");
            continue;
          } else {
            // UNAUTHORIZED BYPASS: use bypass path when not autonomically authorized
            if (!isBypassEnabled) {
              selectedTransition = "t_bypass_verify"; 
            } else {
              selectedTransition = "t_verify";
            }
          }
        } else {
          // Normal case routing
          if (isBypassEnabled) {
            selectedTransition = "t_bypass_verify";
          } else {
            // If hot-swapped, we fire the new transition ID
            if (this.net.transitions.has("t_verify_hardened")) {
              selectedTransition = "t_verify_hardened";
            } else {
              selectedTransition = "t_verify";
            }
          }
        }
      }

      // Safety check: is transition defined?
      if (!selectedTransition || !this.net.transitions.has(selectedTransition)) {
        selectedTransition = presets[0]; // fallback
      }

      // Check transition inputs and fire
      const trans = this.net.transitions.get(selectedTransition);
      const postsets = this.net.getPostset(selectedTransition);
      const targetPlace = postsets[0]; // assume single output place for simplification

      // Decrement source place token
      const srcPlace = this.net.places.get(currentPlace);
      if (srcPlace && srcPlace.tokens > 0) {
        srcPlace.tokens--;
        this.updatePlaceTokensUI(currentPlace, srcPlace.tokens);

        // Flash and spawn anim token
        this.flashTransition(selectedTransition);
        this.spawnVisualToken(currentPlace, selectedTransition, c.tokenColor);
        
        // After delay (half tick), token moves from transition to target place
        setTimeout(() => {
          this.spawnVisualToken(selectedTransition, targetPlace, c.tokenColor);
          const tgtPlace = this.net.places.get(targetPlace);
          if (tgtPlace) {
            tgtPlace.tokens++;
            this.updatePlaceTokensUI(targetPlace, tgtPlace.tokens);
          }
        }, 150 * (10 / parseFloat(this.sliderSpeed.value)));

        // Record trace event
        c.trace.push(trans.label);
        c.currentPlace = targetPlace;
        this.throughputCount++;
      }
    }
  }

  moveCaseDirectly(c, sourcePlaceId, targetPlaceId) {
    const srcPlace = this.net.places.get(sourcePlaceId);
    if (srcPlace && srcPlace.tokens > 0) {
      srcPlace.tokens--;
      this.updatePlaceTokensUI(sourcePlaceId, srcPlace.tokens);

      this.spawnVisualToken(sourcePlaceId, targetPlaceId, c.tokenColor);
      
      setTimeout(() => {
        const tgtPlace = this.net.places.get(targetPlaceId);
        if (tgtPlace) {
          tgtPlace.tokens++;
          this.updatePlaceTokensUI(targetPlaceId, tgtPlace.tokens);
        }
      }, 300 * (10 / parseFloat(this.sliderSpeed.value)));

      c.currentPlace = targetPlaceId;
      this.throughputCount++;
    }
  }

  finalizeCase(c) {
    // Decrement sink place token to clear log instance
    const sinkPlace = this.net.places.get("p_decommissioned");
    if (sinkPlace && sinkPlace.tokens > 0) {
      sinkPlace.tokens--;
      this.updatePlaceTokensUI("p_decommissioned", sinkPlace.tokens);
    }

    // Run actual A* conformance solver
    const alignResult = AlignmentSolver.solve(c.trace, this.net, "p_start", "p_decommissioned");
    
    // Update solver UI view
    this.updateSolverLogUI(c.trace, alignResult);

    // Call autonomic feedback loop check
    const metrics = {
      lastTraceFitness: alignResult.fitness,
      currentNetworkLoad: this.networkLoad / 100
    };

    const decisions = this.controller.tick(metrics, this.net);
    
    // Update live metrics dashboard
    this.valFitness.textContent = decisions.metrics.ewmaFitness.toFixed(4);
    this.valThrottle.textContent = `${Math.round(this.controller.throttledRate * 100)}%`;

    // Render chart history
    this.fitnessHistory.push(decisions.metrics.ewmaFitness);
    this.fitnessHistory.shift();
    this.loadHistory.push(decisions.metrics.ewmaLoad);
    this.loadHistory.shift();
    this.drawChart();

    // Actuate structural hot-swap or routing adjustments if triggered by autonomic planner
    if (decisions.actions.length > 0) {
      this.updateLedgerUI();
      
      // Check if S-component hot-swap happened, redraw SVG if needed
      const hasSwap = decisions.actions.some(a => a.type === "S_COMPONENT_HOT_SWAP" || a.type === "S_COMPONENT_REVERT");
      if (hasSwap) {
        this.renderNetStructure();
      }

      this.updateStatusIndicator();
    }
  }

  // ==========================================================================
  // 6. UI UPDATES & EVENT LISTENERS
  // ==========================================================================

  setupEventListeners() {
    // Play / Pause
    this.btnPlayPause.addEventListener("click", () => {
      this.isPlaying = !this.isPlaying;
      this.btnPlayPause.textContent = this.isPlaying ? "PAUSE" : "PLAY";
      this.btnPlayPause.className = this.isPlaying ? "btn-cyan" : "btn-emerald";
    });

    // Reset
    this.btnReset.addEventListener("click", () => {
      this.buildInitialNet();
      this.renderNetStructure();
      this.cases = [];
      this.visualTokens = [];
      document.getElementById("tokens-group").innerHTML = "";
      this.throughputCount = 0;
      
      this.controller = new AutonomicController();
      this.fitnessHistory = Array(40).fill(1.0);
      this.loadHistory = Array(40).fill(0.10);
      this.drawChart();
      
      this.valFitness.textContent = "1.000";
      this.valThrottle.textContent = "100%";
      this.updateStatusIndicator();
      this.updateLedgerUI();
      
      this.solverList.innerHTML = `
        <div class="solver-step-card match">
          <span style="font-weight: 700; color: var(--emerald-green);">NOMINAL</span>
          <span class="font-mono">Engine reset completed. Monitoring active.</span>
        </div>`;
    });

    // Manual Hot-Swap
    this.btnManualSwap.addEventListener("click", () => {
      try {
        if (!this.controller.hotSwapExecuted) {
          this.net.hotSwapSComponent("S_Verify_Auth", this.controller.hardenedVerificationComponent);
          this.controller.hotSwapExecuted = true;
          this.controller.ledger.addEvent("S_COMPONENT_HOT_SWAP", {
            injectedBy: "Operator Manual Override",
            message: "Hot-swapped to Hardened Autonomic S-Component (Secure Routing)"
          });
        } else {
          this.net.hotSwapSComponent("S_Verify_Auth", this.controller.standardVerificationComponent);
          this.controller.hotSwapExecuted = false;
          this.controller.ledger.addEvent("S_COMPONENT_REVERT", {
            injectedBy: "Operator Manual Override",
            message: "Reverted to Standard S-Component"
          });
        }
        this.renderNetStructure();
        this.updateLedgerUI();
        this.updateStatusIndicator();
      } catch (err) {
        console.error("Manual S-component swap error:", err);
      }
    });

    // Trigger Violation
    this.btnTriggerViolation.addEventListener("click", () => {
      this.triggerManualViolation();
    });

    // Sliders
    this.sliderLoad.addEventListener("input", (e) => {
      this.networkLoad = parseInt(e.target.value);
      this.loadVal.textContent = `${this.networkLoad}%`;
    });

    this.sliderDeviation.addEventListener("input", (e) => {
      this.deviationRate = parseInt(e.target.value);
      this.deviationVal.textContent = `${this.deviationRate}%`;
    });

    this.sliderSpeed.addEventListener("input", (e) => {
      const val = parseFloat(e.target.value) / 10;
      this.speedVal.textContent = `${val.toFixed(1)}x`;
      this.tickSpeedMs = 1000 / val;
    });
  }

  updateStatusIndicator() {
    let status = "NOMINAL (SECURE)";
    let colorClass = "emerald";
    
    if (this.controller.isThrottled) {
      status = `THROTTLED (${Math.round(this.controller.throttledRate * 100)}%)`;
      colorClass = "amber";
    }
    
    if (this.controller.routeBypassActive) {
      status = "SECURE BYPASS ACTIVE";
      colorClass = "crimson";
    }

    if (this.controller.hotSwapExecuted) {
      status = "HARDENED S-NET ACTIVE";
      colorClass = "amber";
    }

    this.statusText.textContent = status;
    this.statusText.style.color = `var(--${colorClass === "emerald" ? "emerald-green" : colorClass === "amber" ? "amber-orange" : "crimson-red"})`;
    
    this.statusDot.className = `status-dot ${colorClass} pulse`;

    if (this.controller.fitnessEWMA.getVal() < 0.90) {
      this.driftAlertBanner.style.display = "flex";
    } else {
      this.driftAlertBanner.style.display = "none";
    }
  }

  updateLedgerUI() {
    const chain = this.controller.ledger.chain;
    this.valBlocks.textContent = chain.length;

    this.blockchainList.innerHTML = "";
    
    // Display newest blocks at the top
    for (let i = chain.length - 1; i >= 0; i--) {
      const block = chain[i];
      const node = document.createElement("div");
      node.className = "blockchain-node";

      let statusColor = "var(--neon-cyan)";
      if (block.eventType.includes("VIOLATION") || block.eventType.includes("CRITICAL")) {
        statusColor = "var(--crimson-red)";
      } else if (block.eventType.includes("SWAP") || block.eventType.includes("TRIGGER")) {
        statusColor = "var(--amber-orange)";
      } else if (block.eventType.includes("RECOVERY") || block.eventType.includes("NOMINAL")) {
        statusColor = "var(--emerald-green)";
      }

      node.innerHTML = `
        <div style="display: flex; justify-content: space-between; border-bottom: 1px solid var(--border-glass); padding-bottom: 4px; margin-bottom: 4px;">
          <span style="font-weight: 700; color: ${statusColor}">[Block #${block.index}] ${block.eventType}</span>
          <span style="color: var(--text-muted); font-size: 0.65rem;">${new Date(block.timestamp).toLocaleTimeString()}</span>
        </div>
        <div style="color: var(--text-secondary); margin-bottom: 4px;">
          ${Object.entries(block.data).map(([k, v]) => `<strong>${k}:</strong> ${JSON.stringify(v)}`).join("<br/>")}
        </div>
        <div style="font-family: var(--font-mono); font-size: 0.65rem; border-top: 1px dashed var(--border-glass); padding-top: 4px; display: flex; flex-direction: column;">
          <div><span class="hash-label">Prev:</span> <span style="color: var(--text-muted);">${block.previousHash.slice(0, 24)}...</span></div>
          <div><span class="hash-label">Hash:</span> <span class="hash-value">${block.hash.slice(0, 24)}...</span></div>
        </div>
      `;
      this.blockchainList.appendChild(node);
    }
  }

  updateSolverLogUI(trace, alignResult) {
    this.solverList.innerHTML = "";
    
    const hInfo = document.createElement("div");
    hInfo.style.cssText = "font-size: 0.75rem; color: var(--text-secondary); margin-bottom: 8px; border-bottom: 1px solid var(--border-glass); padding-bottom: 4px;";
    hInfo.innerHTML = `
      <strong>Trace:</strong> [${trace.join(", ")}]<br/>
      <strong>Alignment Cost:</strong> ${alignResult.cost} | <strong>Fitness Score:</strong> ${alignResult.fitness.toFixed(4)}
    `;
    this.solverList.appendChild(hInfo);

    for (const move of alignResult.path) {
      const card = document.createElement("div");
      
      let moveClass = "match";
      let title = "SYNC MOVE";
      let desc = `Observed activity '${move.label}' aligned with model transition '${move.transId || move.label}'`;

      if (move.type === "model") {
        moveClass = "move-model";
        title = "MODEL-ONLY MOVE";
        desc = `Model forced to fire '${move.label}' (missing in execution log)`;
      } else if (move.type === "log") {
        moveClass = "move-log";
        title = "LOG-ONLY MOVE";
        desc = `Observed log event '${move.label}' not permitted by Petri net at this point`;
      } else if (move.type === "failure") {
        moveClass = "move-log";
        title = "SOLVER LIMIT HIT";
        desc = move.label;
      }

      card.className = `solver-step-card ${moveClass}`;
      card.innerHTML = `
        <div style="display: flex; justify-content: space-between; font-weight: 700;">
          <span>${title}</span>
          <span class="font-mono" style="font-size: 0.7rem;">Cost: ${move.type === 'sync' ? '0' : '1'}</span>
        </div>
        <div style="color: var(--text-secondary); font-size: 0.75rem;">${desc}</div>
      `;
      this.solverList.appendChild(card);
    }
  }

  // ==========================================================================
  // 7. DRIFT CHART CANVAS RENDERING
  // ==========================================================================

  initChart() {
    this.canvas = document.getElementById("drift-canvas");
    this.ctx = this.canvas.getContext("2d");
    this.resizeCanvas();
    window.addEventListener("resize", () => this.resizeCanvas());
  }

  resizeCanvas() {
    const rect = this.canvas.parentElement.getBoundingClientRect();
    this.canvas.width = rect.width;
    this.canvas.height = rect.height || 160;
    this.drawChart();
  }

  drawChart() {
    if (!this.ctx) return;
    const w = this.canvas.width;
    const h = this.canvas.height;
    const ctx = this.ctx;

    ctx.clearRect(0, 0, w, h);

    // Draw Grid lines
    ctx.strokeStyle = "rgba(255, 255, 255, 0.04)";
    ctx.lineWidth = 1;
    for (let i = 0; i < 4; i++) {
      const y = (h / 4) * i;
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(w, y);
      ctx.stroke();
    }

    // Draw threshold boundary line (fitness = 0.90)
    const thresholdY = h - (0.90 * h);
    ctx.strokeStyle = "rgba(239, 68, 68, 0.35)";
    ctx.lineWidth = 1.5;
    ctx.setLineDash([6, 4]);
    ctx.beginPath();
    ctx.moveTo(0, thresholdY);
    ctx.lineTo(w, thresholdY);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = "rgba(239, 68, 68, 0.5)";
    ctx.font = "9px JetBrains Mono";
    ctx.fillText("THRESHOLD (0.90)", 10, thresholdY - 4);

    // Plot Fitness line
    const dataLen = this.fitnessHistory.length;
    ctx.beginPath();
    ctx.lineWidth = 2.5;
    ctx.strokeStyle = "var(--neon-cyan)";
    ctx.shadowBlur = 10;
    ctx.shadowColor = "var(--neon-cyan-glow)";

    for (let i = 0; i < dataLen; i++) {
      const x = (w / (dataLen - 1)) * i;
      const y = h - (this.fitnessHistory[i] * h);
      
      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    }
    ctx.stroke();
    ctx.shadowBlur = 0; // reset shadow

    // Fill underneath the fitness curve
    ctx.fillStyle = "rgba(0, 242, 254, 0.03)";
    ctx.lineTo(w, h);
    ctx.lineTo(0, h);
    ctx.closePath();
    ctx.fill();

    // Plot Load line (dotted yellow)
    ctx.beginPath();
    ctx.lineWidth = 1.5;
    ctx.strokeStyle = "rgba(245, 158, 11, 0.65)";
    for (let i = 0; i < dataLen; i++) {
      const x = (w / (dataLen - 1)) * i;
      const y = h - (this.loadHistory[i] * h);
      
      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    }
    ctx.stroke();
  }

  // ==========================================================================
  // 8. TICK ANIMATION LOOP
  // ==========================================================================

  animateLoop(time) {
    requestAnimationFrame((t) => this.animateLoop(t));

    const delta = time - this.lastTime;
    this.lastTime = time;

    // Update visual token positions
    const tokensGroup = document.getElementById("tokens-group");
    for (let i = this.visualTokens.length - 1; i >= 0; i--) {
      const t = this.visualTokens[i];
      t.progress += t.speed;
      
      if (t.progress >= 1.0) {
        // Complete movement, remove visual circle
        tokensGroup.removeChild(t.element);
        this.visualTokens.splice(i, 1);
      } else {
        const curX = t.startX + (t.endX - t.startX) * t.progress;
        const curY = t.startY + (t.endY - t.startY) * t.progress;
        t.element.setAttribute("cx", curX);
        t.element.setAttribute("cy", curY);
      }
    }

    // Handle simulation pacing
    this.lastTickTime += delta;
    if (this.lastTickTime >= this.tickSpeedMs) {
      this.lastTickTime = 0;
      this.stepSimulation();
    }
  }
}

// Instantiate dashboard on document load
window.addEventListener("DOMContentLoaded", () => {
  window.dashboard = new SimulationDashboard();
});
