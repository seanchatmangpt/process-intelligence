/**
 * Autonomic Feedback Loop Controls (MAPE-K)
 * 
 * Governed under the authority of the Blue River Dam Epistemic Containment Protocol.
 * For details on the containment doctrine, see file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md
 * For the Autonomic Knowledge Actuation specification, see file:///Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md
 * For the general lifecycle stages, see file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md
 */

// ==========================================================================
// 1. PURE JAVASCRIPT STATELESS SHA-256 IMPLEMENTATION (Cryptographic Lineage)
// ==========================================================================

export function sha256(ascii) {
  function rightRotate(value, amount) {
    return (value >>> amount) | (value << (32 - amount));
  }
  
  const mathPow = Math.pow;
  const maxWord = mathPow(2, 32);
  const lengthProperty = 'length';
  let i, j;
  let result = '';

  const words = [];
  const asciiLength = ascii[lengthProperty] * 8;
  
  const hash = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
  
  const k = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
  ];
  
  let paddedAscii = ascii + '\x80';
  while (paddedAscii[lengthProperty] % 64 - 56) paddedAscii += '\x00';
  
  for (i = 0; i < paddedAscii[lengthProperty]; i++) {
    j = paddedAscii.charCodeAt(i);
    words[i >> 2] |= j << ((3 - i % 4) * 8);
  }
  words[words[lengthProperty]] = ((asciiLength / maxWord) | 0);
  words[words[lengthProperty]] = (asciiLength | 0);
  
  for (j = 0; j < words[lengthProperty];) {
    const w = words.slice(j, j += 16);
    const oldHash = hash.slice(0);
    
    for (i = 0; i < 64; i++) {
      const w16 = w[i - 16], w15 = w[i - 15], w7 = w[i - 7], w2 = w[i - 2];
      const a = hash[0], e = hash[4];
      let temp1, temp2;
      
      const s0 = rightRotate(a, 2) ^ rightRotate(a, 13) ^ rightRotate(a, 22);
      const maj = (a & hash[1]) ^ (a & hash[2]) ^ (hash[1] & hash[2]);
      temp2 = s0 + maj;
      
      const s1 = rightRotate(e, 6) ^ rightRotate(e, 11) ^ rightRotate(e, 25);
      const ch = (e & hash[5]) ^ (~e & hash[6]);
      temp1 = hash[7] + s1 + ch + k[i] + (i < 16 ? w[i] : (
        w[i] = (w16 + (
          rightRotate(w15, 7) ^ rightRotate(w15, 18) ^ (w15 >>> 3)
        ) + w7 + (
          rightRotate(w2, 17) ^ rightRotate(w2, 19) ^ (w2 >>> 10)
        )) | 0
      ));
      
      hash.unshift((temp1 + temp2) | 0);
      hash[4] = (hash[4] + temp1) | 0;
      hash.length = 8;
    }
    
    for (i = 0; i < 8; i++) {
      hash[i] = (hash[i] + oldHash[i]) | 0;
    }
  }
  
  for (i = 0; i < 8; i++) {
    const s = (hash[i] >>> 0).toString(16);
    result += ("00000000" + s).slice(-8);
  }
  return result;
}

// ==========================================================================
// 2. CRYPTOGRAPHIC EVENT AUDIT LEDGER (SHA-256 Chaining)
// ==========================================================================

export class AuditLedger {
  constructor() {
    this.chain = [];
    this.createGenesisBlock();
  }

  createGenesisBlock() {
    const block = {
      index: 0,
      timestamp: Date.now(),
      eventType: "GENESIS",
      data: { message: "Blue River Dam Autonomic Ledger Initialized" },
      previousHash: "0000000000000000000000000000000000000000000000000000000000000000"
    };
    block.hash = this.calculateHash(block);
    this.chain.push(block);
  }

  calculateHash(block) {
    const payload = block.index + 
                    block.timestamp + 
                    block.eventType + 
                    JSON.stringify(block.data) + 
                    block.previousHash;
    return sha256(payload);
  }

  addEvent(eventType, data) {
    const previousBlock = this.chain[this.chain.length - 1];
    const newBlock = {
      index: this.chain.length,
      timestamp: Date.now(),
      eventType: eventType,
      data: data,
      previousHash: previousBlock.hash
    };
    newBlock.hash = this.calculateHash(newBlock);
    this.chain.push(newBlock);
    return newBlock;
  }

  verifyChain() {
    for (let i = 1; i < this.chain.length; i++) {
      const currentBlock = this.chain[i];
      const previousBlock = this.chain[i - 1];

      if (currentBlock.hash !== this.calculateHash(currentBlock)) {
        return false;
      }
      if (currentBlock.previousHash !== previousBlock.hash) {
        return false;
      }
    }
    return true;
  }
}

// ==========================================================================
// 3. EWMA DRIFT CALCULATOR (Monitor / Analyze Stage)
// ==========================================================================

export class EWMACalculator {
  /**
   * @param {number} alpha Smoothing parameter (0 < alpha <= 1)
   * @param {number} initialValue Baseline starting value
   */
  constructor(alpha = 0.2, initialValue = 1.0) {
    this.alpha = alpha;
    this.value = initialValue;
  }

  update(newValue) {
    this.value = (this.alpha * newValue) + ((1.0 - this.alpha) * this.value);
    return this.value;
  }

  getVal() {
    return this.value;
  }
}

// ==========================================================================
// 4. PETRI NET MODELS & S-COMPONENT REPRESENTATION
// ==========================================================================

export class Place {
  constructor(id, name, tokens = 0) {
    this.id = id;
    this.name = name;
    this.tokens = tokens;
  }
}

export class Transition {
  constructor(id, name, label = "") {
    this.id = id;
    this.name = name;
    this.label = label || name; // Label maps to event log activity
    this.isEnabled = false;
    this.firing = false;
    // Dynamic guard condition for routing
    this.guard = () => true; 
  }
}

export class Arc {
  constructor(id, sourceId, targetId, type = "normal") {
    this.id = id;
    this.sourceId = sourceId; // can be place or transition ID
    this.targetId = targetId; // can be transition or place ID
    this.type = type;         // "normal" or "inhibitor"
  }
}

export class SComponent {
  /**
   * Defines a strongly-connected sub-net where every transition has
   * at most 1 input and at most 1 output place inside the component.
   */
  constructor(id, name, placeIds = [], transitionIds = []) {
    this.id = id;
    this.name = name;
    this.placeIds = new Set(placeIds);
    this.transitionIds = new Set(transitionIds);
  }
}

export class PetriNet {
  constructor() {
    this.places = new Map();
    this.transitions = new Map();
    this.arcs = [];
    this.sComponents = new Map();
    this.activeRoutingRules = new Map(); // Dynamic route overrides
  }

  addPlace(id, name, tokens = 0) {
    const place = new Place(id, name, tokens);
    this.places.set(id, place);
    return place;
  }

  addTransition(id, name, label = "") {
    const trans = new Transition(id, name, label);
    this.transitions.set(id, trans);
    return trans;
  }

  addArc(id, sourceId, targetId, type = "normal") {
    const arc = new Arc(id, sourceId, targetId, type);
    this.arcs.push(arc);
    return arc;
  }

  addSComponent(id, name, placeIds, transitionIds) {
    const sComp = new SComponent(id, name, placeIds, transitionIds);
    this.sComponents.set(id, sComp);
    return sComp;
  }

  getPreset(nodeId) {
    // Preset of a place/transition are the sources of incoming arcs
    return this.arcs
      .filter(arc => arc.targetId === nodeId)
      .map(arc => arc.sourceId);
  }

  getPostset(nodeId) {
    // Postset of a place/transition are the targets of outgoing arcs
    return this.arcs
      .filter(arc => arc.sourceId === nodeId)
      .map(arc => arc.targetId);
  }

  getMarking() {
    const marking = {};
    for (const [id, place] of this.places.entries()) {
      if (place.tokens > 0) {
        marking[id] = place.tokens;
      }
    }
    return marking;
  }

  setMarking(marking) {
    for (const place of this.places.values()) {
      place.tokens = marking[place.id] || 0;
    }
  }

  checkEnabledTransitions() {
    const enabled = [];
    for (const trans of this.transitions.values()) {
      const presets = this.getPreset(trans.id);
      let isTransEnabled = true;

      if (presets.length === 0) {
        isTransEnabled = false;
      } else {
        for (const pId of presets) {
          const place = this.places.get(pId);
          // Standard Petri net transition firing requires at least 1 token in preset places
          if (!place || place.tokens < 1) {
            isTransEnabled = false;
            break;
          }
        }
      }

      // Check dynamic guard condition
      if (isTransEnabled && !trans.guard()) {
        isTransEnabled = false;
      }

      trans.isEnabled = isTransEnabled;
      if (isTransEnabled) {
        enabled.push(trans.id);
      }
    }
    return enabled;
  }

  fireTransition(transId) {
    const trans = this.transitions.get(transId);
    if (!trans) return false;

    // Check enablement
    this.checkEnabledTransitions();
    if (!trans.isEnabled) return false;

    // Fire! Consume tokens from preset
    const presets = this.getPreset(transId);
    for (const pId of presets) {
      const place = this.places.get(pId);
      place.tokens = Math.max(0, place.tokens - 1);
    }

    // Produce tokens in postset
    const postsets = this.getPostset(transId);
    for (const pId of postsets) {
      const place = this.places.get(pId);
      if (place) place.tokens += 1;
    }

    // Reset status
    this.checkEnabledTransitions();
    return true;
  }

  /**
   * HOT-SWAP S-COMPONENT: Replaces an entire sub-net topology
   * while migrating token markings safely.
   */
  hotSwapSComponent(oldCompId, newSComponentData) {
    const oldComp = this.sComponents.get(oldCompId);
    if (!oldComp) throw new Error(`S-Component ${oldCompId} not found.`);

    // 1. Identify boundary places (places in the old component connected to transitions outside)
    // and extract active token markings inside the old component
    const savedMarkings = {};
    for (const pId of oldComp.placeIds) {
      const place = this.places.get(pId);
      if (place) {
        savedMarkings[pId] = place.tokens;
        // Delete places from map
        this.places.delete(pId);
      }
    }

    // Delete transitions in the old component
    for (const tId of oldComp.transitionIds) {
      this.transitions.delete(tId);
    }

    // Delete related arcs
    this.arcs = this.arcs.filter(arc => {
      const isInternalSource = oldComp.placeIds.has(arc.sourceId) || oldComp.transitionIds.has(arc.sourceId);
      const isInternalTarget = oldComp.placeIds.has(arc.targetId) || oldComp.transitionIds.has(arc.targetId);
      return !(isInternalSource || isInternalTarget);
    });

    // 2. Insert new S-Component elements
    const { places, transitions, arcs, placeIds, transitionIds, markingMigrationMap } = newSComponentData;

    for (const p of places) {
      this.addPlace(p.id, p.name, 0);
    }
    for (const t of transitions) {
      this.addTransition(t.id, t.name, t.label);
    }
    for (const a of arcs) {
      this.addArc(a.id, a.sourceId, a.targetId, a.type);
    }

    // Migrate tokens based on mapping
    for (const [oldPlaceId, newPlaceId] of Object.entries(markingMigrationMap)) {
      const oldTokens = savedMarkings[oldPlaceId] || 0;
      const newPlace = this.places.get(newPlaceId);
      if (newPlace) {
        newPlace.tokens += oldTokens;
      }
    }

    // Replace S-component registry entry
    this.sComponents.delete(oldCompId);
    const newComp = new SComponent(newSComponentData.id, newSComponentData.name, placeIds, transitionIds);
    this.sComponents.set(newSComponentData.id, newComp);

    // Refresh enablement status
    this.checkEnabledTransitions();
    return true;
  }
}

// ==========================================================================
// 5. ACTUAL A* SEARCH ALIGNMENT CONFORMANCE SOLVER
// ==========================================================================

export class AlignmentSolver {
  /**
   * Computes the optimal alignment of an observed trace against a Petri Net model.
   * Finds a sequence of moves (Sync, Model-Only, Log-Only) that minimizes edit distance.
   * 
   * @param {string[]} trace Array of observed activity names, e.g. ["verify", "anomaly", "settle"]
   * @param {PetriNet} net The Petri Net to align against
   * @param {string} startPlaceId Initial place
   * @param {string} endPlaceId Sink place
   * @returns {object} Object containing cost, alignment path, and fitness score
   */
  static solve(trace, net, startPlaceId = "p_start", endPlaceId = "p_decommissioned") {
    const initialMarking = { [startPlaceId]: 1 };
    
    // A* Node Definition
    // state key: JSON.stringify(marking) + "_" + traceIdx
    const openSet = [{
      marking: initialMarking,
      traceIdx: 0,
      g: 0,      // actual path cost
      h: trace.length, // Heuristic: remaining trace items to align
      f: trace.length, // f = g + h
      path: []   // array of moves: { type: "sync"|"model"|"log", label, transId }
    }];

    const closedSet = new Set();
    let bestNode = null;

    // Helper to compare markings
    function isFinalState(marking, traceIdx) {
      return traceIdx === trace.length && 
             marking[endPlaceId] === 1 && 
             Object.keys(marking).length === 1;
    }

    // Helper to serialize marking for visited check
    function serializeState(marking, traceIdx) {
      const keys = Object.keys(marking).sort();
      const markStr = keys.map(k => `${k}:${marking[k]}`).join(",");
      return `${markStr}_${traceIdx}`;
    }

    let iterations = 0;
    const maxIterations = 2000; // Limit execution search boundaries in sandbox

    while (openSet.length > 0 && iterations < maxIterations) {
      iterations++;
      
      // Sort openSet by f cost (ascending), then by g cost (descending, depth-first tie-breaker)
      openSet.sort((a, b) => {
        if (a.f !== b.f) return a.f - b.f;
        return b.g - a.g;
      });

      const current = openSet.shift();

      // Check if goal reached
      if (isFinalState(current.marking, current.traceIdx)) {
        bestNode = current;
        break;
      }

      const stateKey = serializeState(current.marking, current.traceIdx);
      if (closedSet.has(stateKey)) continue;
      closedSet.add(stateKey);

      // --- 1. EXPLORE MOVES ON MODEL (Model-Only Moves) ---
      // Instantiate a temporary net to check enabled transitions under current marking
      const tempNet = new PetriNet();
      tempNet.places = new Map(Object.entries(net.places).map(([k, v]) => [k, new Place(v.id, v.name, current.marking[v.id] || 0)]));
      tempNet.transitions = new Map(net.transitions);
      tempNet.arcs = [...net.arcs];

      const enabledTransitions = tempNet.checkEnabledTransitions();
      for (const tId of enabledTransitions) {
        // Fire transition to get new marking
        const fireNet = new PetriNet();
        fireNet.places = new Map(Object.entries(tempNet.places).map(([k, v]) => [k, new Place(v.id, v.name, v.tokens)]));
        fireNet.transitions = new Map(tempNet.transitions);
        fireNet.arcs = [...tempNet.arcs];
        fireNet.fireTransition(tId);

        const nextMarking = fireNet.getMarking();
        const trans = net.transitions.get(tId);
        
        // Model-only cost: 1 (if visible transition) or 0 (if silent/tau transition)
        const isSilent = trans.label.startsWith("tau") || trans.label === "";
        const cost = isSilent ? 0 : 1;

        const nextState = {
          marking: nextMarking,
          traceIdx: current.traceIdx,
          g: current.g + cost,
          h: trace.length - current.traceIdx,
          f: 0,
          path: [...current.path, { type: "model", label: trans.label, transId: tId }]
        };
        nextState.f = nextState.g + nextState.h;

        const key = serializeState(nextState.marking, nextState.traceIdx);
        if (!closedSet.has(key)) {
          openSet.push(nextState);
        }
      }

      // --- 2. EXPLORE MOVES ON LOG (Log-Only Moves) ---
      if (current.traceIdx < trace.length) {
        const nextState = {
          marking: { ...current.marking },
          traceIdx: current.traceIdx + 1,
          g: current.g + 1, // Log-only cost = 1
          h: trace.length - (current.traceIdx + 1),
          f: 0,
          path: [...current.path, { type: "log", label: trace[current.traceIdx] }]
        };
        nextState.f = nextState.g + nextState.h;

        const key = serializeState(nextState.marking, nextState.traceIdx);
        if (!closedSet.has(key)) {
          openSet.push(nextState);
        }
      }

      // --- 3. EXPLORE SYNCHRONOUS MOVES (Sync Moves) ---
      if (current.traceIdx < trace.length) {
        const currentEvent = trace[current.traceIdx];
        
        for (const tId of enabledTransitions) {
          const trans = net.transitions.get(tId);
          if (trans.label === currentEvent) {
            // Fire transition to get new marking
            const fireNet = new PetriNet();
            fireNet.places = new Map(Object.entries(tempNet.places).map(([k, v]) => [k, new Place(v.id, v.name, v.tokens)]));
            fireNet.transitions = new Map(tempNet.transitions);
            fireNet.arcs = [...tempNet.arcs];
            fireNet.fireTransition(tId);

            const nextMarking = fireNet.getMarking();
            const nextState = {
              marking: nextMarking,
              traceIdx: current.traceIdx + 1,
              g: current.g + 0, // Synchronous cost = 0
              h: trace.length - (current.traceIdx + 1),
              f: 0,
              path: [...current.path, { type: "sync", label: currentEvent, transId: tId }]
            };
            nextState.f = nextState.g + nextState.h;

            const key = serializeState(nextState.marking, nextState.traceIdx);
            if (!closedSet.has(key)) {
              openSet.push(nextState);
            }
          }
        }
      }
    }

    // If search failed to complete, compute a fallback heuristic score
    if (!bestNode) {
      return {
        cost: trace.length + 5,
        path: [{ type: "failure", label: "Solver limit hit" }],
        fitness: 0.0
      };
    }

    // Cost of empty log align = shortest path cost from start to end in net (approximate or exact)
    // To simplify and ensure correctness, worst possible cost matches: trace.length + model_only_depth
    const worstCost = trace.length + Object.keys(net.transitions).length;
    const fitness = worstCost > 0 ? 1.0 - (bestNode.g / worstCost) : 1.0;

    return {
      cost: bestNode.g,
      path: bestNode.path,
      fitness: parseFloat(Math.max(0.0, Math.min(1.0, fitness)).toFixed(4))
    };
  }
}

// ==========================================================================
// 6. AUTONOMIC CONTROLLER (MAPE-K feedback loop orchestrator)
// ==========================================================================

export class AutonomicController {
  constructor() {
    this.ledger = new AuditLedger();
    
    // Monitors
    this.fitnessEWMA = new EWMACalculator(0.15, 1.0);
    this.networkLoadEWMA = new EWMACalculator(0.20, 0.10);
    
    // Knowledge parameters
    this.conformanceThreshold = 0.90;
    this.loadLimitThreshold = 0.75;
    
    // Control status
    this.isThrottled = false;
    this.throttledRate = 1.0; // 1.0 = full speed, 0.1 = maximum throttle (90% delay increase)
    this.routeBypassActive = false;
    this.hotSwapExecuted = false;

    // S-Component Definitions
    this.setupSComponents();
  }

  setupSComponents() {
    // Defines standard, baseline verification component
    this.standardVerificationComponent = {
      id: "S_Verify_Auth",
      name: "Standard Transaction Verification & Auth",
      placeIds: new Set(["p_checked", "p_auth_pending"]),
      transitionIds: new Set(["t_verify", "t_authorize"]),
      places: [
        { id: "p_checked", name: "Transaction Checked" },
        { id: "p_auth_pending", name: "Authorization Pending" }
      ],
      transitions: [
        { id: "t_verify", name: "Verify Ingestion", label: "t_verify" },
        { id: "t_authorize", name: "Authorize Funds", label: "t_authorize" }
      ],
      arcs: [
        { id: "a_v_1", sourceId: "p_ingested", targetId: "t_verify", type: "normal" },
        { id: "a_v_2", sourceId: "t_verify", targetId: "p_checked", type: "normal" },
        { id: "a_v_3", sourceId: "p_checked", targetId: "t_authorize", type: "normal" },
        { id: "a_v_4", sourceId: "t_authorize", targetId: "p_auth_pending", type: "normal" },
        { id: "a_v_5", sourceId: "p_auth_pending", targetId: "t_settle", type: "normal" }
      ],
      markingMigrationMap: {
        "p_checked": "p_checked",
        "p_auth_pending": "p_auth_pending"
      }
    };

    // Defines a hardened, self-healing, autonomic S-component to hot-swap into the runtime
    this.hardenedVerificationComponent = {
      id: "S_Verify_Auth",
      name: "Hardened Autonomic S-Component (Secure Routing)",
      placeIds: new Set(["p_checked_hardened", "p_escalated_auth"]),
      transitionIds: new Set(["t_verify_hardened", "t_escalate_auth"]),
      places: [
        { id: "p_checked_hardened", name: "Audit Trail Enforced" },
        { id: "p_escalated_auth", name: "Escalated Authorization Complete" }
      ],
      transitions: [
        { id: "t_verify_hardened", name: "Hardened Audit Scan", label: "t_verify_hardened" },
        { id: "t_escalate_auth", name: "Cryptographic Credential Scan", label: "t_escalate_auth" }
      ],
      arcs: [
        { id: "a_vh_1", sourceId: "p_ingested", targetId: "t_verify_hardened", type: "normal" },
        { id: "a_vh_2", sourceId: "t_verify_hardened", targetId: "p_checked_hardened", type: "normal" },
        { id: "a_vh_3", sourceId: "p_checked_hardened", targetId: "t_escalate_auth", type: "normal" },
        { id: "a_vh_4", sourceId: "t_escalate_auth", targetId: "p_escalated_auth", type: "normal" },
        { id: "a_vh_5", sourceId: "p_escalated_auth", targetId: "t_settle", type: "normal" }
      ],
      markingMigrationMap: {
        "p_checked": "p_checked_hardened",
        "p_auth_pending": "p_escalated_auth"
      }
    };
  }

  /**
   * MAPE-K FEEDBACK LOOP ENTRY POINT
   * 
   * @param {object} metrics Real-time metrics from the simulator
   * @param {PetriNet} net Current Petri Net instance
   * @returns {object} Summary of planning & execution decisions
   */
  tick(metrics, net) {
    // 1. MONITOR STAGE: Record real-time raw values and compute EWMA averages
    const currentFitness = metrics.lastTraceFitness;
    const currentLoad = metrics.currentNetworkLoad;
    
    const ewmaFitness = this.fitnessEWMA.update(currentFitness);
    const ewmaLoad = this.networkLoadEWMA.update(currentLoad);
    
    const decisions = {
      timestamp: Date.now(),
      metrics: {
        rawFitness: currentFitness,
        ewmaFitness: ewmaFitness,
        rawLoad: currentLoad,
        ewmaLoad: ewmaLoad
      },
      actions: []
    };

    // 2. ANALYZE STAGE: Evaluate against policy constraints (LTL & Drift thresholds)
    const isFitnessBelowThreshold = ewmaFitness < this.conformanceThreshold;
    const isLoadAboveThreshold = ewmaLoad > this.loadLimitThreshold;

    // 3. PLAN & EXECUTE STAGE: Coordinate actuator feedback loop controls

    // A. RATE LIMITING ACTUATION (Monitor -> Execute)
    if (isLoadAboveThreshold || isFitnessBelowThreshold) {
      // Calculate dynamic throttling factor based on load overload
      const loadOverload = Math.max(0, ewmaLoad - this.loadLimitThreshold);
      const violationSeverity = Math.max(0, this.conformanceThreshold - ewmaFitness);
      
      // Reduce throughput proportionally (minimum throttled rate 0.15 = 85% slowdown)
      const targetRate = Math.max(0.15, 1.0 - (loadOverload * 1.5 + violationSeverity * 2.0));
      
      if (!this.isThrottled || Math.abs(this.throttledRate - targetRate) > 0.05) {
        this.isThrottled = true;
        this.throttledRate = parseFloat(targetRate.toFixed(2));
        
        const actionData = { throttledRate: this.throttledRate, ewmaLoad, ewmaFitness };
        this.ledger.addEvent("RATE_LIMIT_TRIGGER", actionData);
        decisions.actions.push({ type: "RATE_LIMIT_TRIGGER", data: actionData });
      }
    } else {
      // Normal state recovery
      if (this.isThrottled) {
        this.isThrottled = false;
        this.throttledRate = 1.0;
        
        const actionData = { throttledRate: 1.0, message: "Load and Conformance restored to nominal ranges" };
        this.ledger.addEvent("RATE_LIMIT_RECOVERY", actionData);
        decisions.actions.push({ type: "RATE_LIMIT_RECOVERY", data: actionData });
      }
    }

    // B. DYNAMIC ROUTE CHANGES (Monitor -> Execute)
    // If fitness is collapsing severely (< 0.85), route events through secure bypass to avoid failures
    if (ewmaFitness < 0.85 && !this.routeBypassActive) {
      this.routeBypassActive = true;
      
      // Apply bypass guard logic inside Petri Net transitions
      // Standard transition verify is disabled, bypass path is enabled
      const tVerify = net.transitions.get("t_verify");
      if (tVerify) {
        tVerify.guard = () => false; // disable standard path
      }
      
      const tBypassVerify = net.transitions.get("t_bypass_verify");
      if (tBypassVerify) {
        tBypassVerify.guard = () => true; // force bypass path
      }

      const actionData = { message: "Diverted workflow to backup bypass route: t_bypass_verify activated." };
      this.ledger.addEvent("DYNAMIC_ROUTE_CHANGE", actionData);
      decisions.actions.push({ type: "DYNAMIC_ROUTE_CHANGE", data: actionData });
    } else if (ewmaFitness >= 0.95 && this.routeBypassActive) {
      // Restore standard routing once model matches conformant bounds
      this.routeBypassActive = false;
      const tVerify = net.transitions.get("t_verify");
      if (tVerify) {
        tVerify.guard = () => true;
      }
      
      const tBypassVerify = net.transitions.get("t_bypass_verify");
      if (tBypassVerify) {
        tBypassVerify.guard = () => false;
      }

      const actionData = { message: "Restored workflow pathing to standard. Bypass route disabled." };
      this.ledger.addEvent("DYNAMIC_ROUTE_RESTORE", actionData);
      decisions.actions.push({ type: "DYNAMIC_ROUTE_RESTORE", data: actionData });
    }

    // C. S-COMPONENT HOT-SWAPS (Monitor -> Plan -> Execute)
    // Under continuous low fitness (< 0.82) and high load, replace standard S-component with hardened version
    if (ewmaFitness < 0.82 && !this.hotSwapExecuted) {
      try {
        net.hotSwapSComponent("S_Verify_Auth", this.hardenedVerificationComponent);
        this.hotSwapExecuted = true;
        
        // Disable bypass route if active, as the new component itself is a robust resolution
        if (this.routeBypassActive) {
          const tBypassVerify = net.transitions.get("t_bypass_verify");
          if (tBypassVerify) tBypassVerify.guard = () => false;
        }

        const actionData = {
          swappedFrom: "S_Verify_Auth",
          swappedTo: "Hardened Autonomic S-Component (Secure Routing)",
          message: "Hot-swapped S-component. Markings migrated successfully."
        };
        this.ledger.addEvent("S_COMPONENT_HOT_SWAP", actionData);
        decisions.actions.push({ type: "S_COMPONENT_HOT_SWAP", data: actionData });
      } catch (error) {
        const failureData = { error: error.message };
        this.ledger.addEvent("S_COMPONENT_HOT_SWAP_FAILURE", failureData);
        decisions.actions.push({ type: "S_COMPONENT_HOT_SWAP_FAILURE", data: failureData });
      }
    } else if (ewmaFitness > 0.96 && this.hotSwapExecuted) {
      // Hot-swap recovery: return to standard S-component to optimize operational speed (since standard has lower overhead)
      try {
        net.hotSwapSComponent("S_Verify_Auth", this.standardVerificationComponent);
        this.hotSwapExecuted = false;
        
        const actionData = {
          swappedFrom: "Hardened Autonomic S-Component (Secure Routing)",
          swappedTo: "S_Verify_Auth",
          message: "Hot-swapped back to Standard S-component. Reverted safety margins to normal."
        };
        this.ledger.addEvent("S_COMPONENT_REVERT", actionData);
        decisions.actions.push({ type: "S_COMPONENT_REVERT", data: actionData });
      } catch (error) {
        console.error("Autonomic reversion error:", error);
      }
    }

    return decisions;
  }
}
