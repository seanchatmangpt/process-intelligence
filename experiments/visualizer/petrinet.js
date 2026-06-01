/**
 * petrinet.js
 * Petri Net Structure and Token Game Simulator
 * 
 * Manages Petri Net configuration, active markings, and fires transitions,
 * supplying positions for rendering arcs and tokens in the SVG workspace.
 */

class PetriNetModel {
    constructor() {
        this.reset();
    }

    reset() {
        // Default model: Loan Approval Process
        this.places = {
            'p_start': { id: 'p_start', name: 'Start', x: 80, y: 200, isStart: true },
            'p_rec': { id: 'p_rec', name: 'Received', x: 240, y: 200 },
            'p_chk': { id: 'p_chk', name: 'Checked', x: 400, y: 200 },
            'p_dec': { id: 'p_dec', name: 'Verified', x: 560, y: 200 },
            'p_end': { id: 'p_end', name: 'End', x: 720, y: 200, isEnd: true }
        };

        this.transitions = {
            't_receive': { id: 't_receive', label: 'ReceiveRequest', x: 160, y: 200, w: 20, h: 40, preset: ['p_start'], postset: ['p_rec'] },
            't_check_credit': { id: 't_check_credit', label: 'CheckCredit', x: 320, y: 200, w: 20, h: 40, preset: ['p_rec'], postset: ['p_chk'] },
            't_verify_income': { id: 't_verify_income', label: 'VerifyIncome', x: 480, y: 200, w: 20, h: 40, preset: ['p_chk'], postset: ['p_dec'] },
            't_approve': { id: 't_approve', label: 'Approve', x: 640, y: 150, w: 20, h: 40, preset: ['p_dec'], postset: ['p_end'] },
            't_reject': { id: 't_reject', label: 'Reject', x: 640, y: 250, w: 20, h: 40, preset: ['p_dec'], postset: ['p_end'] },
            // Bypass/FastReject transition that bypasses credit checks
            't_bypass': { id: 't_bypass', label: 'FastReject', x: 400, y: 320, w: 20, h: 40, preset: ['p_rec'], postset: ['p_end'] }
        };

        this.arcs = [
            { id: 'a1', source: 'p_start', target: 't_receive', type: 'p2t' },
            { id: 'a2', source: 't_receive', target: 'p_rec', type: 't2p' },
            { id: 'a3', source: 'p_rec', target: 't_check_credit', type: 'p2t' },
            { id: 'a4', source: 't_check_credit', target: 'p_chk', type: 't2p' },
            { id: 'a5', source: 'p_chk', target: 't_verify_income', type: 'p2t' },
            { id: 'a6', source: 't_verify_income', target: 'p_dec', type: 't2p' },
            { id: 'a7', source: 'p_dec', target: 't_approve', type: 'p2t' },
            { id: 'a8', source: 't_approve', target: 'p_end', type: 't2p' },
            { id: 'a9', source: 'p_dec', target: 't_reject', type: 'p2t' },
            { id: 'a10', source: 't_reject', target: 'p_end', type: 't2p' },
            
            // Bypass arcs
            { id: 'a11', source: 'p_rec', target: 't_bypass', type: 'p2t', path: 'M 240,225 Q 240,320 390,320' },
            { id: 'a12', source: 't_bypass', target: 'p_end', type: 't2p', path: 'M 410,320 Q 720,320 720,225' }
        ];

        this.marking = { 'p_start': 1 };
        this.initialMarking = { 'p_start': 1 };
        this.finalMarking = { 'p_end': 1 };
    }

    /**
     * Gets a list of transition objects enabled in the current marking.
     */
    getEnabledTransitions() {
        const enabled = [];
        for (const tId in this.transitions) {
            const t = this.transitions[tId];
            let isEnabled = true;
            for (const pId of t.preset) {
                if ((this.marking[pId] || 0) < 1) {
                    isEnabled = false;
                    break;
                }
            }
            if (isEnabled) {
                enabled.push(t);
            }
        }
        return enabled;
    }

    /**
     * Fires a transition if enabled.
     * Updates marking state.
     * Returns true if successful.
     */
    fire(transitionId) {
        const t = this.transitions[transitionId];
        if (!t) return false;

        // Check if enabled
        for (const pId of t.preset) {
            if ((this.marking[pId] || 0) < 1) {
                return false;
            }
        }

        // Consume tokens
        for (const pId of t.preset) {
            this.marking[pId]--;
            if (this.marking[pId] <= 0) {
                delete this.marking[pId];
            }
        }

        // Produce tokens
        for (const pId of t.postset) {
            this.marking[pId] = (this.marking[pId] || 0) + 1;
        }

        return true;
    }

    /**
     * Set explicit marking
     */
    setMarking(newMarking) {
        this.marking = { ...newMarking };
    }

    /**
     * Checks if final marking has been reached
     */
    isFinished() {
        for (const pId in this.finalMarking) {
            if ((this.marking[pId] || 0) < this.finalMarking[pId]) {
                return false;
            }
        }
        return true;
    }

    /**
     * Gets coordinates of a node (place or transition)
     */
    getNodeCoords(id) {
        if (this.places[id]) {
            return { x: this.places[id].x, y: this.places[id].y };
        }
        if (this.transitions[id]) {
            return { x: this.transitions[id].x, y: this.transitions[id].y };
        }
        return null;
    }
}

// Export for usage in ESModules or global window object
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PetriNetModel;
} else {
    window.PetriNetModel = PetriNetModel;
}
