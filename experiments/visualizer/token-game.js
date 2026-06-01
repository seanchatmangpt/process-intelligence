/**
 * ==========================================================================
 * Petri Net Token Game Simulator & Process Map Renderer
 * ==========================================================================
 */

class TokenGameSimulator {
  constructor() {
    this.canvas = document.getElementById("token-game-canvas");
    this.ctx = this.canvas ? this.canvas.getContext("2d") : null;
    
    // Process model layout coordinates
    this.places = [
      { id: 0, name: "P1", label: "Start", x: 60, y: 200, tokens: 1 },
      { id: 1, name: "P2", label: "Order Rec'd", x: 220, y: 200, tokens: 0 },
      { id: 2, name: "P3", label: "Inv. Checked", x: 420, y: 110, tokens: 0 },
      { id: 3, name: "P4", label: "Approved", x: 580, y: 200, tokens: 0 },
      { id: 4, name: "P5", label: "Shipped", x: 700, y: 200, tokens: 0 },
      { id: 5, name: "P6", label: "End", x: 800, y: 200, tokens: 0 }
    ];

    this.transitions = [
      { id: 0, name: "t1", label: "Receive Order", x: 140, y: 200, w: 20, h: 40, active: false },
      { id: 1, name: "t2", label: "Check Inventory", x: 320, y: 110, w: 20, h: 40, active: false },
      { id: 2, name: "t3", label: "Approve", x: 500, y: 110, w: 20, h: 40, active: false },
      { id: 3, name: "t4", label: "Approve (Bypass)", x: 400, y: 275, w: 40, h: 20, active: false },
      { id: 4, name: "t5", label: "Ship", x: 640, y: 200, w: 20, h: 40, active: false },
      { id: 5, name: "t6", label: "End", x: 750, y: 200, w: 20, h: 40, active: false }
    ];

    // Connect dependencies: source -> target
    this.arcs = [
      { from: { type: "place", id: 0 }, to: { type: "trans", id: 0 } },
      { from: { type: "trans", id: 0 }, to: { type: "place", id: 1 } },
      
      { from: { type: "place", id: 1 }, to: { type: "trans", id: 1 } },
      { from: { type: "trans", id: 1 }, to: { type: "place", id: 2 } },
      
      { from: { type: "place", id: 2 }, to: { type: "trans", id: 2 } },
      { from: { type: "trans", id: 2 }, to: { type: "place", id: 3 } },
      
      // Bypass route
      { from: { type: "place", id: 1 }, to: { type: "trans", id: 3 } },
      { from: { type: "trans", id: 3 }, to: { type: "place", id: 3 } },
      
      { from: { type: "place", id: 3 }, to: { type: "trans", id: 4 } },
      { from: { type: "trans", id: 4 }, to: { type: "place", id: 4 } },
      
      { from: { type: "place", id: 4 }, to: { type: "trans", id: 5 } },
      { from: { type: "trans", id: 5 }, to: { type: "place", id: 5 } }
    ];

    // Simulation states
    this.activeCases = new Map(); // caseId -> CaseState
    this.flyingTokens = []; // Tokens currently in transit
    this.completedCount = 0;
    this.totalDuration = 0;
    
    this.init();
  }

  init() {
    this.resizeCanvas();
    this.animationLoop();
    window.addEventListener("resize", () => this.resizeCanvas());

    // Controls
    document.getElementById("btn-token-reset")?.addEventListener("click", () => this.resetMap());
  }

  resizeCanvas() {
    if (!this.canvas) return;
    const rect = this.canvas.parentElement.getBoundingClientRect();
    this.canvas.width = rect.width * window.devicePixelRatio;
    this.canvas.height = rect.height * window.devicePixelRatio;
    if (this.ctx) {
      this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    }
  }

  resetMap() {
    this.places.forEach(p => p.tokens = 0);
    this.places[0].tokens = 1; // P1 Start
    this.flyingTokens = [];
    this.activeCases.clear();
    this.render();
    this.updateMarkingVector();
  }

  updateMarkingVector() {
    const vectorStr = `[${this.places.map(p => `${p.name}:${p.tokens}`).join(", ")}]`;
    const markerEl = document.getElementById("marking-vector");
    if (markerEl) markerEl.textContent = vectorStr;
  }

  // Inject a new case into the simulator
  startCase(caseId, isDeviant = false, deviantType = "") {
    // Generate case color
    const colors = ["#3b82f6", "#a855f7", "#10b981", "#f59e0b", "#ec4899", "#06b6d4"];
    const color = colors[Math.floor(Math.random() * colors.length)];
    
    // Choose path
    let activityPath = [];
    let expectedDuration = 5.0; // Base duration

    const isDriftActive = document.getElementById("toggle-drift-injection")?.checked;
    if (isDriftActive) {
      // In concept drift, the process slows down significantly
      expectedDuration = 9.5; 
    }

    if (isDeviant) {
      if (deviantType === "deviant_skipped_approval") {
        activityPath = ["Receive Order", "Check Inventory", "Ship", "End"];
      } else if (deviantType === "deviant_extra_ship") {
        activityPath = ["Receive Order", "Check Inventory", "Approve", "Ship", "Ship", "End"];
      } else if (deviantType === "deviant_illegal_activity") {
        activityPath = ["Receive Order", "Refund", "Ship", "End"];
      }
    } else {
      // Normal path: either Standard or Check-Bypass
      if (Math.random() < 0.4) {
        activityPath = ["Receive Order", "Approve", "Ship", "End"]; // Check inventory bypass
      } else {
        activityPath = ["Receive Order", "Check Inventory", "Approve", "Ship", "End"]; // Standard conforming
      }
    }

    const caseState = {
      caseId,
      color,
      isDeviant,
      deviantType,
      activityPath,
      currentStep: 0,
      history: [],
      startTime: Date.now(),
      expectedDuration: expectedDuration
    };

    this.activeCases.set(caseId, caseState);
    this.places[0].tokens++;
    this.updateMarkingVector();
    this.triggerNextStep(caseId);
  }

  triggerNextStep(caseId) {
    const cs = this.activeCases.get(caseId);
    if (!cs) return;

    if (cs.currentStep >= cs.activityPath.length) {
      // Case Finished
      this.completeCase(caseId);
      return;
    }

    const nextActivity = cs.activityPath[cs.currentStep];
    cs.history.push(nextActivity);
    cs.currentStep++;

    // Translate activity into a transition firing
    let matchingTransId = -1;
    let inputPlaceId = -1;
    let outputPlaceId = -1;

    // Find place currently holding this case's token
    // For visual simplicity, we tie places to the current state index:
    // P1 (Start) -> t1 -> P2
    // P2 -> t2 -> P3 -> t3 -> P4 OR P2 -> t4 -> P4
    // P4 -> t5 -> P5 -> t6 -> P6
    
    if (nextActivity === "Receive Order") {
      matchingTransId = 0; inputPlaceId = 0; outputPlaceId = 1;
    } else if (nextActivity === "Check Inventory") {
      matchingTransId = 1; inputPlaceId = 1; outputPlaceId = 2;
    } else if (nextActivity === "Approve") {
      // Check which route
      if (cs.history.includes("Check Inventory")) {
        matchingTransId = 2; inputPlaceId = 2; outputPlaceId = 3;
      } else {
        matchingTransId = 3; inputPlaceId = 1; outputPlaceId = 3; // Bypass
      }
    } else if (nextActivity === "Ship") {
      // Check if we are doing double shipping
      if (cs.deviantType === "deviant_extra_ship" && cs.history.filter(a => a === "Ship").length === 1) {
        // First ship of double ship - stays in P4
        matchingTransId = 4; inputPlaceId = 3; outputPlaceId = 3;
      } else {
        matchingTransId = 4; inputPlaceId = 3; outputPlaceId = 4;
      }
    } else if (nextActivity === "End") {
      matchingTransId = 5; inputPlaceId = 4; outputPlaceId = 5;
    } else {
      // Deviant/unsupported activities (e.g. Refund, or Skipped Approval where t5 fires from P2/P3 directly)
      if (nextActivity === "Refund") {
        // Move on log deviant activity. We can show it jumping from P2 to P4
        matchingTransId = 3; inputPlaceId = 1; outputPlaceId = 3;
      } else if (cs.deviantType === "deviant_skipped_approval") {
        // Jump from Check Inventory (P3) or Order Rec (P2) directly to P4 or P5
        matchingTransId = 4; inputPlaceId = 2; outputPlaceId = 4;
      }
    }

    if (matchingTransId !== -1 && inputPlaceId !== -1) {
      this.fireTransition(matchingTransId, inputPlaceId, outputPlaceId, cs.color, () => {
        // Step complete callback
        this.triggerNextStep(caseId);
      });
    } else {
      // Fallback if no matching transition (e.g. deviant step couldn't fire visual model)
      setTimeout(() => this.triggerNextStep(caseId), 300);
    }
  }

  fireTransition(transId, fromPlaceId, toPlaceId, tokenColor, onComplete) {
    const transition = this.transitions[transId];
    const fromPlace = this.places[fromPlaceId];
    const toPlace = this.places[toPlaceId];

    // Temporarily reduce token in source place
    if (fromPlace.tokens > 0) fromPlace.tokens--;
    
    // Flash transition
    transition.active = true;
    setTimeout(() => transition.active = false, 150);

    // Create flying token
    const token = {
      x: fromPlace.x,
      y: fromPlace.y,
      targetX: toPlace.x,
      targetY: toPlace.y,
      color: tokenColor,
      progress: 0,
      speed: 0.05, // progress increment per frame
      onReach: () => {
        toPlace.tokens++;
        this.updateMarkingVector();
        onComplete();
      }
    };

    this.flyingTokens.push(token);
  }

  completeCase(caseId) {
    const cs = this.activeCases.get(caseId);
    if (!cs) return;

    // Remove token from end place
    if (this.places[5].tokens > 0) this.places[5].tokens--;
    this.updateMarkingVector();

    // Calculate throughput duration
    const actualDurationMs = Date.now() - cs.startTime;
    
    // Map ms to process simulation scale seconds
    // Let's say 1 tick in simulation represents 1 second
    const simulatedDuration = window.driftEngine.generateNormalRandom(cs.expectedDuration, window.driftEngine.sigma0);

    // 1. Update Drift Detector
    window.driftEngine.processPoint(simulatedDuration);
    window.driftEngine.render();

    // 2. Update Ledger with case trace
    cs.history.forEach((act, stepIdx) => {
      window.ledgerEngine.addEvent(caseId, act, {
        step: stepIdx + 1,
        duration: (simulatedDuration / cs.history.length).toFixed(2)
      });
    });

    // 3. Update Alignment Solver if auto-alignment is on
    const solveAuto = document.getElementById("toggle-auto-alignment")?.checked;
    if (solveAuto) {
      const solverResult = window.alignmentSolver.solve(cs.history);
      
      // Update UI metrics
      this.completedCount++;
      this.totalDuration += simulatedDuration;
      
      const compCasesEl = document.getElementById("metric-completed-cases");
      const avgDurEl = document.getElementById("metric-avg-duration");
      const avgFitEl = document.getElementById("metric-avg-fitness");
      
      if (compCasesEl) compCasesEl.textContent = this.completedCount;
      if (avgDurEl) avgDurEl.textContent = (this.totalDuration / this.completedCount).toFixed(1) + "s";
      if (avgFitEl) {
        // Average alignment fitness
        const currentFitness = parseFloat(solverResult.fitness);
        const prevFitnessSum = parseFloat(avgFitEl.textContent) * (this.completedCount - 1) || 1.0;
        avgFitEl.textContent = ((prevFitnessSum + currentFitness) / this.completedCount).toFixed(2);
      }

      // Display the completed alignment in A* Solver panel for visual feedback
      this.displayAlignment(cs.history, solverResult);
    }

    // Remove case
    this.activeCases.delete(caseId);
    document.getElementById("active-tokens-count").textContent = this.activeCases.size;
  }

  displayAlignment(logTrace, result) {
    const logCells = document.getElementById("log-trace-cells");
    const modelCells = document.getElementById("model-trace-cells");
    const opCells = document.getElementById("operation-cells");
    
    const nodesExp = document.getElementById("solver-nodes-expanded");
    const solDur = document.getElementById("solver-duration");
    const solCost = document.getElementById("solver-total-cost");

    if (nodesExp) nodesExp.textContent = result.nodesExpanded;
    if (solDur) solDur.textContent = result.duration + "ms";
    if (solCost) solCost.textContent = result.cost;

    if (!logCells || !modelCells || !opCells) return;

    logCells.innerHTML = "";
    modelCells.innerHTML = "";
    opCells.innerHTML = "";

    result.alignment.forEach(step => {
      // Log trace cell
      const logCell = document.createElement("div");
      logCell.className = `align-cell ${step.type === 'model' ? 'skipped' : step.type}`;
      logCell.textContent = step.logAct;
      logCells.appendChild(logCell);

      // Model trace cell
      const modelCell = document.createElement("div");
      modelCell.className = `align-cell ${step.type === 'log' ? 'skipped' : step.type}`;
      modelCell.textContent = step.modelAct;
      modelCells.appendChild(modelCell);

      // Operation cell
      const opCell = document.createElement("div");
      opCell.className = "align-cell cell-op";
      opCell.textContent = step.type;
      opCells.appendChild(opCell);
    });
  }

  // Animation Loop
  animationLoop() {
    this.updateTokensPosition();
    this.render();
    requestAnimationFrame(() => this.animationLoop());
  }

  updateTokensPosition() {
    for (let i = this.flyingTokens.length - 1; i >= 0; i--) {
      const token = this.flyingTokens[i];
      token.progress += token.speed;
      
      // Interpolation (ease-in-out)
      const t = token.progress;
      token.x = token.x + (token.targetX - token.x) * t;
      token.y = token.y + (token.targetY - token.y) * t;

      if (token.progress >= 1.0) {
        token.onReach();
        this.flyingTokens.splice(i, 1);
      }
    }
  }

  render() {
    if (!this.ctx || !this.canvas) return;
    const width = this.canvas.width / window.devicePixelRatio;
    const height = this.canvas.height / window.devicePixelRatio;
    const ctx = this.ctx;

    ctx.clearRect(0, 0, width, height);

    // 1. Draw Arcs (Arrows)
    ctx.strokeStyle = "rgba(255, 255, 255, 0.15)";
    ctx.lineWidth = 2;
    this.arcs.forEach(arc => {
      const start = arc.from.type === "place" ? this.places[arc.from.id] : this.transitions[arc.from.id];
      const end = arc.to.type === "place" ? this.places[arc.to.id] : this.transitions[arc.to.id];
      
      this.drawArrow(ctx, start.x, start.y, end.x, end.y);
    });

    // 2. Draw Transitions (Boxes)
    this.transitions.forEach(t => {
      ctx.fillStyle = t.active ? "rgba(168, 85, 247, 0.6)" : "rgba(255, 255, 255, 0.05)";
      ctx.strokeStyle = t.active ? "var(--color-purple)" : "rgba(255, 255, 255, 0.25)";
      ctx.lineWidth = 1.5;
      
      ctx.beginPath();
      ctx.rect(t.x - t.w/2, t.y - t.h/2, t.w, t.h);
      ctx.fill();
      ctx.stroke();

      // Glowing border on active transition
      if (t.active) {
        ctx.shadowColor = "var(--color-purple)";
        ctx.shadowBlur = 10;
        ctx.stroke();
        ctx.shadowBlur = 0; // reset
      }

      // Transition label
      ctx.fillStyle = "var(--text-muted)";
      ctx.font = "10px var(--font-sans)";
      ctx.textAlign = "center";
      ctx.fillText(t.label, t.x, t.y - t.h/2 - 6);
    });

    // 3. Draw Places (Circles)
    this.places.forEach(p => {
      ctx.fillStyle = "rgba(10, 15, 30, 0.85)";
      ctx.strokeStyle = p.tokens > 0 ? "var(--color-blue)" : "rgba(255, 255, 255, 0.25)";
      ctx.lineWidth = 2;

      ctx.beginPath();
      ctx.arc(p.x, p.y, 20, 0, 2 * Math.PI);
      ctx.fill();
      ctx.stroke();

      // Glow on active places
      if (p.tokens > 0) {
        ctx.shadowColor = "var(--color-blue)";
        ctx.shadowBlur = 8;
        ctx.stroke();
        ctx.shadowBlur = 0;
      }

      // Place labels
      ctx.fillStyle = "#fff";
      ctx.font = "11px var(--font-sans)";
      ctx.fontWeight = "600";
      ctx.textAlign = "center";
      ctx.fillText(p.name, p.x, p.y + 4);

      ctx.fillStyle = "var(--text-muted)";
      ctx.font = "9px var(--font-sans)";
      ctx.fillText(p.label, p.x, p.y + 32);

      // Render static token counts inside places
      if (p.tokens > 0) {
        ctx.fillStyle = "var(--color-blue)";
        ctx.beginPath();
        ctx.arc(p.x - 8, p.y - 8, 4, 0, 2 * Math.PI);
        ctx.fill();
        
        if (p.tokens > 1) {
          ctx.fillStyle = "#fff";
          ctx.font = "9px var(--font-mono)";
          ctx.fillText(`+${p.tokens - 1}`, p.x + 8, p.y - 5);
        }
      }
    });

    // 4. Draw Flying Tokens
    this.flyingTokens.forEach(token => {
      ctx.fillStyle = token.color;
      ctx.shadowColor = token.color;
      ctx.shadowBlur = 8;
      
      ctx.beginPath();
      ctx.arc(token.x, token.y, 6, 0, 2 * Math.PI);
      ctx.fill();
      
      ctx.shadowBlur = 0; // reset
    });
  }

  drawArrow(ctx, fromx, fromy, tox, toy) {
    const headlen = 8; // length of head in pixels
    const dx = tox - fromx;
    const dy = toy - fromy;
    const angle = Math.atan2(dy, dx);
    
    // Draw arc line from edge of start circle/box to end circle/box
    // For simplicity, we calculate the vectors
    const offset = 22; // Offset to avoid overlapping inside the circles
    const sx = fromx + Math.cos(angle) * offset;
    const sy = fromy + Math.sin(angle) * offset;
    const ex = tox - Math.cos(angle) * offset;
    const ey = toy - Math.sin(angle) * offset;

    ctx.beginPath();
    ctx.moveTo(sx, sy);
    ctx.lineTo(ex, ey);
    ctx.stroke();
    
    // Draw arrow head
    ctx.beginPath();
    ctx.moveTo(ex, ey);
    ctx.lineTo(ex - headlen * Math.cos(angle - Math.PI / 6), ey - headlen * Math.sin(angle - Math.PI / 6));
    ctx.lineTo(ex - headlen * Math.cos(angle + Math.PI / 6), ey - headlen * Math.sin(angle + Math.PI / 6));
    ctx.closePath();
    ctx.fillStyle = "rgba(255, 255, 255, 0.25)";
    ctx.fill();
  }
}

// Global initialization
window.tokenEngine = new TokenGameSimulator();
