/**
 * ==========================================================================
 * Main Application Orchestrator
 * ==========================================================================
 */

class AppOrchestrator {
  constructor() {
    this.simInterval = null;
    this.simSpeed = 800; // Tick rate in ms
    this.noiseRate = 15; // Percent chance of deviant case
    this.isPlaying = false;
    this.caseCounter = 1000;
    
    this.init();
  }

  init() {
    // 1. Connect Logging Stream
    window.ledgerEngine.onLogMessage = (msg) => this.writeToLiveLog(msg);
    
    // 2. Setup Event Listeners
    this.setupControlPanelListeners();
    this.setupAlignmentPanelListeners();
    this.setupActionListeners();

    // 3. Initialize metrics displays
    this.writeToLiveLog("[System Startup] Process Intelligence Engine Initialized.");
    this.writeToLiveLog("[System Startup] SHA-256 Ledger online. A* Alignment solver loaded.");
    this.writeToLiveLog("[System Startup] EWMA Concept Drift limits initialized.");

    // Initial render
    window.ledgerEngine.verifyAndRender();
    window.driftEngine.render();
    window.tokenEngine.render();

    // Set speed & noise displays
    document.getElementById("speed-val").textContent = this.simSpeed + "ms";
    document.getElementById("noise-val").textContent = this.noiseRate + "%";
  }

  writeToLiveLog(message) {
    const logEl = document.getElementById("live-ledger-log");
    if (!logEl) return;
    
    const timestamp = new Date().toLocaleTimeString();
    logEl.innerHTML += `\n[${timestamp}] ${message}`;
    logEl.scrollTop = logEl.scrollHeight;
  }

  setupControlPanelListeners() {
    const btnPlay = document.getElementById("btn-global-play");
    const btnPause = document.getElementById("btn-global-pause");
    const btnStep = document.getElementById("btn-global-step");
    const btnReset = document.getElementById("btn-global-reset");
    
    const speedSlider = document.getElementById("simulation-speed");
    const noiseSlider = document.getElementById("noise-rate");

    // Play button
    btnPlay?.addEventListener("click", () => {
      if (this.isPlaying) return;
      this.isPlaying = true;
      btnPlay.classList.add("active");
      btnPause?.classList.remove("active");
      
      this.simInterval = setInterval(() => this.tickSimulation(), this.simSpeed);
      this.writeToLiveLog("▶️ Global simulation started.");
    });

    // Pause button
    btnPause?.addEventListener("click", () => {
      if (!this.isPlaying) return;
      this.isPlaying = false;
      btnPlay?.classList.remove("active");
      btnPause.classList.add("active");
      
      clearInterval(this.simInterval);
      this.writeToLiveLog("⏸️ Global simulation paused.");
    });

    // Step button (Manual tick)
    btnStep?.addEventListener("click", () => {
      this.writeToLiveLog("⏭️ Manual step triggered.");
      this.tickSimulation();
    });

    // Reset button
    btnReset?.addEventListener("click", () => {
      this.isPlaying = false;
      clearInterval(this.simInterval);
      
      btnPlay?.classList.remove("active");
      btnPause?.classList.remove("active");
      
      this.caseCounter = 1000;
      
      // Reset engines
      window.tokenEngine.resetMap();
      window.ledgerEngine.init();
      window.ledgerEngine.verifyAndRender();
      window.driftEngine.init();
      
      // Reset local stats
      window.tokenEngine.completedCount = 0;
      window.tokenEngine.totalDuration = 0;
      
      document.getElementById("metric-completed-cases").textContent = "0";
      document.getElementById("metric-avg-duration").textContent = "0.0s";
      document.getElementById("metric-avg-fitness").textContent = "1.00";
      document.getElementById("log-trace-cells").innerHTML = `<div class="empty-state">No alignment solved yet. Click 'Solve Alignment' to compute.</div>`;
      document.getElementById("model-trace-cells").innerHTML = "";
      document.getElementById("operation-cells").innerHTML = "";
      
      this.writeToLiveLog("🔄 Entire Process Simulation Engine has been reset.");
    });

    // Speed Slider
    speedSlider?.addEventListener("input", (e) => {
      this.simSpeed = parseInt(e.target.value);
      document.getElementById("speed-val").textContent = this.simSpeed + "ms";
      
      if (this.isPlaying) {
        // Restart interval with new speed
        clearInterval(this.simInterval);
        this.simInterval = setInterval(() => this.tickSimulation(), this.simSpeed);
      }
    });

    // Noise Slider
    noiseSlider?.addEventListener("input", (e) => {
      this.noiseRate = parseInt(e.target.value);
      document.getElementById("noise-val").textContent = this.noiseRate + "%";
    });
  }

  setupAlignmentPanelListeners() {
    const traceSelect = document.getElementById("trace-select");
    const customContainer = document.getElementById("custom-trace-container");
    const btnRunAlign = document.getElementById("btn-run-alignment");

    traceSelect?.addEventListener("change", (e) => {
      if (e.target.value === "custom") {
        customContainer.style.display = "flex";
      } else {
        customContainer.style.display = "none";
      }
    });

    btnRunAlign?.addEventListener("click", () => {
      let trace = [];
      const selectVal = traceSelect.value;
      
      if (selectVal === "custom") {
        const rawInput = document.getElementById("custom-trace-input").value;
        trace = rawInput.split(",").map(s => s.trim()).filter(s => s.length > 0);
        if (trace.length === 0) {
          alert("Please enter a valid comma-separated trace. E.g. Receive Order, Ship, End");
          return;
        }
      } else {
        trace = this.getPredefinedTrace(selectVal);
      }

      this.writeToLiveLog(`🔬 Running A* Alignment Solver for trace: [${trace.join(" → ")}]`);
      const result = window.alignmentSolver.solve(trace);
      window.tokenEngine.displayAlignment(trace, result);
    });
  }

  setupActionListeners() {
    const btnInject = document.getElementById("btn-inject-anomaly");
    const btnResetEngine = document.getElementById("btn-reset-engine");
    const driftToggle = document.getElementById("toggle-drift-injection");

    btnInject?.addEventListener("click", () => {
      if (driftToggle) {
        driftToggle.checked = true;
      }
      this.writeToLiveLog("⚠️ Anomaly Injected: Inducing process throughput delay drift!");
      // Simulate immediate drift injection
      for (let i = 0; i < 5; i++) {
        // Inject large values into drift chart
        const slowTime = window.driftEngine.generateNormalRandom(9.5, 2.5);
        window.driftEngine.processPoint(slowTime);
      }
      window.driftEngine.render();
    });

    btnResetEngine?.addEventListener("click", () => {
      document.getElementById("btn-global-reset")?.click();
    });
  }

  getPredefinedTrace(variantName) {
    switch (variantName) {
      case "conforming_standard":
        return ["Receive Order", "Check Inventory", "Approve", "Ship", "End"];
      case "conforming_skip_check":
        return ["Receive Order", "Approve", "Ship", "End"];
      case "deviant_skipped_approval":
        return ["Receive Order", "Check Inventory", "Ship", "End"];
      case "deviant_extra_ship":
        return ["Receive Order", "Check Inventory", "Approve", "Ship", "Ship", "End"];
      case "deviant_illegal_activity":
        return ["Receive Order", "Refund", "Ship", "End"];
      default:
        return ["Receive Order", "End"];
    }
  }

  tickSimulation() {
    // Only spawn a new case if the active queue is not overloaded
    if (window.tokenEngine.activeCases.size < 3) {
      this.caseCounter++;
      const caseId = `C-${this.caseCounter}`;
      
      const isDeviant = Math.random() * 100 < this.noiseRate;
      let deviantType = "";
      
      if (isDeviant) {
        const types = ["deviant_skipped_approval", "deviant_extra_ship", "deviant_illegal_activity"];
        deviantType = types[Math.floor(Math.random() * types.length)];
      }

      this.writeToLiveLog(`🚀 Starting Case ${caseId} (${isDeviant ? `Deviant: ${deviantType}` : 'Conforming'}).`);
      window.tokenEngine.startCase(caseId, isDeviant, deviantType);
      
      // Update UI active tokens count
      document.getElementById("active-tokens-count").textContent = window.tokenEngine.activeCases.size;
    }
  }
}

// Instantiate
window.appOrchestrator = new AppOrchestrator();
