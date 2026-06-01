/**
 * ==========================================================================
 * EWMA Concept Drift Detector & Chart Renderer
 * ==========================================================================
 */

class EWMADriftDetector {
  constructor() {
    this.canvas = document.getElementById("drift-chart-canvas");
    this.ctx = this.canvas ? this.canvas.getContext("2d") : null;
    
    // Parameters
    this.lambda = 0.20; // Smoothing factor
    this.L = 3.00; // Control limit multiplier (sigma)
    this.mu0 = 5.0; // Target process mean (seconds)
    this.sigma0 = 1.2; // Target process standard deviation (seconds)
    
    // Data structures
    this.points = []; // Array of { actual, ewma, ucl, lcl, index }
    this.maxPoints = 40; // Max points visible on chart
    
    this.init();
  }

  init() {
    this.points = [];
    // Pre-populate with some clean in-control history
    for (let i = 0; i < 15; i++) {
      const val = this.generateNormalRandom(this.mu0, this.sigma0);
      this.processPoint(val);
    }
    this.resizeCanvas();
    this.render();
    
    // Bind controls
    const lambdaSlider = document.getElementById("drift-lambda");
    const sigmaSlider = document.getElementById("drift-sigma");
    
    if (lambdaSlider) {
      lambdaSlider.addEventListener("input", (e) => {
        this.lambda = parseFloat(e.target.value);
        document.getElementById("val-lambda").textContent = this.lambda.toFixed(2);
        this.recalculateChain();
        this.render();
      });
    }

    if (sigmaSlider) {
      sigmaSlider.addEventListener("input", (e) => {
        this.L = parseFloat(e.target.value);
        document.getElementById("val-sigma").textContent = this.L.toFixed(2);
        this.recalculateChain();
        this.render();
      });
    }

    window.addEventListener("resize", () => {
      this.resizeCanvas();
      this.render();
    });
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

  // Box-Muller transform for normal distribution simulation
  generateNormalRandom(mean, stdDev) {
    let u = 0, v = 0;
    while(u === 0) u = Math.random(); 
    while(v === 0) v = Math.random();
    let num = Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
    return num * stdDev + mean;
  }

  // Calculate EWMA step
  processPoint(actualValue) {
    const i = this.points.length + 1;
    const prevEwma = i === 1 ? this.mu0 : this.points[this.points.length - 1].ewma;
    
    // EWMA formula
    const ewma = this.lambda * actualValue + (1 - this.lambda) * prevEwma;
    
    // EWMA standard deviation formula
    const term = 1 - Math.pow(1 - this.lambda, 2 * i);
    const ewmaSigma = this.sigma0 * Math.sqrt((this.lambda / (2 - this.lambda)) * term);
    
    // Control limits
    const ucl = this.mu0 + this.L * ewmaSigma;
    const lcl = Math.max(0, this.mu0 - this.L * ewmaSigma); // Limit LCL to >= 0

    const newPoint = {
      actual: actualValue,
      ewma: ewma,
      ucl: ucl,
      lcl: lcl,
      isOut: ewma > ucl || ewma < lcl
    };

    this.points.push(newPoint);
    
    // Roll buffer
    if (this.points.length > this.maxPoints) {
      this.points.shift();
    }

    this.checkDriftAlert();
  }

  recalculateChain() {
    // Recalculates EWMA and bounds for all current points in the buffer
    const tempActuals = this.points.map(p => p.actual);
    this.points = [];
    tempActuals.forEach(actual => {
      this.processPoint(actual);
    });
  }

  checkDriftAlert() {
    if (this.points.length === 0) return;
    const latest = this.points[this.points.length - 1];
    
    const alertBanner = document.getElementById("drift-alert");
    const statusText = document.getElementById("drift-status-indicator");
    const globalDriftMetric = document.getElementById("metric-avg-duration"); // Average throughput time display
    
    if (alertBanner && statusText) {
      if (latest.isOut) {
        alertBanner.style.display = "flex";
        statusText.innerHTML = `Status: <span class="badge badge-danger">DRIFT DETECTED</span>`;
      } else {
        alertBanner.style.display = "none";
        statusText.innerHTML = `Status: <span class="badge badge-success">In Control</span>`;
      }
    }
  }

  render() {
    if (!this.ctx || !this.canvas) return;
    
    const width = this.canvas.width / window.devicePixelRatio;
    const height = this.canvas.height / window.devicePixelRatio;
    const ctx = this.ctx;
    
    ctx.clearRect(0, 0, width, height);

    if (this.points.length === 0) return;

    // Margins
    const margin = { top: 20, right: 20, bottom: 30, left: 40 };
    const plotWidth = width - margin.left - margin.right;
    const plotHeight = height - margin.top - margin.bottom;

    // Find min and max for Y scaling
    let minY = Math.min(...this.points.map(p => Math.min(p.actual, p.lcl)));
    let maxY = Math.max(...this.points.map(p => Math.max(p.actual, p.ucl)));
    
    // Add 10% padding
    const yPad = (maxY - minY) * 0.1 || 1.0;
    minY = Math.max(0, minY - yPad);
    maxY = maxY + yPad;

    const getX = (idx) => margin.left + (idx / (this.maxPoints - 1)) * plotWidth;
    const getY = (val) => margin.top + plotHeight - ((val - minY) / (maxY - minY)) * plotHeight;

    // Draw Grid Lines (Y)
    ctx.strokeStyle = "rgba(255, 255, 255, 0.05)";
    ctx.lineWidth = 1;
    const gridLines = 5;
    for (let k = 0; k <= gridLines; k++) {
      const val = minY + (k / gridLines) * (maxY - minY);
      const y = getY(val);
      
      ctx.beginPath();
      ctx.moveTo(margin.left, y);
      ctx.lineTo(width - margin.right, y);
      ctx.stroke();

      // Label
      ctx.fillStyle = "rgba(255, 255, 255, 0.4)";
      ctx.font = "9px 'Fira Code'";
      ctx.textAlign = "right";
      ctx.fillText(val.toFixed(1) + "s", margin.left - 8, y + 3);
    }

    // Draw Target Mean Line (mu0)
    ctx.strokeStyle = "rgba(255, 255, 255, 0.15)";
    ctx.setLineDash([5, 5]);
    ctx.beginPath();
    ctx.moveTo(margin.left, getY(this.mu0));
    ctx.lineTo(width - margin.right, getY(this.mu0));
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = "rgba(255, 255, 255, 0.3)";
    ctx.fillText("μ₀ Target", width - margin.right, getY(this.mu0) - 4);

    // Draw Control Limit Band (LCL to UCL)
    ctx.fillStyle = "rgba(59, 130, 246, 0.03)";
    ctx.beginPath();
    this.points.forEach((p, idx) => {
      const x = getX(idx);
      const y = getY(p.ucl);
      if (idx === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    for (let idx = this.points.length - 1; idx >= 0; idx--) {
      const x = getX(idx);
      const y = getY(this.points[idx].lcl);
      ctx.lineTo(x, y);
    }
    ctx.closePath();
    ctx.fill();

    // Draw UCL and LCL boundaries
    ctx.strokeStyle = "rgba(239, 68, 68, 0.35)";
    ctx.lineWidth = 1;
    ctx.setLineDash([3, 3]);
    
    // UCL
    ctx.beginPath();
    this.points.forEach((p, idx) => {
      ctx.lineTo(getX(idx), getY(p.ucl));
    });
    ctx.stroke();
    
    // LCL
    ctx.beginPath();
    this.points.forEach((p, idx) => {
      ctx.lineTo(getX(idx), getY(p.lcl));
    });
    ctx.stroke();
    ctx.setLineDash([]);

    // Draw Actual Value Data Points
    ctx.strokeStyle = "rgba(255, 255, 255, 0.12)";
    ctx.lineWidth = 1;
    ctx.beginPath();
    this.points.forEach((p, idx) => {
      ctx.lineTo(getX(idx), getY(p.actual));
    });
    ctx.stroke();

    this.points.forEach((p, idx) => {
      ctx.fillStyle = "rgba(255, 255, 255, 0.4)";
      ctx.beginPath();
      ctx.arc(getX(idx), getY(p.actual), 2, 0, 2 * Math.PI);
      ctx.fill();
    });

    // Draw EWMA Trend Line
    ctx.strokeStyle = "rgba(16, 185, 129, 0.85)";
    ctx.lineWidth = 2.5;
    ctx.beginPath();
    this.points.forEach((p, idx) => {
      ctx.lineTo(getX(idx), getY(p.ewma));
    });
    ctx.stroke();

    // Draw Glowing effect on EWMA Trend
    ctx.strokeStyle = "rgba(16, 185, 129, 0.2)";
    ctx.lineWidth = 8;
    ctx.beginPath();
    this.points.forEach((p, idx) => {
      ctx.lineTo(getX(idx), getY(p.ewma));
    });
    ctx.stroke();

    // Highlight Out of Control points (Drift)
    this.points.forEach((p, idx) => {
      if (p.isOut) {
        ctx.fillStyle = "#ef4444";
        ctx.shadowColor = "#ef4444";
        ctx.shadowBlur = 8;
        ctx.beginPath();
        ctx.arc(getX(idx), getY(p.ewma), 5, 0, 2 * Math.PI);
        ctx.fill();
        ctx.shadowBlur = 0; // Reset shadow
      }
    });

    // Draw bottom X labels (Cases)
    ctx.fillStyle = "rgba(255, 255, 255, 0.4)";
    ctx.font = "9px 'Fira Code'";
    ctx.textAlign = "center";
    const step = Math.max(1, Math.floor(this.points.length / 5));
    for (let idx = 0; idx < this.points.length; idx += step) {
      ctx.fillText(`#${idx + 1}`, getX(idx), height - margin.bottom + 12);
    }
  }
}

// Global initialization
window.driftEngine = new EWMADriftDetector();
