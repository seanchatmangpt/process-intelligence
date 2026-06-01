/**
 * Process Intelligence Dashboard Controller
 *
 * This file coordinates the interactive simulation, Petri Net SVG rendering,
 * token flow physics, EWMA concept drift monitoring, and the chained ledger logging.
 *
 * Theory & Placement standards:
 * - Petri Net Standard Ledger Placement: file:///Users/sac/process-intelligence/standards/petri_net_placement.md
 * - Petri Net Conformance & Token Game Replay: file:///Users/sac/process-intelligence/experiments/petri_conformance_sample.md
 */

// --- Cryptographic SHA-256 Implementation (Synchronous) ---
function sha256(ascii) {
  function rightRotate(value, amount) {
    return (value >>> amount) | (value << (32 - amount));
  }
  
  const mathPow = Math.pow;
  const maxWord = mathPow(2, 32);
  const lengthProperty = 'length';
  let i, j;
  
  let asciiLength = ascii[lengthProperty];
  const words = [];
  for (i = 0; i < asciiLength; i++) {
    j = ascii.charCodeAt(i);
    words[i >> 2] |= j << ((3 - i % 4) * 8);
  }
  
  asciiLength = asciiLength * 8;
  words[asciiLength >> 5] |= 0x80 << (24 - asciiLength % 32);
  words[(((asciiLength + 64) >>> 9) << 4) + 15] = asciiLength;
  
  const hash = [];
  const k = [];
  let primeCounter = 0;
  
  const isComposite = {};
  for (let candidate = 2; primeCounter < 64; candidate++) {
    if (!isComposite[candidate]) {
      for (i = 0; i < 313; i += candidate) {
        isComposite[i] = 1;
      }
      hash[primeCounter] = (mathPow(candidate, .5) * maxWord) | 0;
      k[primeCounter++] = (mathPow(candidate, 1 / 3) * maxWord) | 0;
    }
  }
  
  let h0 = hash[0], h1 = hash[1], h2 = hash[2], h3 = hash[3],
      h4 = hash[4], h5 = hash[5], h6 = hash[6], h7 = hash[7];
      
  for (i = 0; i < words[lengthProperty]; i += 16) {
    const w = words.slice(i, i + 16);
    const oldH0 = h0, oldH1 = h1, oldH2 = h2, oldH3 = h3,
          oldH4 = h4, oldH5 = h5, oldH6 = h6, oldH7 = h7;
          
    for (j = 0; j < 64; j++) {
      if (j < 16) {
        w[j] = w[j] || 0;
      } else {
        const s0 = rightRotate(w[j - 15], 7) ^ rightRotate(w[j - 15], 18) ^ (w[j - 15] >>> 3);
        const s1 = rightRotate(w[j - 2], 17) ^ rightRotate(w[j - 2], 19) ^ (w[j - 2] >>> 10);
        w[j] = (w[j - 16] + s0 + w[j - 7] + s1) | 0;
      }
      
      const ch = (h4 & h5) ^ (~h4 & h6);
      const maj = (h0 & h1) ^ (h0 & h2) ^ (h1 & h2);
      const temp1 = (h7 + (rightRotate(h4, 6) ^ rightRotate(h4, 11) ^ rightRotate(h4, 25)) + ch + k[j] + w[j]) | 0;
      const temp2 = ((rightRotate(h0, 2) ^ rightRotate(h0, 13) ^ rightRotate(h0, 22)) + maj) | 0;
      
      h7 = h6;
      h6 = h5;
      h5 = (h4 + temp1) | 0;
      h4 = h3;
      h3 = h2;
      h2 = h1;
      h1 = h0;
      h0 = (temp1 + temp2) | 0;
    }
    
    h0 = (h0 + oldH0) | 0;
    h1 = (h1 + oldH1) | 0;
    h2 = (h2 + oldH2) | 0;
    h3 = (h3 + oldH3) | 0;
    h4 = (h4 + oldH4) | 0;
    h5 = (h5 + oldH5) | 0;
    h6 = (h6 + oldH6) | 0;
    h7 = (h7 + oldH7) | 0;
  }
  
  const hashWords = [h0, h1, h2, h3, h4, h5, h6, h7];
  let hex = '';
  for (i = 0; i < 8; i++) {
    const val = hashWords[i];
    hex += ((val >>> 24) & 0xff).toString(16).padStart(2, '0') +
           ((val >>> 16) & 0xff).toString(16).padStart(2, '0') +
           ((val >>> 8) & 0xff).toString(16).padStart(2, '0') +
           (val & 0xff).toString(16).padStart(2, '0');
  }
  return hex;
}

// --- Petri Net Presets with Coordinates for Visual Render ---
const PETRI_NET_PRESETS = {
  o2c: {
    name: "Order-to-Cash (Standard Linear)",
    description: "Standard Order-to-Cash process with linear sequences of Register, Approve, Ship, and Invoice transitions.",
    places: [
      { id: "i", name: "Start", x: 80, y: 150 },
      { id: "p1", name: "Reg_Done", x: 240, y: 150 },
      { id: "p2", name: "App_Done", x: 400, y: 150 },
      { id: "p3", name: "Ship_Done", x: 560, y: 150 },
      { id: "o", name: "End", x: 720, y: 150 }
    ],
    transitions: [
      { id: "t_reg", label: "Register", x: 160, y: 150 },
      { id: "t_app", label: "Approve", x: 320, y: 150 },
      { id: "t_shp", label: "Ship", x: 480, y: 150 },
      { id: "t_inv", label: "Invoice", x: 640, y: 150 }
    ],
    arcs: [
      { source: "i", target: "t_reg" },
      { source: "t_reg", target: "p1" },
      { source: "p1", target: "t_app" },
      { source: "t_app", target: "p2" },
      { source: "p2", target: "t_shp" },
      { source: "t_shp", target: "p3" },
      { source: "p3", target: "t_inv" },
      { source: "t_inv", target: "o" }
    ],
    initialMarking: { i: 1 },
    finalMarking: { o: 1 },
    presetTraces: [
      { name: "Fitting Trace", activities: ["Register", "Approve", "Ship", "Invoice"] },
      { name: "Missing Approve (Violation)", activities: ["Register", "Ship", "Invoice"] },
      { name: "Double Approve (Violation)", activities: ["Register", "Approve", "Approve", "Ship", "Invoice"] },
      { name: "Unrecognized Steps (Drift)", activities: ["Register", "Audit", "Approve", "Ship", "Launder", "Invoice"] }
    ]
  },
  loan: {
    name: "Loan Application (AND-Split/Join)",
    description: "Multi-threaded process showing loan application submission, concurrent credit checks and income verification, and a unified decision gating.",
    places: [
      { id: "i", name: "Start", x: 80, y: 150 },
      { id: "p_cre_p", name: "Credit_Pend", x: 280, y: 80 },
      { id: "p_inc_p", name: "Inc_Pend", x: 280, y: 220 },
      { id: "p_cre_d", name: "Credit_Done", x: 480, y: 80 },
      { id: "p_inc_d", name: "Inc_Done", x: 480, y: 220 },
      { id: "o", name: "End", x: 680, y: 150 }
    ],
    transitions: [
      { id: "t_sub", label: "Submit Application", x: 180, y: 150 },
      { id: "t_chk", label: "Check Credit", x: 380, y: 80 },
      { id: "t_ver", label: "Verify Income", x: 380, y: 220 },
      { id: "t_dec", label: "Make Decision", x: 580, y: 150 }
    ],
    arcs: [
      { source: "i", target: "t_sub" },
      { source: "t_sub", target: "p_cre_p" },
      { source: "t_sub", target: "p_inc_p" },
      { source: "p_cre_p", target: "t_chk" },
      { source: "t_chk", target: "p_cre_d" },
      { source: "p_inc_p", target: "t_ver" },
      { source: "t_ver", target: "p_inc_d" },
      { source: "p_cre_d", target: "t_dec" },
      { source: "p_inc_d", target: "t_dec" },
      { source: "t_dec", target: "o" }
    ],
    initialMarking: { i: 1 },
    finalMarking: { o: 1 },
    presetTraces: [
      { name: "Parallel Order 1", activities: ["Submit Application", "Check Credit", "Verify Income", "Make Decision"] },
      { name: "Parallel Order 2", activities: ["Submit Application", "Verify Income", "Check Credit", "Make Decision"] },
      { name: "Skip Credit (Violation)", activities: ["Submit Application", "Verify Income", "Make Decision"] },
      { name: "Premature Decision (Violation)", activities: ["Submit Application", "Make Decision", "Check Credit", "Verify Income"] }
    ]
  },
  ma_claims: {
    name: "M&A Claims Verification (Looping & Choices)",
    description: "M&A transaction validation model with XOR choice (regular approve vs audit pathway) and re-evaluation loops.",
    places: [
      { id: "i", name: "Start", x: 80, y: 150 },
      { id: "p_eval", name: "Eval_Pend", x: 240, y: 150 },
      { id: "p_rev", name: "Rev_Pend", x: 400, y: 150 },
      { id: "p_audit", name: "Audit_Pend", x: 520, y: 230 },
      { id: "o", name: "End", x: 680, y: 150 }
    ],
    transitions: [
      { id: "t_init", label: "Initialize", x: 160, y: 150 },
      { id: "t_eval", label: "Evaluate", x: 320, y: 150 },
      { id: "t_app", label: "Approve", x: 480, y: 150 },
      { id: "t_aud", label: "Trigger Audit", x: 460, y: 230 },
      { id: "t_aud_ok", label: "Pass Audit", x: 340, y: 230 },
      { id: "t_fail", label: "Re-evaluate", x: 320, y: 70 }
    ],
    arcs: [
      { source: "i", target: "t_init" },
      { source: "t_init", target: "p_eval" },
      { source: "p_eval", target: "t_eval" },
      { source: "t_eval", target: "p_rev" },
      { source: "p_rev", target: "t_app" },
      { source: "t_app", target: "o" },
      { source: "p_rev", target: "t_aud" },
      { source: "t_aud", target: "p_audit" },
      { source: "p_audit", target: "t_aud_ok" },
      { source: "t_aud_ok", target: "p_rev" },
      { source: "p_rev", target: "t_fail" },
      { source: "t_fail", target: "p_eval" }
    ],
    initialMarking: { i: 1 },
    finalMarking: { o: 1 },
    presetTraces: [
      { name: "Direct Approval", activities: ["Initialize", "Evaluate", "Approve"] },
      { name: "Re-evaluation Loop", activities: ["Initialize", "Evaluate", "Re-evaluate", "Evaluate", "Approve"] },
      { name: "Audited Approval", activities: ["Initialize", "Evaluate", "Trigger Audit", "Pass Audit", "Approve"] },
      { name: "Bypass Audit (Violation)", activities: ["Initialize", "Evaluate", "Trigger Audit", "Approve"] }
    ]
  }
};

// --- Dashboard State ---
const state = {
  currentModelId: 'o2c',
  currentModel: PETRI_NET_PRESETS.o2c,
  compiledNet: null,
  currentMarking: {},
  ledger: [],
  streamActive: false,
  streamIntervalId: null,
  driftInjected: false,
  fitnessHistory: [],
  avgFitness: 1.0,
  totalTraces: 0,

  // EWMA parameters
  ewmaLambda: 0.15,
  ewmaMean: 0.95,
  ewmaStDev: 0.05,
  ewmaMultiplier: 3.0,
  ewmaHistory: [],
  isDrifting: false
};

// --- DOM elements ---
const DOM = {
  modelSelect: document.getElementById('model-preset-select'),
  modelDesc: document.getElementById('model-description'),
  tracePresetContainer: document.getElementById('trace-preset-container'),
  customTraceInput: document.getElementById('custom-trace-input'),
  btnRunAlignment: document.getElementById('btn-run-alignment'),
  alignResultsSummary: document.getElementById('alignment-results-summary'),
  alignValCost: document.getElementById('align-val-cost'),
  alignValFitness: document.getElementById('align-val-fitness'),
  alignmentGrid: document.getElementById('alignment-grid-visualizer'),
  metricAvgFitness: document.getElementById('metric-avg-fitness'),
  metricTotalTraces: document.getElementById('metric-total-traces'),
  metricDriftStatus: document.getElementById('metric-drift-status'),
  metricBlockHeight: document.getElementById('metric-block-height'),
  btnStreamStart: document.getElementById('btn-stream-start'),
  btnStreamStop: document.getElementById('btn-stream-stop'),
  btnStreamStep: document.getElementById('btn-stream-step'),
  simulationSpeed: document.getElementById('simulation-speed'),
  chkInjectDrift: document.getElementById('chk-inject-drift'),
  btnReplayStep: document.getElementById('btn-replay-step'),
  btnReplayReset: document.getElementById('btn-replay-reset'),
  ledgerContainer: document.getElementById('ledger-container'),
  driftAlertContainer: document.getElementById('drift-alert-container'),
  svgArcs: document.getElementById('svg-arcs'),
  svgPlaces: document.getElementById('svg-places'),
  svgTransitions: document.getElementById('svg-transitions'),
  svgTokens: document.getElementById('svg-tokens'),
  ewmaChart: document.getElementById('ewma-chart')
};

// --- Petri Net Rendering ---
function getCoordinates(nodeId) {
  const place = state.currentModel.places.find(p => p.id === nodeId);
  if (place) return { x: place.x, y: place.y };
  const transition = state.currentModel.transitions.find(t => t.id === nodeId);
  if (transition) return { x: transition.x, y: transition.y };
  return { x: 0, y: 0 };
}

function drawPetriNet() {
  // Clear elements
  DOM.svgArcs.innerHTML = '';
  DOM.svgPlaces.innerHTML = '';
  DOM.svgTransitions.innerHTML = '';
  DOM.svgTokens.innerHTML = '';

  const net = state.currentModel;

  // 1. Draw Arcs
  net.arcs.forEach(arc => {
    const src = getCoordinates(arc.source);
    const dst = getCoordinates(arc.target);

    const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    path.setAttribute('d', `M ${src.x} ${src.y} L ${dst.x} ${dst.y}`);
    path.setAttribute('class', 'arc-path');
    path.setAttribute('id', `arc-${arc.source}-${arc.target}`);
    DOM.svgArcs.appendChild(path);
  });

  // 2. Draw Places
  net.places.forEach(place => {
    const group = document.createElementNS('http://www.w3.org/2000/svg', 'g');

    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
    circle.setAttribute('cx', place.x);
    circle.setAttribute('cy', place.y);
    circle.setAttribute('r', 20);
    circle.setAttribute('class', 'place-node');
    circle.setAttribute('id', `place-${place.id}`);
    group.appendChild(circle);

    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('x', place.x);
    text.setAttribute('y', place.y + 35);
    text.setAttribute('class', 'node-label');
    text.textContent = place.name;
    group.appendChild(text);

    DOM.svgPlaces.appendChild(group);
  });

  // 3. Draw Transitions
  net.transitions.forEach(trans => {
    const group = document.createElementNS('http://www.w3.org/2000/svg', 'g');

    const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
    rect.setAttribute('x', trans.x - 35);
    rect.setAttribute('y', trans.y - 18);
    rect.setAttribute('width', 70);
    rect.setAttribute('height', 36);
    rect.setAttribute('rx', 4);
    rect.setAttribute('class', 'transition-node');
    rect.setAttribute('id', `trans-${trans.id}`);

    // Click handler to manually fire transition if enabled
    rect.addEventListener('click', () => {
      triggerManualFire(trans.id);
    });

    group.appendChild(rect);

    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('x', trans.x);
    text.setAttribute('y', trans.y + 4);
    text.setAttribute('class', 'node-text');
    text.textContent = trans.label || 'tau';
    group.appendChild(text);

    DOM.svgTransitions.appendChild(group);
  });

  updateVisualMarkings();
}

function updateVisualMarkings() {
  const net = state.currentModel;

  // Update places token styles
  net.places.forEach(place => {
    const pEl = document.getElementById(`place-${place.id}`);
    const tokenCount = state.currentMarking[place.id] || 0;
    if (pEl) {
      if (tokenCount > 0) {
        pEl.classList.add('active-token');
      } else {
        pEl.classList.remove('active-token');
      }
    }
  });

  // Draw tokens inside places
  DOM.svgTokens.innerHTML = '';
  net.places.forEach(place => {
    const tokenCount = state.currentMarking[place.id] || 0;
    if (tokenCount === 1) {
      const dot = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
      dot.setAttribute('cx', place.x);
      dot.setAttribute('cy', place.y);
      dot.setAttribute('r', 5);
      dot.setAttribute('class', 'token-dot');
      DOM.svgTokens.appendChild(dot);
    } else if (tokenCount > 1) {
      // Draw multiple tokens or numerical overlay
      const dot = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
      dot.setAttribute('cx', place.x - 4);
      dot.setAttribute('cy', place.y);
      dot.setAttribute('r', 4);
      dot.setAttribute('class', 'token-dot');
      DOM.svgTokens.appendChild(dot);

      const dot2 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
      dot2.setAttribute('cx', place.x + 4);
      dot2.setAttribute('cy', place.y);
      dot2.setAttribute('r', 4);
      dot2.setAttribute('class', 'token-dot');
      DOM.svgTokens.appendChild(dot2);

      const num = document.createElementNS('http://www.w3.org/2000/svg', 'text');
      num.setAttribute('x', place.x);
      num.setAttribute('y', place.y - 8);
      num.setAttribute('style', 'fill:#03dac6; font-size: 9px; font-weight: bold; text-anchor: middle;');
      num.textContent = tokenCount;
      DOM.svgTokens.appendChild(num);
    }
  });

  // Highlight enabled transitions
  state.compiledNet.transitions.forEach(t => {
    const tEl = document.getElementById(`trans-${t.id}`);
    if (tEl) {
      const enabled = AStarAligner.isEnabled(state.currentMarking, t);
      if (enabled) {
        tEl.classList.add('enabled');
      } else {
        tEl.classList.remove('enabled');
      }
    }
  });
}

// --- Animated Token Flow ---
function animateTokenFlow(transitionId, onComplete) {
  const transition = state.compiledNet.transitions.find(t => t.id === transitionId);
  if (!transition) return;

  const duration = 400; // ms
  const startTime = performance.now();

  // Highlight active arcs
  const activeArcs = [];
  Object.keys(transition.preset).forEach(pId => {
    const arcEl = document.getElementById(`arc-${pId}-${transitionId}`);
    if (arcEl) {
      arcEl.classList.add('highlighted');
      activeArcs.push(arcEl);
    }
  });
  Object.keys(transition.postset).forEach(pId => {
    const arcEl = document.getElementById(`arc-${transitionId}-${pId}`);
    if (arcEl) {
      arcEl.classList.add('highlighted');
      activeArcs.push(arcEl);
    }
  });

  // Create temporary tokens for animation
  const animTokens = [];

  // Input to Transition particles
  Object.keys(transition.preset).forEach(pId => {
    const src = getCoordinates(pId);
    const dst = getCoordinates(transitionId);
    const particle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
    particle.setAttribute('r', 5);
    particle.setAttribute('class', 'token-dot');
    DOM.svgTokens.appendChild(particle);
    animTokens.push({ particle, src, dst, phase: 'in' });
  });

  function step(now) {
    const elapsed = now - startTime;
    const progress = Math.min(elapsed / duration, 1.0);

    // Ease-in-out interpolation
    const t = progress < 0.5 ? 2 * progress * progress : -1 + (4 - 2 * progress) * progress;

    animTokens.forEach(tok => {
      let x, y;
      if (tok.phase === 'in') {
        x = tok.src.x + t * (tok.dst.x - tok.src.x);
        y = tok.src.y + t * (tok.dst.y - tok.src.y);
      } else {
        x = tok.src.x + t * (tok.dst.x - tok.src.x);
        y = tok.src.y + t * (tok.dst.y - tok.src.y);
      }
      tok.particle.setAttribute('cx', x);
      tok.particle.setAttribute('cy', y);
    });

    if (progress < 1.0) {
      requestAnimationFrame(step);
    } else {
      // Clear input particles, spawn output particles
      animTokens.forEach(tok => tok.particle.remove());
      activeArcs.forEach(arc => arc.classList.remove('highlighted'));

      // Output from Transition particles
      const outTokens = [];
      const outStartTime = performance.now();
      Object.keys(transition.postset).forEach(pId => {
        const src = getCoordinates(transitionId);
        const dst = getCoordinates(pId);
        const particle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
        particle.setAttribute('r', 5);
        particle.setAttribute('class', 'token-dot');
        DOM.svgTokens.appendChild(particle);
        outTokens.push({ particle, src, dst });
      });

      if (outTokens.length === 0) {
        if (onComplete) onComplete();
        return;
      }

      function stepOut(nowOut) {
        const elapsedOut = nowOut - outStartTime;
        const progressOut = Math.min(elapsedOut / duration, 1.0);
        const tOut = progressOut < 0.5 ? 2 * progressOut * progressOut : -1 + (4 - 2 * progressOut) * progressOut;

        outTokens.forEach(tok => {
          const x = tok.src.x + tOut * (tok.dst.x - tok.src.x);
          const y = tok.src.y + tOut * (tok.dst.y - tok.src.y);
          tok.particle.setAttribute('cx', x);
          tok.particle.setAttribute('cy', y);
        });

        if (progressOut < 1.0) {
          requestAnimationFrame(stepOut);
        } else {
          outTokens.forEach(tok => tok.particle.remove());
          if (onComplete) onComplete();
        }
      }
      requestAnimationFrame(stepOut);
    }
  }
  requestAnimationFrame(step);
}

// --- Manual token play ---
function triggerManualFire(transitionId) {
  const transition = state.compiledNet.transitions.find(t => t.id === transitionId);
  if (!transition) return;

  if (AStarAligner.isEnabled(state.currentMarking, transition)) {
    const initMarking = { ...state.currentMarking };
    animateTokenFlow(transitionId, () => {
      state.currentMarking = AStarAligner.fire(state.currentMarking, transition);
      updateVisualMarkings();
      appendLedgerBlock(transition.label || 'tau', initMarking, state.currentMarking);
    });
  }
}

// --- Cryptographic Ledger logging ---
function appendLedgerBlock(activity, initialMarking, finalMarking) {
  const prevBlock = state.ledger[state.ledger.length - 1];
  const prevHash = prevBlock ? prevBlock.hash : "0000000000000000000000000000000000000000000000000000000000000000";
  const index = state.ledger.length;
  const timestamp = new Date().toISOString();
  
  const initMarkingStr = AStarAligner.getMarkingKey(initialMarking);
  const finalMarkingStr = AStarAligner.getMarkingKey(finalMarking);

  // Content string to hash
  const payload = index + timestamp + activity + initMarkingStr + finalMarkingStr + prevHash;
  const hash = sha256(payload);

  const block = {
    index,
    timestamp,
    activity,
    initialMarking: initMarkingStr,
    finalMarking: finalMarkingStr,
    prevHash,
    hash,
    isValid: true
  };

  state.ledger.push(block);
  DOM.metricBlockHeight.textContent = state.ledger.length;

  // Append to UI
  const el = document.createElement('div');
  el.className = 'ledger-block';
  el.innerHTML = `
    <div class="ledger-header">
      <span>BLOCK #${index}</span>
      <span class="ledger-badge" id="badge-${index}">VERIFIED</span>
    </div>
    <div style="font-weight:bold; color:#fff;">Event: ${activity}</div>
    <div>Marking: [${initMarkingStr || 'Ø'}] &rarr; [${finalMarkingStr || 'Ø'}]</div>
    <div class="ledger-hash">Hash: ${hash.substring(0, 16)}...</div>
    <div class="ledger-prev-hash">Prev: ${prevHash.substring(0, 16)}...</div>
  `;
  
  DOM.ledgerContainer.insertBefore(el, DOM.ledgerContainer.firstChild);
}

// --- Initialize Preset Model ---
function loadPresetModel(modelId) {
  state.currentModelId = modelId;
  state.currentModel = PETRI_NET_PRESETS[modelId];
  state.compiledNet = AStarAligner.compilePetriNet(state.currentModel);
  state.currentMarking = { ...state.compiledNet.initialMarking };
  
  DOM.modelDesc.textContent = state.currentModel.description;

  // Load preset traces UI
  DOM.tracePresetContainer.innerHTML = '';
  state.currentModel.presetTraces.forEach((t, i) => {
    const pill = document.createElement('div');
    pill.className = `trace-pill ${i === 0 ? 'active' : ''}`;
    pill.textContent = t.name;
    pill.addEventListener('click', () => {
      document.querySelectorAll('.trace-pill').forEach(p => p.classList.remove('active'));
      pill.classList.add('active');
      DOM.customTraceInput.value = t.activities.join(', ');
    });
    DOM.tracePresetContainer.appendChild(pill);
  });

  // Set default custom trace input value
  if (state.currentModel.presetTraces.length > 0) {
    DOM.customTraceInput.value = state.currentModel.presetTraces[0].activities.join(', ');
  }

  // Draw net
  drawPetriNet();
}

// --- Alignment Solver UI Bindings ---
function runAlignmentSolver() {
  const val = DOM.customTraceInput.value;
  if (!val.trim()) return;

  const trace = val.split(',').map(s => s.trim()).filter(s => s.length > 0);

  // Compute fitness and path
  const res = AStarAligner.calculateFitness(trace, state.currentModel);

  DOM.alignResultsSummary.style.display = 'grid';
  DOM.alignValCost.textContent = res.cost;
  DOM.alignValFitness.textContent = res.fitness.toFixed(3);

  // Render alignment table
  DOM.alignmentGrid.innerHTML = `
    <div class="alignment-row" style="margin-bottom:6px;">
      <div class="alignment-cell alignment-header-cell">Step</div>
      <div class="alignment-cell alignment-header-cell log-cell">Log Activity</div>
      <div class="alignment-cell alignment-header-cell model-cell">Model Action</div>
      <div class="alignment-cell alignment-header-cell cost-cell">Cost</div>
    </div>
  `;

  if (res.alignment) {
    res.alignment.forEach((step, idx) => {
      const row = document.createElement('div');
      row.className = `alignment-row ${step.type.replace('_', '-')}`;
      row.innerHTML = `
        <div class="alignment-cell" style="color:var(--text-muted); max-width:45px;">${idx + 1}</div>
        <div class="alignment-cell log-cell">${step.logActivity}</div>
        <div class="alignment-cell model-cell">${step.modelTransition} (${step.activity || 'tau'})</div>
        <div class="alignment-cell cost-cell">${step.cost}</div>
      `;
      DOM.alignmentGrid.appendChild(row);
    });
  } else {
    const errorEl = document.createElement('div');
    errorEl.style.color = 'var(--accent-red)';
    errorEl.style.padding = '8px';
    errorEl.style.textAlign = 'center';
    errorEl.textContent = res.error || 'Alignment solver failed.';
    DOM.alignmentGrid.appendChild(errorEl);
  }
}

// --- EWMA Chart Rendering on HTML5 Canvas ---
function drawEWMAChart() {
  const canvas = DOM.ewmaChart;
  // Get computed styles for scaling
  const rect = canvas.parentNode.getBoundingClientRect();
  canvas.width = rect.width * window.devicePixelRatio;
  canvas.height = rect.height * window.devicePixelRatio;
  
  const ctx = canvas.getContext('2d');
  ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

  const w = rect.width;
  const h = rect.height;

  // Clear canvas
  ctx.fillStyle = '#0f172a';
  ctx.fillRect(0, 0, w, h);

  // Margins
  const m = { left: 45, right: 20, top: 20, bottom: 35 };
  const graphW = w - m.left - m.right;
  const graphH = h - m.top - m.bottom;

  // Draw background grid lines
  ctx.strokeStyle = 'rgba(255,255,255,0.05)';
  ctx.lineWidth = 1;
  for (let i = 0; i <= 5; i++) {
    const yVal = m.top + (i / 5) * graphH;
    ctx.beginPath();
    ctx.moveTo(m.left, yVal);
    ctx.lineTo(w - m.right, yVal);
    ctx.stroke();

    // Labels
    ctx.fillStyle = '#64748b';
    ctx.font = '9px sans-serif';
    ctx.fillText((1.0 - (i / 5)).toFixed(2), m.left - 30, yVal + 3);
  }

  // Draw horizontal Target/Mean fitness line
  ctx.strokeStyle = 'rgba(148, 163, 184, 0.4)';
  ctx.setLineDash([4, 4]);
  ctx.beginPath();
  const targetY = m.top + (1.0 - state.ewmaMean) * graphH;
  ctx.moveTo(m.left, targetY);
  ctx.lineTo(w - m.right, targetY);
  ctx.stroke();
  ctx.setLineDash([]);
  ctx.fillText('Target (0.95)', w - m.right - 60, targetY - 4);

  // Plot EWMA points and LCL (Lower Control Limit)
  const history = state.ewmaHistory;
  if (history.length === 0) return;

  const pointsCount = Math.max(25, history.length);
  const getX = (idx) => m.left + (idx / (pointsCount - 1)) * graphW;
  const getY = (val) => m.top + (1.0 - val) * graphH;

  // Plot LCL boundary curve
  ctx.strokeStyle = 'var(--accent-red)';
  ctx.lineWidth = 1.5;
  ctx.setLineDash([2, 2]);
  ctx.beginPath();
  history.forEach((pt, idx) => {
    const x = getX(idx);
    const y = getY(pt.lcl);
    if (idx === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.stroke();
  ctx.setLineDash([]);
  ctx.fillStyle = 'var(--accent-red)';
  ctx.fillText('LCL', w - m.right - 25, getY(history[history.length - 1].lcl) - 4);

  // Plot individual fitness points
  history.forEach((pt, idx) => {
    ctx.beginPath();
    ctx.arc(getX(idx), getY(pt.fitness), 3, 0, 2 * Math.PI);
    ctx.fillStyle = pt.fitness >= pt.lcl ? 'var(--accent-green)' : 'var(--accent-red)';
    ctx.fill();
  });

  // Plot EWMA smoothed line
  ctx.strokeStyle = 'var(--accent-cyan)';
  ctx.lineWidth = 2.5;
  ctx.shadowColor = 'var(--accent-cyan-glow)';
  ctx.shadowBlur = 8;
  ctx.beginPath();
  history.forEach((pt, idx) => {
    const x = getX(idx);
    const y = getY(pt.ewma);
    if (idx === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.stroke();
  ctx.shadowBlur = 0; // Reset shadow

  // Mark drift violation highlights
  history.forEach((pt, idx) => {
    if (pt.ewma < pt.lcl) {
      ctx.beginPath();
      ctx.arc(getX(idx), getY(pt.ewma), 6, 0, 2 * Math.PI);
      ctx.strokeStyle = 'var(--accent-red)';
      ctx.lineWidth = 2;
      ctx.stroke();
    }
  });

  // X-axis label
  ctx.fillStyle = '#8a99ad';
  ctx.fillText('Historical Traces Stream (Timeline)', w / 2 - 40, h - 8);
}

// --- EWMA Calculation ---
function appendEWMAPoint(fitness) {
  const history = state.ewmaHistory;
  const t = history.length + 1;

  // Calculate moving average
  let prevEWMA = t === 1 ? fitness : history[history.length - 1].ewma;
  let ewma = state.ewmaLambda * fitness + (1 - state.ewmaLambda) * prevEWMA;

  // Calculate dynamic control limits based on sample size t
  // Formula: LCL = Mean - L * StDev * sqrt( (lambda / (2-lambda)) * (1 - (1-lambda)^(2t)) )
  const term1 = state.ewmaLambda / (2.0 - state.ewmaLambda);
  const term2 = 1.0 - Math.pow(1.0 - state.ewmaLambda, 2 * t);
  const limitStDev = state.ewmaStDev * Math.sqrt(term1 * term2);
  const lcl = Math.max(0, state.ewmaMean - state.ewmaMultiplier * limitStDev);

  const isDrift = ewma < lcl;

  history.push({
    t,
    fitness,
    ewma,
    lcl,
    isDrift
  });

  // Slide windows to fit inside graph width gracefully
  if (history.length > 50) {
    history.shift();
  }

  // Update global drift status
  state.isDrifting = isDrift;
  if (isDrift) {
    DOM.metricDriftStatus.textContent = 'DRIFT DETECTED';
    DOM.metricDriftStatus.className = 'metric-val red';
    DOM.driftAlertContainer.style.display = 'block';
  } else {
    DOM.metricDriftStatus.textContent = 'STABLE';
    DOM.metricDriftStatus.className = 'metric-val green';
    DOM.driftAlertContainer.style.display = 'none';
  }

  // Recalculate average fitness
  state.totalTraces++;
  state.avgFitness = ((state.avgFitness * (state.totalTraces - 1)) + fitness) / state.totalTraces;
  
  DOM.metricAvgFitness.textContent = state.avgFitness.toFixed(3);
  DOM.metricTotalTraces.textContent = state.totalTraces;

  drawEWMAChart();
}

// --- Automated Trace Stream Simulator ---
function simulateTraceStreamStep() {
  const modelId = state.currentModelId;
  const net = state.currentModel;
  let activities = [];

  if (state.driftInjected) {
    // Generate faulty/drifted traces
    if (modelId === 'o2c') {
      // Out of order or bypasses approve
      activities = Math.random() > 0.5
        ? ["Register", "Ship", "Invoice"] // Bypass Approve
        : ["Register", "Approve", "Approve", "Invoice"]; // Double Approve, missing Ship
    } else if (modelId === 'loan') {
      activities = Math.random() > 0.5
        ? ["Submit Application", "Make Decision"] // Skip concurrent checks
        : ["Submit Application", "Check Credit", "Make Decision"]; // Skip Income verification
    } else {
      activities = ["Initialize", "Trigger Audit", "Approve"]; // Bypasses Evaluate
    }
  } else {
    // Generate conforming traces
    if (modelId === 'o2c') {
      activities = ["Register", "Approve", "Ship", "Invoice"];
    } else if (modelId === 'loan') {
      activities = Math.random() > 0.5
        ? ["Submit Application", "Check Credit", "Verify Income", "Make Decision"]
        : ["Submit Application", "Verify Income", "Check Credit", "Make Decision"];
    } else {
      activities = Math.random() > 0.5
        ? ["Initialize", "Evaluate", "Approve"]
        : ["Initialize", "Evaluate", "Trigger Audit", "Pass Audit", "Approve"];
    }

    // Add slight random noise (95% conforming, 5% minor alignment cost)
    if (Math.random() > 0.95) {
      activities.splice(Math.floor(Math.random() * activities.length), 0, "Inspect");
    }
  }

  // Compute fitness of the simulated trace
  const res = AStarAligner.calculateFitness(activities, net);
  appendEWMAPoint(res.fitness);

  // Replay this step's trace on the visual Petri net
  replaySimulatedTraceOnNet(activities);
}

function replaySimulatedTraceOnNet(activities) {
  // Clear marking to initial state
  state.currentMarking = { ...state.compiledNet.initialMarking };
  updateVisualMarkings();

  // Run a visual trace playback (we sequentially fire enabled transitions if match)
  let stepIdx = 0;
  
  function triggerNextReplayStep() {
    if (stepIdx >= activities.length) return;
    const act = activities[stepIdx];
    
    // Find transition corresponding to this activity that is enabled
    const enabledTrans = state.compiledNet.transitions.find(t => 
      t.label === act && AStarAligner.isEnabled(state.currentMarking, t)
    );

    if (enabledTrans) {
      const initMarking = { ...state.currentMarking };
      animateTokenFlow(enabledTrans.id, () => {
        state.currentMarking = AStarAligner.fire(state.currentMarking, enabledTrans);
        updateVisualMarkings();
        appendLedgerBlock(enabledTrans.label, initMarking, state.currentMarking);
        stepIdx++;
        setTimeout(triggerNextReplayStep, 250);
      });
    } else {
      // If not enabled, it's a conformance violation! Log-only or model-only move bypasses
      // We log to ledger as bypass/error
      const errorLabel = `${act} (Violation)`;
      appendLedgerBlock(errorLabel, state.currentMarking, state.currentMarking);
      stepIdx++;
      setTimeout(triggerNextReplayStep, 250);
    }
  }

  triggerNextReplayStep();
}

// --- Setup Event Listeners ---
function setupEventListeners() {
  DOM.modelSelect.addEventListener('change', (e) => {
    loadPresetModel(e.target.value);
  });

  DOM.btnRunAlignment.addEventListener('click', runAlignmentSolver);

  // Replay Reset
  DOM.btnReplayReset.addEventListener('click', () => {
    state.currentMarking = { ...state.compiledNet.initialMarking };
    updateVisualMarkings();
  });

  // Replay Step manually (performs A* step visually)
  DOM.btnReplayStep.addEventListener('click', () => {
    const val = DOM.customTraceInput.value;
    if (!val) return;
    const trace = val.split(',').map(s => s.trim());
    replaySimulatedTraceOnNet(trace);
  });

  // Stream controls
  DOM.btnStreamStart.addEventListener('click', () => {
    if (state.streamActive) return;
    state.streamActive = true;
    DOM.btnStreamStart.disabled = true;
    DOM.btnStreamStop.disabled = false;
    
    const intervalSec = parseFloat(DOM.simulationSpeed.value) * 1000;
    state.streamIntervalId = setInterval(simulateTraceStreamStep, intervalSec);
  });

  DOM.btnStreamStop.addEventListener('click', () => {
    if (!state.streamActive) return;
    state.streamActive = false;
    DOM.btnStreamStart.disabled = false;
    DOM.btnStreamStop.disabled = true;
    clearInterval(state.streamIntervalId);
  });

  DOM.btnStreamStep.addEventListener('click', simulateTraceStreamStep);

  DOM.simulationSpeed.addEventListener('input', (e) => {
    if (state.streamActive) {
      // Restart interval with new speed
      clearInterval(state.streamIntervalId);
      const intervalSec = parseFloat(e.target.value) * 1000;
      state.streamIntervalId = setInterval(simulateTraceStreamStep, intervalSec);
    }
  });

  DOM.chkInjectDrift.addEventListener('change', (e) => {
    state.driftInjected = e.target.checked;
  });

  window.addEventListener('resize', drawEWMAChart);
}

// --- Application Init ---
function init() {
  setupEventListeners();
  loadPresetModel('o2c');
  drawEWMAChart();
}

window.addEventListener('DOMContentLoaded', init);
