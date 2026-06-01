/**
 * ==========================================================================
 * A* Process Alignment Solver
 * ==========================================================================
 */

class PriorityQueue {
  constructor() {
    this.elements = [];
  }

  enqueue(element, priority) {
    this.elements.push({ element, priority });
    this.elements.sort((a, b) => a.priority - b.priority);
  }

  dequeue() {
    return this.elements.shift().element;
  }

  isEmpty() {
    return this.elements.length === 0;
  }
}

class AStarAlignmentSolver {
  constructor() {
    // Petri net definition
    // Places: P1 (Start), P2 (Received), P3 (Checked), P4 (Approved), P5 (Shipped), P6 (End)
    // Transitions consume from input places and produce in output places
    this.transitions = [
      { name: "Receive Order", inputs: [0], outputs: [1] },
      { name: "Check Inventory", inputs: [1], outputs: [2] },
      { name: "Approve", inputs: [2], outputs: [3] },
      { name: "Approve", inputs: [1], outputs: [3] }, // Bypass check inventory
      { name: "Ship", inputs: [3], outputs: [4] },
      { name: "End", inputs: [4], outputs: [5] }
    ];
  }

  // Helper to check if a transition is enabled under a given marking
  isEnabled(transition, marking) {
    return transition.inputs.every(placeIdx => marking[placeIdx] > 0);
  }

  // Fire transition and return next marking
  fire(transition, marking) {
    const nextMarking = [...marking];
    transition.inputs.forEach(placeIdx => nextMarking[placeIdx]--);
    transition.outputs.forEach(placeIdx => nextMarking[placeIdx]++);
    return nextMarking;
  }

  // Admissible heuristic function
  heuristic(marking, logIndex, logTraceLength) {
    const remainingLog = logTraceLength - logIndex;
    
    // Estimate remaining model distance to P6 (End)
    let modelDist = 0;
    if (marking[5] > 0) modelDist = 0;
    else if (marking[4] > 0) modelDist = 1; // Ship -> End
    else if (marking[3] > 0) modelDist = 2; // Approve -> Ship -> End
    else if (marking[2] > 0) modelDist = 3; // Checked -> Approve -> Ship -> End
    else if (marking[1] > 0) modelDist = 3; // Bypass check: Approve -> Ship -> End
    else if (marking[0] > 0) modelDist = 4; // Receive Order -> ...
    else modelDist = 4; // default safe fallback

    // Admissible heuristic: minimum extra moves required due to mismatch in length
    return Math.abs(remainingLog - modelDist);
  }

  solve(logTrace) {
    const startTime = performance.now();
    const pq = new PriorityQueue();
    const visited = new Set();
    let nodesExpanded = 0;

    const startMarking = [1, 0, 0, 0, 0, 0];
    const startState = {
      marking: startMarking,
      logIndex: 0,
      g: 0,
      path: [] // List of alignment steps
    };

    const startH = this.heuristic(startMarking, 0, logTrace.length);
    pq.enqueue(startState, startState.g + startH);

    while (!pq.isEmpty()) {
      const current = pq.dequeue();
      nodesExpanded++;

      // Goal test: Petri Net has token in P6, and all log events consumed
      if (current.marking[5] > 0 && current.logIndex === logTrace.length) {
        const duration = performance.now() - startTime;
        return {
          alignment: current.path,
          cost: current.g,
          nodesExpanded,
          duration: duration.toFixed(2),
          fitness: this.calculateFitness(current.g, logTrace.length)
        };
      }

      const stateKey = `${current.marking.join(",")}|${current.logIndex}`;
      if (visited.has(stateKey)) continue;
      visited.add(stateKey);

      const currentLogEvent = current.logIndex < logTrace.length ? logTrace[current.logIndex] : null;

      // 1. SYNCHRONOUS MOVES
      // If model transition is enabled AND matches the current log event
      if (currentLogEvent) {
        this.transitions.forEach(transition => {
          if (transition.name === currentLogEvent && this.isEnabled(transition, current.marking)) {
            const nextMarking = this.fire(transition, current.marking);
            const nextState = {
              marking: nextMarking,
              logIndex: current.logIndex + 1,
              g: current.g, // Cost = 0
              path: [...current.path, { type: "sync", logAct: transition.name, modelAct: transition.name }]
            };
            const h = this.heuristic(nextMarking, nextState.logIndex, logTrace.length);
            pq.enqueue(nextState, nextState.g + h);
          }
        });
      }

      // 2. MOVE ON MODEL
      // Fire any enabled transition in the model, log index stays same
      this.transitions.forEach(transition => {
        if (this.isEnabled(transition, current.marking)) {
          const nextMarking = this.fire(transition, current.marking);
          const nextState = {
            marking: nextMarking,
            logIndex: current.logIndex,
            g: current.g + 1, // Cost = 1
            path: [...current.path, { type: "model", logAct: "»", modelAct: transition.name }]
          };
          const h = this.heuristic(nextMarking, nextState.logIndex, logTrace.length);
          pq.enqueue(nextState, nextState.g + h);
        }
      });

      // 3. MOVE ON LOG
      // Advance log index, model marking stays same
      if (currentLogEvent) {
        const nextState = {
          marking: current.marking,
          logIndex: current.logIndex + 1,
          g: current.g + 1, // Cost = 1
          path: [...current.path, { type: "log", logAct: currentLogEvent, modelAct: "»" }]
        };
        const h = this.heuristic(current.marking, nextState.logIndex, logTrace.length);
        pq.enqueue(nextState, nextState.g + h);
      }
    }

    // Fallback: If no alignment found (should not happen with Move on Log fallback)
    const duration = performance.now() - startTime;
    return {
      alignment: logTrace.map(act => ({ type: "log", logAct: act, modelAct: "»" })),
      cost: logTrace.length,
      nodesExpanded,
      duration: duration.toFixed(2),
      fitness: 0.0
    };
  }

  calculateFitness(cost, logLength) {
    // Classic alignment fitness: 1 - cost / (log_cost + model_cost)
    // Model cost for genesis-to-end is 4 (min path: Receive -> Approve bypass -> Ship -> End)
    const minModelCost = 4;
    const denominator = logLength + minModelCost;
    if (denominator === 0) return 1.0;
    return (1 - cost / denominator).toFixed(2);
  }
}

// Global initialization
window.alignmentSolver = new AStarAlignmentSolver();
