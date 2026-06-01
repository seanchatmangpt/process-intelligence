/**
 * alignment.js
 * A* Alignment Solver for Process Mining
 * 
 * Computes the optimal alignment between an observed event trace and a Petri Net model.
 * Supports synchronous moves (cost 0), log-only moves (cost 1), and model-only moves (cost 1).
 */

class AlignmentSolver {
    /**
     * Solve the alignment problem for a trace and a Petri Net.
     * @param {Array<string>} trace - Sequence of activity names in the observed log.
     * @param {Object} petriNet - The Petri net definition:
     *   - places: Array of place IDs.
     *   - transitions: Array of transition objects: { id, label, preset: [], postset: [] }
     *   - initialMarking: Object mapping place ID to token count.
     *   - finalMarking: Object mapping place ID to token count.
     */
    static solve(trace, petriNet) {
        const startMarking = petriNet.initialMarking;
        const endMarking = petriNet.finalMarking;

        // Priority Queue implementation for A*
        const openList = [];
        const closedSet = new Set();

        // State representation:
        // { marking, logIndex, g, h, f, moves }
        const startState = {
            marking: { ...startMarking },
            logIndex: 0,
            g: 0,
            h: trace.length, // Heuristic: remaining trace elements
            f: trace.length,
            moves: []
        };

        openList.push(startState);

        const stateKey = (state) => {
            const markingStr = Object.keys(state.marking)
                .sort()
                .map(p => `${p}:${state.marking[p]}`)
                .join(',');
            return `${markingStr}|idx:${state.logIndex}`;
        };

        const isFinalMarking = (marking) => {
            for (const p of Object.keys(endMarking)) {
                if ((marking[p] || 0) < endMarking[p]) {
                    return false;
                }
            }
            return true;
        };

        let iterations = 0;
        const maxIterations = 5000; // Fail-safe limit

        while (openList.length > 0 && iterations < maxIterations) {
            iterations++;
            // Sort open list by f cost, then by g (favor deeper paths if f is equal)
            openList.sort((a, b) => a.f - b.f || b.logIndex - a.logIndex);
            const current = openList.shift();

            // Goal check
            if (current.logIndex === trace.length && isFinalMarking(current.marking)) {
                return {
                    cost: current.g,
                    moves: current.moves,
                    iterations,
                    success: true
                };
            }

            const key = stateKey(current);
            if (closedSet.has(key)) {
                continue;
            }
            closedSet.add(key);

            const enabledTransitions = this.getEnabledTransitions(current.marking, petriNet.transitions);

            // 1. Sync moves
            if (current.logIndex < trace.length) {
                const logAct = trace[current.logIndex];
                for (const t of enabledTransitions) {
                    if (t.label === logAct) {
                        const newMarking = this.fireTransition(current.marking, t);
                        const nextState = {
                            marking: newMarking,
                            logIndex: current.logIndex + 1,
                            g: current.g, // Sync move cost is 0
                            h: trace.length - (current.logIndex + 1),
                            moves: [...current.moves, {
                                type: 'sync',
                                logAct: logAct,
                                modelAct: t.label,
                                transitionId: t.id,
                                cost: 0
                            }]
                        };
                        nextState.f = nextState.g + nextState.h;
                        openList.push(nextState);
                    }
                }
            }

            // 2. Model-only moves
            for (const t of enabledTransitions) {
                const isSilent = t.label === '' || t.label.startsWith('tau');
                const moveCost = isSilent ? 0 : 1;
                const newMarking = this.fireTransition(current.marking, t);
                const nextState = {
                    marking: newMarking,
                    logIndex: current.logIndex,
                    g: current.g + moveCost,
                    h: trace.length - current.logIndex,
                    moves: [...current.moves, {
                        type: 'model',
                        logAct: '≫',
                        modelAct: t.label || 'τ',
                        transitionId: t.id,
                        cost: moveCost
                    }]
                };
                nextState.f = nextState.g + nextState.h;
                openList.push(nextState);
            }

            // 3. Log-only moves
            if (current.logIndex < trace.length) {
                const logAct = trace[current.logIndex];
                const nextState = {
                    marking: { ...current.marking },
                    logIndex: current.logIndex + 1,
                    g: current.g + 1, // Log move cost is 1
                    h: trace.length - (current.logIndex + 1),
                    moves: [...current.moves, {
                        type: 'log',
                        logAct: logAct,
                        modelAct: '≫',
                        transitionId: null,
                        cost: 1
                    }]
                };
                nextState.f = nextState.g + nextState.h;
                openList.push(nextState);
            }
        }

        return {
            cost: Infinity,
            moves: [],
            iterations,
            success: false,
            error: 'Search space exhausted or reached max iterations without finding path'
        };
    }

    /**
     * Determines which transitions are enabled in the current marking.
     */
    static getEnabledTransitions(marking, transitions) {
        return transitions.filter(t => {
            for (const p of t.preset) {
                if ((marking[p] || 0) < 1) {
                    return false;
                }
            }
            return true;
        });
    }

    /**
     * Fires a transition and returns the new marking.
     */
    static fireTransition(marking, transition) {
        const nextMarking = { ...marking };
        for (const p of transition.preset) {
            nextMarking[p] = (nextMarking[p] || 0) - 1;
            if (nextMarking[p] <= 0) {
                delete nextMarking[p];
            }
        }
        for (const p of transition.postset) {
            nextMarking[p] = (nextMarking[p] || 0) + 1;
        }
        return nextMarking;
    }
}

// Export for usage in ESModules or global window object
if (typeof module !== 'undefined' && module.exports) {
    module.exports = AlignmentSolver;
} else {
    window.AlignmentSolver = AlignmentSolver;
}
