/**
 * drift.js
 * EWMA (Exponentially Weighted Moving Average) Drift Detector
 * 
 * Used for detecting concept drift in streaming process logs (e.g., delays or compliance drops).
 */

class EWMADriftDetector {
    /**
     * @param {Object} options
     *   - lambda: Weight of new observations (0 < lambda <= 1, default: 0.15)
     *   - L: Control limit multiplier (default: 2.7)
     *   - baselineMean: Expected historical mean (default: 0)
     *   - baselineStDev: Expected historical standard deviation (default: 1)
     */
    constructor(options = {}) {
        this.lambda = options.lambda !== undefined ? options.lambda : 0.15;
        this.L = options.L !== undefined ? options.L : 2.7;
        this.baselineMean = options.baselineMean !== undefined ? options.baselineMean : 0.0;
        this.baselineStDev = options.baselineStDev !== undefined ? options.baselineStDev : 1.0;
        
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
     * Updates the EWMA filter with a new observation.
     * @param {number} value - The observed metric (e.g. cycle time, compliance score).
     * @returns {Object} State containing current EWMA value, limits, and drift status.
     */
    update(value) {
        this.t++;
        if (this.t === 1) {
            this.currentValue = this.lambda * value + (1 - this.lambda) * this.baselineMean;
        } else {
            this.currentValue = this.lambda * value + (1 - this.lambda) * this.currentValue;
        }

        // Calculate standard deviation of EWMA statistic
        // sigma_{S_t} = sigma_0 * sqrt( (lambda / (2 - lambda)) * (1 - (1 - lambda)^(2*t)) )
        const term = (this.lambda / (2.0 - this.lambda)) * (1.0 - Math.pow(1.0 - this.lambda, 2 * this.t));
        const ewmaStDev = this.baselineStDev * Math.sqrt(term);

        const ucl = this.baselineMean + this.L * ewmaStDev;
        const lcl = this.baselineMean - this.L * ewmaStDev;

        const isDrift = this.currentValue > ucl || this.currentValue < lcl;

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
     * Set a new baseline based on a training dataset.
     */
    calibrate(data) {
        if (data.length === 0) return;
        const sum = data.reduce((a, b) => a + b, 0);
        this.baselineMean = sum / data.length;

        const variance = data.reduce((a, b) => a + Math.pow(b - this.baselineMean, 2), 0) / data.length;
        this.baselineStDev = Math.sqrt(variance) || 1.0;
        this.reset();
    }
}

// Export for usage in ESModules or global window object
if (typeof module !== 'undefined' && module.exports) {
    module.exports = EWMADriftDetector;
} else {
    window.EWMADriftDetector = EWMADriftDetector;
}
