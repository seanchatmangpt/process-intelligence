/**
 * alignment.js
 * A* Process Alignment Solver
 * 
 * Computes optimal alignments between event logs and Petri Nets.
 * Matches log sequences to model executions, classifying moves as:
 * - Synchronous (cost = 0)
 * - Move on Log (cost = 1)
 * - Move on Model (cost = 1)
 * 
 * References:
 * - PM4Py Alignment Formulation: file:///Users/sac/process-intelligence/sources/papers/workflow-ledger.md
 * - Conformance Doctrine: file:///Users/sac/process-intelligence/experiments/petri_conformance_sample.md
 */

const ProcessAlignment = (() => {

    /**
     * Finds the shortest transition distance from a place to the sink place ('p_end')
     * using BFS on the Petri Net structure.
     */
    function getPlaceDistance(place, petriNet) {
        const target = petriNet.finalPlace || "p_end";
        if (place === target) return 0;
        
        const queue = [{ place, dist: 0 }];
        const visited = new Set([place]);
        
        while (queue.length > 0) {
            const curr = queue.shift();
            if (curr.place === target) return curr.dist;
            
            // Find transitions that have curr.place as input
            for (const t of petriNet.transitions) {
                if (t.inputs.includes(curr.place)) {
                    for (const out of t.outputs) {
                        if (!visited.has(out)) {
                            visited.add(out);
                            queue.push({ place: out, dist: curr.dist + 1 });
                        }
                    }
                }
            }
        }
        return 10; // Default penalty if sink is unreachable
    }

    /**
     * Admissible heuristic estimating remaining distance to the final marking.
     */
    function calculateHeuristic(marking, logIndex, trace, petriNet) {
        // Remaining activities in the log
        const logRemaining = trace.length - logIndex;
        
        // Shortest distance from current tokens to the final place
        let modelRemaining = 0;
        for (const place in marking) {
            if (marking[place] > 0) {
                modelRemaining = Math.max(modelRemaining, getPlaceDistance(place, petriNet));
            }
        }
        
        // Return admissible heuristic (the max of log and model remaining is admissible)
        return Math.max(logRemaining, modelRemaining);
    }

    /**
     * Checks if a transition is enabled in the current marking.
     */
    function isTransitionEnabled(transition, marking) {
        for (const input of transition.inputs) {
            if (!marking[input] || marking[input] < 1) {
                return false;
            }
        }
        return true;
    }

    /**
     * Fires a transition, consuming and producing tokens.
     */
    function fireTransition(transition, marking) {
        const newMarking = { ...marking };
        for (const input of transition.inputs) {
            newMarking[input] = (newMarking[input] || 0) - 1;
            if (newMarking[input] === 0) {
                delete newMarking[input];
            }
        }
        for (const output of transition.outputs) {
            newMarking[output] = (newMarking[output] || 0) + 1;
        }
        return newMarking;
    }

    /**
     * Compares two markings for equality.
     */
    function isFinalMarkingReached(marking, petriNet) {
        const target = petriNet.finalPlace || "p_end";
        const keys = Object.keys(marking);
        return keys.length === 1 && keys[0] === target && marking[target] === 1;
    }

    /**
     * Encodes marking state to a string key for duplicate checking.
     */
    function encodeStateKey(marking, logIndex) {
        const sortedPlaces = Object.keys(marking).sort().map(p => `${p}:${marking[p]}`).join(",");
        return `${logIndex}|${sortedPlaces}`;
    }

    /**
     * Solves the optimal alignment of a trace against a Petri Net using A* Search.
     */
    function solveAlignment(trace, petriNet) {
        const startMarking = { ...petriNet.initialMarking };
        const openSet = [];
        const closedSet = new Set();

        const startState = {
            marking: startMarking,
            logIndex: 0,
            g: 0, // cost of moves so far
            h: calculateHeuristic(startMarking, 0, trace, petriNet),
            moves: []
        };
        startState.f = startState.g + startState.h;
        openSet.push(startState);

        let iterations = 0;
        const maxIterations = 5000; // Safeguard boundary

        while (openSet.length > 0 && iterations < maxIterations) {
            iterations++;
            // Sort openSet by f, then by g (favor deeper exploration)
            openSet.sort((a, b) => a.f - b.f || b.logIndex - a.logIndex);
            
            const curr = openSet.shift();

            // Check if goal state is reached
            const isFinished = curr.logIndex === trace.length;
            const isMarkingAtEnd = isFinalMarkingReached(curr.marking, petriNet);

            if (isFinished && isMarkingAtEnd) {
                return {
                    alignment: curr.moves,
                    cost: curr.g,
                    fitness: 1 - (curr.g / (trace.length + getPlaceDistance(petriNet.initialPlace || "p_start", petriNet))),
                    success: true,
                    iterations
                };
            }

            const stateKey = encodeStateKey(curr.marking, curr.logIndex);
            if (closedSet.has(stateKey)) continue;
            closedSet.add(stateKey);

            // 1. Successor: SYNCHRONOUS MOVE
            if (curr.logIndex < trace.length) {
                const currentActivity = trace[curr.logIndex];
                // Find transition matching activity
                const transitions = petriNet.transitions.filter(t => t.name === currentActivity);
                for (const t of transitions) {
                    if (isTransitionEnabled(t, curr.marking)) {
                        const nextMarking = fireTransition(t, curr.marking);
                        const newMoves = [...curr.moves, {
                            type: "sync",
                            activity: currentActivity,
                            transition: t.name
                        }];
                        const g = curr.g; // cost = 0
                        const h = calculateHeuristic(nextMarking, curr.logIndex + 1, trace, petriNet);
                        openSet.push({
                            marking: nextMarking,
                            logIndex: curr.logIndex + 1,
                            g,
                            h,
                            f: g + h,
                            moves: newMoves
                        });
                    }
                }
            }

            // 2. Successor: MOVE ON MODEL
            for (const t of petriNet.transitions) {
                if (isTransitionEnabled(t, curr.marking)) {
                    const nextMarking = fireTransition(t, curr.marking);
                    const newMoves = [...curr.moves, {
                        type: "model",
                        activity: null,
                        transition: t.name
                    }];
                    const g = curr.g + 1; // cost = 1
                    const h = calculateHeuristic(nextMarking, curr.logIndex, trace, petriNet);
                    openSet.push({
                        marking: nextMarking,
                        logIndex: curr.logIndex,
                        g,
                        h,
                        f: g + h,
                        moves: newMoves
                    });
                }
            }

            // 3. Successor: MOVE ON LOG
            if (curr.logIndex < trace.length) {
                const currentActivity = trace[curr.logIndex];
                const newMoves = [...curr.moves, {
                    type: "log",
                    activity: currentActivity,
                    transition: null
                }];
                const g = curr.g + 1; // cost = 1
                const h = calculateHeuristic(curr.marking, curr.logIndex + 1, trace, petriNet);
                openSet.push({
                    marking: curr.marking,
                    logIndex: curr.logIndex + 1,
                    g,
                    h,
                    f: g + h,
                    moves: newMoves
                });
            }
        }

        // Fallback alignment (all log moves followed by model transitions to end)
        const fallbackMoves = [];
        trace.forEach(a => {
            fallbackMoves.push({ type: "log", activity: a, transition: null });
        });
        fallbackMoves.push({ type: "model", activity: null, transition: "Close (Auto)" });

        return {
            alignment: fallbackMoves,
            cost: trace.length + 1,
            fitness: 0.0,
            success: false,
            iterations
        };
    }

    /**
     * Renders the alignment diagram in HTML.
     */
    function renderAlignment(container, alignmentData) {
        container.innerHTML = "";

        const wrapper = document.createElement("div");
        wrapper.className = "alignment-results-container";

        const summary = document.createElement("div");
        summary.className = "alignment-summary";
        const fitScore = (alignmentData.fitness * 100).toFixed(1);
        
        summary.innerHTML = `
            <div class="alignment-metric-card">
                <span class="metric-label">Alignment Fitness</span>
                <span class="metric-value ${alignmentData.fitness > 0.8 ? 'text-success' : 'text-danger'}">${fitScore}%</span>
            </div>
            <div class="alignment-metric-card">
                <span class="metric-label">Alignment Cost</span>
                <span class="metric-value">${alignmentData.cost}</span>
            </div>
            <div class="alignment-metric-card">
                <span class="metric-label">A* Iterations</span>
                <span class="metric-value text-muted">${alignmentData.iterations}</span>
            </div>
        `;
        wrapper.appendChild(summary);

        // Alignment Blocks Visualization
        const blocksWrapper = document.createElement("div");
        blocksWrapper.className = "alignment-blocks";

        alignmentData.alignment.forEach(move => {
            const block = document.createElement("div");
            block.className = `alignment-block ${move.type}`;
            
            let topText = "";
            let bottomText = "";
            
            if (move.type === "sync") {
                topText = move.activity;
                bottomText = move.transition;
            } else if (move.type === "log") {
                topText = move.activity;
                bottomText = "-";
            } else if (move.type === "model") {
                topText = "-";
                bottomText = move.transition;
            }

            block.innerHTML = `
                <div class="move-label">${move.type.toUpperCase()}</div>
                <div class="move-log-row" title="Activity in Event Log">
                    <span class="row-hdr">Log:</span>
                    <span class="row-val">${topText}</span>
                </div>
                <div class="move-model-row" title="Transition in Petri Net">
                    <span class="row-hdr">Model:</span>
                    <span class="row-val">${bottomText}</span>
                </div>
                <div class="move-cost">Cost: ${move.type === 'sync' ? '0' : '1'}</div>
            `;
            blocksWrapper.appendChild(block);
        });

        wrapper.appendChild(blocksWrapper);
        container.appendChild(wrapper);
    }

    return {
        solveAlignment,
        renderAlignment
    };
})();

// Export globally for browser use
window.ProcessAlignment = ProcessAlignment;
