/**
 * drift.js
 * Sliding-Window Concept Drift Detector & EWMA Chart Renderer
 * 
 * Computes Jaccard distance between directly-follows graph (DFG) profiles of 
 * a reference baseline window and a detection window, smoothing results via EWMA.
 * 
 * References:
 * - EWMA Drift Formulation: file:///Users/sac/process-intelligence/experiments/visualizer/index.html
 * - Petri Net structure: file:///Users/sac/process-intelligence/experiments/visualizer/petrinet.js
 * - Ledger blockchain: file:///Users/sac/process-intelligence/experiments/visualizer/ledger.js
 */

class EWMADriftDetector {
    /**
     * @param {Object} options
     *   - lambda: Weight of new observations (0 < lambda <= 1, default: 0.20)
     *   - L: Control limit multiplier (default: 3.0)
     *   - baselineMean: Expected historical mean Jaccard distance (default: 0.0)
     *   - baselineStDev: Expected historical standard deviation (default: 0.05)
     */
    constructor(options = {}) {
        this.lambda = options.lambda !== undefined ? options.lambda : 0.20;
        this.L = options.L !== undefined ? options.L : 3.0;
        this.baselineMean = options.baselineMean !== undefined ? options.baselineMean : 0.0;
        this.baselineStDev = options.baselineStDev !== undefined ? options.baselineStDev : 0.05;
        this.reset();
    }

    /**
     * Resets the detector state.
     */
    reset() {
        this.t = 0;
        this.currentValue = this.baselineMean;
        this.history = [];
    }

    /**
     * Updates the EWMA filter with a new Jaccard distance value.
     * @param {number} value - The Jaccard distance observation.
     * @returns {Object} State containing EWMA, limits, and drift status.
     */
    update(value) {
        this.t++;
        
        // EWMA update formula
        if (this.t === 1) {
            this.currentValue = this.lambda * value + (1 - this.lambda) * this.baselineMean;
        } else {
            this.currentValue = this.lambda * value + (1 - this.lambda) * this.currentValue;
        }

        // Standard deviation of EWMA statistic
        // sigma_{S_t} = sigma_0 * sqrt( (lambda / (2 - lambda)) * (1 - (1 - lambda)^(2*t)) )
        const term = (this.lambda / (2.0 - this.lambda)) * (1.0 - Math.pow(1.0 - this.lambda, 2 * this.t));
        const ewmaStDev = this.baselineStDev * Math.sqrt(term);

        // Control limits
        const ucl = this.baselineMean + this.L * ewmaStDev;
        const lcl = Math.max(0, this.baselineMean - this.L * ewmaStDev);
        const isDrift = this.currentValue > ucl;

        const point = {
            t: this.t,
            input: value,
            ewma: this.currentValue,
            ucl: ucl,
            lcl: lcl,
            isDrift: isDrift
        };

        this.history.push(point);
        return point;
    }

    /**
     * Set a new baseline based on a calibration dataset of Jaccard distances.
     */
    calibrate(data) {
        if (!data || data.length === 0) return;
        const sum = data.reduce((a, b) => a + b, 0);
        this.baselineMean = sum / data.length;

        const variance = data.reduce((a, b) => a + Math.pow(b - this.baselineMean, 2), 0) / data.length;
        // Establish a lower bound for standard deviation to prevent divisions by zero or overly sensitive alarms
        this.baselineStDev = Math.max(0.01, Math.sqrt(variance));
        this.reset();
    }
}

class SlidingWindowDriftMonitor {
    /**
     * @param {Object} options
     *   - windowSize: Capacity of reference and detection windows (default: 30)
     *   - lambda: EWMA smoothing factor (default: 0.20)
     *   - L: EWMA control limit multiplier (default: 3.0)
     */
    constructor(options = {}) {
        this.windowSize = options.windowSize || 30;
        this.referenceWindow = [];
        this.detectionWindow = [];
        this.calibrationData = [];
        this.isCalibrated = false;
        this.calibrationLimit = 15; // Number of stable Jaccard points needed to calibrate

        this.detector = new EWMADriftDetector({
            lambda: options.lambda !== undefined ? options.lambda : 0.20,
            L: options.L !== undefined ? options.L : 3.0,
            baselineMean: 0.0,
            baselineStDev: 0.02 // start with tight expectation
        });

        this.history = [];
        this.casesProcessed = 0;
    }

    /**
     * Resets the entire sliding window monitor state.
     */
    reset() {
        this.referenceWindow = [];
        this.detectionWindow = [];
        this.calibrationData = [];
        this.isCalibrated = false;
        this.detector.reset();
        this.history = [];
        this.casesProcessed = 0;
    }

    /**
     * Extracts directly-follows relation (DFG) edges from a single case trace.
     * @param {Array<string>} trace - Array of activity names, e.g. ['A', 'B', 'C']
     * @returns {Set<string>} Set of activity pair strings, e.g. Set {'A->B', 'B->C'}
     */
    extractDFGEdges(trace) {
        const edges = new Set();
        if (!trace || trace.length < 2) return edges;
        for (let i = 0; i < trace.length - 1; i++) {
            edges.add(`${trace[i]}->${trace[i + 1]}`);
        }
        return edges;
    }

    /**
     * Computes the aggregated DFG edge profile for a window of traces.
     * @param {Array<Array<string>>} window - Array of trace activity arrays
     * @returns {Set<string>} Set of all DFG edges observed in the window
     */
    computeWindowProfile(window) {
        const aggregatedProfile = new Set();
        for (const trace of window) {
            const traceEdges = this.extractDFGEdges(trace);
            for (const edge of traceEdges) {
                aggregatedProfile.add(edge);
            }
        }
        return aggregatedProfile;
    }

    /**
     * Calculates the Jaccard distance between two profile sets.
     * @param {Set<string>} profileA
     * @param {Set<string>} profileB
     * @returns {number} Jaccard distance (0.0 to 1.0)
     */
    calculateJaccardDistance(profileA, profileB) {
        if (profileA.size === 0 && profileB.size === 0) return 0.0;
        if (profileA.size === 0 || profileB.size === 0) return 1.0;

        let intersectionSize = 0;
        for (const item of profileA) {
            if (profileB.has(item)) {
                intersectionSize++;
            }
        }

        const unionSize = profileA.size + profileB.size - intersectionSize;
        if (unionSize === 0) return 0.0;

        const similarity = intersectionSize / unionSize;
        return 1.0 - similarity;
    }

    /**
     * Process a newly completed case trace.
     * @param {Array<string>} trace - Array of activities for the completed case
     * @returns {Object|null} Updates state object or null if warming up
     */
    ingestCase(trace) {
        this.casesProcessed++;

        // Add to reference window if not yet filled
        if (this.referenceWindow.length < this.windowSize) {
            this.referenceWindow.push(trace);
        }

        // Always push to sliding detection window
        this.detectionWindow.push(trace);
        if (this.detectionWindow.length > this.windowSize) {
            this.detectionWindow.shift();
        }

        // Wait until we have enough data in both windows to calculate distance
        if (this.referenceWindow.length < 5 || this.detectionWindow.length < 5) {
            return null;
        }

        // Extract DFG profiles
        const refProfile = this.computeWindowProfile(this.referenceWindow);
        const detProfile = this.computeWindowProfile(this.detectionWindow);

        // Compute Jaccard distance
        const jaccardDist = this.calculateJaccardDistance(refProfile, detProfile);

        let updateResult;

        if (!this.isCalibrated) {
            // Collect calibration data during stable startup
            this.calibrationData.push(jaccardDist);
            
            // Set initial baseline statistics when threshold is reached
            if (this.calibrationData.length >= this.calibrationLimit) {
                this.detector.calibrate(this.calibrationData);
                this.isCalibrated = true;
            }
            
            // Run update with baseline defaults for now
            updateResult = this.detector.update(jaccardDist);
        } else {
            // Calibrated normal operations
            updateResult = this.detector.update(jaccardDist);
        }

        const point = {
            caseIndex: this.casesProcessed,
            jaccardDistance: jaccardDist,
            ewma: updateResult.ewma,
            ucl: updateResult.ucl,
            lcl: updateResult.lcl,
            isDrift: updateResult.isDrift,
            isCalibrated: this.isCalibrated
        };

        this.history.push(point);
        if (this.history.length > 100) {
            this.history.shift(); // Limit plotting memory
        }

        return point;
    }

    /**
     * Update EWMA tuning parameters dynamically from UI controls.
     */
    updateParameters(lambda, sigmaLimit) {
        this.detector.lambda = lambda;
        this.detector.L = sigmaLimit;
        
        // Re-calculate drift history limits if calibrated
        if (this.isCalibrated && this.history.length > 0) {
            const tempDetector = new EWMADriftDetector({
                lambda: lambda,
                L: sigmaLimit,
                baselineMean: this.detector.baselineMean,
                baselineStDev: this.detector.baselineStDev
            });

            const rawDistances = this.history.map(pt => pt.jaccardDistance);
            this.history.forEach((pt, idx) => {
                const res = tempDetector.update(pt.jaccardDistance);
                pt.ewma = res.ewma;
                pt.ucl = res.ucl;
                pt.lcl = res.lcl;
                pt.isDrift = res.isDrift;
            });
            
            this.detector.currentValue = tempDetector.currentValue;
            this.detector.t = tempDetector.t;
        }
    }
}

class DriftChartRenderer {
    /**
     * @param {string} canvasId - DOM ID of target HTML5 canvas element
     */
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        if (!this.canvas) {
            console.error(`Canvas element with ID '${canvasId}' not found.`);
            return;
        }
        this.ctx = this.canvas.getContext('2d');
        this.width = 0;
        this.height = 0;
        
        this.resize();
        // Bind window resize event handler
        this.resizeHandler = this.resize.bind(this);
        window.addEventListener('resize', this.resizeHandler);
    }

    /**
     * Handles canvas sizing matching actual container dimensions.
     */
    resize() {
        if (!this.canvas) return;
        const rect = this.canvas.parentNode.getBoundingClientRect();
        const dpr = window.devicePixelRatio || 1;
        
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;
        this.ctx.scale(dpr, dpr);
        
        this.width = rect.width;
        this.height = rect.height;
    }

    /**
     * Detaches event listeners before teardown.
     */
    destroy() {
        window.removeEventListener('resize', this.resizeHandler);
    }

    /**
     * Draws the control chart representing concept drift status.
     * @param {Array<Object>} history - Log points from SlidingWindowDriftMonitor
     */
    draw(history) {
        if (!this.ctx || !this.canvas) return;

        // Clear canvas
        this.ctx.clearRect(0, 0, this.width, this.height);

        // Fill background
        this.ctx.fillStyle = 'rgba(10, 10, 12, 0.6)';
        this.ctx.fillRect(0, 0, this.width, this.height);

        const padding = { top: 25, right: 30, bottom: 30, left: 45 };
        const chartWidth = this.width - padding.left - padding.right;
        const chartHeight = this.height - padding.top - padding.bottom;

        // Draw empty state message if insufficient data
        if (!history || history.length < 2) {
            this.ctx.fillStyle = '#94a3b8';
            this.ctx.font = '500 13px "Outfit", sans-serif';
            this.ctx.textAlign = 'center';
            this.ctx.textBaseline = 'middle';
            this.ctx.fillText(
                'Awaiting completed cases to calibrate baseline (min. 15 cases)...',
                this.width / 2,
                this.height / 2
            );
            return;
        }

        // Calculate Y scale boundaries
        let maxVal = 0.4; // Default baseline scale
        history.forEach(pt => {
            if (pt.jaccardDistance > maxVal) maxVal = pt.jaccardDistance;
            if (pt.ewma > maxVal) maxVal = pt.ewma;
            if (pt.ucl > maxVal) maxVal = pt.ucl;
        });
        // Round up to nearest 0.1 for clean grids
        maxVal = Math.min(1.0, Math.ceil(maxVal * 10) / 10 + 0.1);

        const minVal = 0;

        // Helper coordinates functions
        const getX = (index) => padding.left + (index / (history.length - 1)) * chartWidth;
        const getY = (val) => padding.top + chartHeight - ((val - minVal) / (maxVal - minVal)) * chartHeight;

        // 1. Draw gridlines and axes labels
        this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
        this.ctx.lineWidth = 1;
        this.ctx.fillStyle = '#64748b';
        this.ctx.font = '500 9px "Fira Code", monospace';
        this.ctx.textAlign = 'right';
        this.ctx.textBaseline = 'middle';

        const yGridLines = 5;
        for (let i = 0; i <= yGridLines; i++) {
            const val = minVal + (i / yGridLines) * (maxVal - minVal);
            const y = getY(val);
            
            // Grid line
            this.ctx.beginPath();
            this.ctx.moveTo(padding.left, y);
            this.ctx.lineTo(this.width - padding.right, y);
            this.ctx.stroke();

            // Label
            this.ctx.fillText(val.toFixed(2), padding.left - 10, y);
        }

        // X-axis ticks (Case Indexes)
        this.ctx.textAlign = 'center';
        this.ctx.textBaseline = 'top';
        const xTickStep = Math.max(1, Math.floor(history.length / 5));
        for (let i = 0; i < history.length; i += xTickStep) {
            const x = getX(i);
            const pt = history[i];
            
            this.ctx.beginPath();
            this.ctx.moveTo(x, padding.top + chartHeight);
            this.ctx.lineTo(x, padding.top + chartHeight + 4);
            this.ctx.stroke();

            this.ctx.fillText(`#${pt.caseIndex}`, x, padding.top + chartHeight + 8);
        }

        // 2. Draw UCL (Upper Control Limit) Line
        this.ctx.strokeStyle = 'rgba(239, 68, 68, 0.8)';
        this.ctx.lineWidth = 1.5;
        this.ctx.setLineDash([5, 5]);
        this.ctx.beginPath();
        this.ctx.moveTo(getX(0), getY(history[0].ucl));
        for (let i = 1; i < history.length; i++) {
            this.ctx.lineTo(getX(i), getY(history[i].ucl));
        }
        this.ctx.stroke();
        this.ctx.setLineDash([]); // Reset dash

        // Draw UCL label near the end
        this.ctx.fillStyle = '#ef4444';
        this.ctx.font = '600 8px "Outfit", sans-serif';
        this.ctx.textAlign = 'right';
        this.ctx.fillText('UCL', this.width - padding.right, getY(history[history.length - 1].ucl) - 6);

        // 3. Draw Jaccard Raw Distance Area/Line (Background Series)
        this.ctx.strokeStyle = 'rgba(59, 130, 246, 0.3)';
        this.ctx.lineWidth = 1;
        this.ctx.beginPath();
        this.ctx.moveTo(getX(0), getY(history[0].jaccardDistance));
        for (let i = 1; i < history.length; i++) {
            this.ctx.lineTo(getX(i), getY(history[i].jaccardDistance));
        }
        this.ctx.stroke();

        // 4. Draw EWMA Smoothed Line
        const isCurrentlyDrifting = history[history.length - 1].isDrift;
        const ewmaColor = isCurrentlyDrifting ? '#ef4444' : '#10b981'; // Red if drifting, emerald if stable
        const ewmaFill = isCurrentlyDrifting ? 'rgba(239, 68, 68, 0.12)' : 'rgba(16, 185, 129, 0.1)';

        // Draw Area Fill under EWMA
        this.ctx.fillStyle = ewmaFill;
        this.ctx.beginPath();
        this.ctx.moveTo(getX(0), padding.top + chartHeight);
        for (let i = 0; i < history.length; i++) {
            this.ctx.lineTo(getX(i), getY(history[i].ewma));
        }
        this.ctx.lineTo(getX(history.length - 1), padding.top + chartHeight);
        this.ctx.closePath();
        this.ctx.fill();

        // Draw EWMA Line with glowing effect
        this.ctx.shadowColor = ewmaColor;
        this.ctx.shadowBlur = 4;
        this.ctx.strokeStyle = ewmaColor;
        this.ctx.lineWidth = 3;
        this.ctx.beginPath();
        this.ctx.moveTo(getX(0), getY(history[0].ewma));
        for (let i = 1; i < history.length; i++) {
            this.ctx.lineTo(getX(i), getY(history[i].ewma));
        }
        this.ctx.stroke();
        
        // Reset Shadow/Blur
        this.ctx.shadowBlur = 0;

        // 5. Draw Alarm Markers (Red dots where isDrift is true)
        history.forEach((pt, i) => {
            if (pt.isDrift) {
                this.ctx.fillStyle = '#ef4444';
                this.ctx.beginPath();
                this.ctx.arc(getX(i), getY(pt.ewma), 4, 0, 2 * Math.PI);
                this.ctx.fill();

                // Outer stroke circle
                this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.8)';
                this.ctx.lineWidth = 1;
                this.ctx.beginPath();
                this.ctx.arc(getX(i), getY(pt.ewma), 6, 0, 2 * Math.PI);
                this.ctx.stroke();
            } else {
                // Regular small dot for current points
                this.ctx.fillStyle = i === history.length - 1 ? '#10b981' : 'rgba(255, 255, 255, 0.2)';
                this.ctx.beginPath();
                this.ctx.arc(getX(i), getY(pt.ewma), i === history.length - 1 ? 3 : 1.5, 0, 2 * Math.PI);
                this.ctx.fill();
            }
        });

        // 6. Draw Chart Legend/Labels overlay
        this.ctx.fillStyle = '#f8fafc';
        this.ctx.font = '600 10px "Outfit", sans-serif';
        this.ctx.textAlign = 'left';
        this.ctx.textBaseline = 'top';
        
        const textOffset = 8;
        this.ctx.fillStyle = 'rgba(59, 130, 246, 0.8)';
        this.ctx.fillRect(padding.left + textOffset, padding.top + 4, 8, 8);
        this.ctx.fillStyle = '#94a3b8';
        this.ctx.fillText('Jaccard Distance (Raw)', padding.left + textOffset + 14, padding.top + 3);

        const ewmaOffset = padding.left + textOffset + 130;
        this.ctx.fillStyle = ewmaColor;
        this.ctx.fillRect(ewmaOffset, padding.top + 4, 8, 8);
        this.ctx.fillStyle = '#94a3b8';
        this.ctx.fillText('EWMA (Smoothed)', ewmaOffset + 14, padding.top + 3);
    }
}

// Export for usage in ESModules or global window object
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        EWMADriftDetector,
        SlidingWindowDriftMonitor,
        DriftChartRenderer
    };
} else {
    window.EWMADriftDetector = EWMADriftDetector;
    window.SlidingWindowDriftMonitor = SlidingWindowDriftMonitor;
    window.DriftChartRenderer = DriftChartRenderer;
}
