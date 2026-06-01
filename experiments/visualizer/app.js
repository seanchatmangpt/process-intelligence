/**
 * app.js
 * Process Intelligence Simulation Dashboard - Core Controller & Visual Binder
 * Conforms to Dr. Wil van der Aalst's Process Mining & Streaming Telemetry Standards.
 * 
 * Binds:
 * - Dynamic Petri net SVG token game player & animation
 * - Priority-queue based A* alignment solver
 * - LTL Declare constraint verification engine
 * - EWMA process drift calculator & Canvas chart renderer
 * - Cryptographic process ledger with SHA-256 chaining and interactive tampering
 * - M&A Diligence Claims board verification bridge
 * - Real-time Stream Director Telemetry HUD (FPS, Latency < 200ms, Frame Drops)
 */

// --- 1. Petri Net Model and Layout Coordinates ---
const net = {
    places: {
        p_start: { name: "Start", x: 80, y: 150 },
        p_created: { name: "Created", x: 240, y: 150 },
        p_received: { name: "Received", x: 400, y: 150 },
        p_audited: { name: "Audited", x: 560, y: 150 },
        p_approved: { name: "Approved", x: 720, y: 150 },
        p_end: { name: "End", x: 880, y: 150 }
    },
    transitions: {
        t_create: { label: "Create_Order", inputs: ["p_start"], outputs: ["p_created"], x: 160, y: 150 },
        t_receive: { label: "Receive_Goods", inputs: ["p_created"], outputs: ["p_received"], x: 320, y: 150 },
        t_audit: { label: "Audit_Invoice", inputs: ["p_received"], outputs: ["p_audited"], x: 480, y: 150 },
        t_approve: { label: "Approve_Payment", inputs: ["p_audited"], outputs: ["p_approved"], x: 640, y: 150 },
        t_close: { label: "Close_Case", inputs: ["p_approved"], outputs: ["p_end"], x: 800, y: 150 }
    }
};

// --- 2. State & Statistics ---
let isRunning = false;
let playbackTimeout = null;
let speed = 1000; // ms per event step

let currentTrace = null;
let currentEventIndex = 0;
let currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };

let totalTraces = 0;
let totalEvents = 0;
let totalViolations = 0;
let traceCounter = 0;

let activeInspectedTrace = null; // Holds the trace selected for step-replay

// Drift and Chart Tracking
const history = []; // Holds { fitness, ewma, traceId }
const lambda = 0.2; // EWMA smoothing factor
const lcl = 0.92;   // Lower Control Limit for drift
let isViolationModeActive = false;

// --- 2.1 Update Global Metrics in UI ---
function updateGlobalMetrics() {
    const tracesEl = document.getElementById("statTraces");
    const eventsEl = document.getElementById("statEvents");
    const fitnessEl = document.getElementById("statFitness");
    const violationsEl = document.getElementById("statViolations");

    if (tracesEl) tracesEl.textContent = totalTraces;
    if (eventsEl) eventsEl.textContent = totalEvents;
    if (violationsEl) violationsEl.textContent = totalViolations;

    if (fitnessEl) {
        const avgFitness = history.length > 0 
            ? (history.reduce((sum, h) => sum + h.fitness, 0) / history.length) 
            : 1.0;
        fitnessEl.textContent = (avgFitness * 100).toFixed(1) + "%";
        
        if (avgFitness >= 0.95) {
            fitnessEl.className = "metric-val text-success";
        } else if (avgFitness >= 0.85) {
            fitnessEl.className = "metric-val text-warning";
        } else {
            fitnessEl.className = "metric-val text-danger";
        }
    }
}

// Cryptographic Ledger Tracking
const ledger = [];
let isLedgerIntact = true;

// DECLARE Rules Setup
const declareRules = [
    "Precedence(Audit_Invoice, Approve_Payment)",
    "Response(Create_Order, Close_Case)",
    "NotCoExistence(Create_Order, Raw_Laundering)"
];

const declareRuleStats = {
    "Precedence(Audit_Invoice, Approve_Payment)": { activations: 0, satisfactions: 0, violations: 0, status: "PENDING" },
    "Response(Create_Order, Close_Case)": { activations: 0, satisfactions: 0, violations: 0, status: "PENDING" },
    "NotCoExistence(Create_Order, Raw_Laundering)": { activations: 0, satisfactions: 0, violations: 0, status: "PENDING" }
};

// M&A Diligence Claims Setup
const maClaims = [
    {
        id: "synergy_procure_to_pay_001",
        slideUuid: "slide-synergy-p2p-7281",
        category: "synergy",
        title: "Procurement Consolidation Synergy",
        description: "Seller assertion of $5M in annual cost savings due to unified purchasing process.",
        valueUsd: 5000000,
        verify: () => {
            const recent = history.slice(-10);
            if (recent.length === 0) return { status: "PENDING", details: "Awaiting simulation data..." };
            const avg = recent.reduce((sum, r) => sum + r.fitness, 0) / recent.length;
            const ok = avg >= 0.95;
            return {
                status: ok ? "DEFENSIBLE" : "REJECTED",
                details: `Avg Fitness: ${avg.toFixed(3)} (Threshold: &ge; 0.950, n=${recent.length})`
            };
        }
    },
    {
        id: "liability_audit_enforcement_002",
        slideUuid: "slide-compliance-aud-9410",
        category: "liability",
        title: "Regulatory Compliance Enforcement",
        description: "Seller assertion of zero un-audited invoice approvals (regulatory risk mitigation).",
        valueUsd: 3500000,
        verify: () => {
            const rule = "Precedence(Audit_Invoice, Approve_Payment)";
            const stats = declareRuleStats[rule];
            if (!stats || (stats.activations === 0 && stats.violations === 0)) {
                return { status: "PENDING", details: "Awaiting audit validation..." };
            }
            const ok = stats.violations === 0;
            return {
                status: ok ? "DEFENSIBLE" : "REJECTED",
                details: `Violations detected: ${stats.violations} (Expected: 0)`
            };
        }
    },
    {
        id: "integration_ledger_lineage_003",
        slideUuid: "slide-lineage-prov-3011",
        category: "integration_risk",
        title: "Data Room Trail Lineage",
        description: "Buyer condition: 100% cryptographic ledger audit trails for data room transaction files.",
        valueUsd: 1200000,
        verify: () => {
            if (ledger.length < 3) {
                return { status: "PENDING", details: `Ledger blocks: ${ledger.length}/3 (Awaiting size requirement)` };
            }
            return {
                status: isLedgerIntact ? "DEFENSIBLE" : "REJECTED",
                details: isLedgerIntact ? `Ledger verified. Blocks: ${ledger.length}. No tampering.` : "Ledger signature mismatch / tampered!"
            };
        }
    }
];

// --- 3. Cryptographic Helper (SHA-256) ---
async function calculateSHA256(message) {
    const msgBuffer = new TextEncoder().encode(message);
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

// --- 4. Dynamic Petri Net SVG Renderer ---
function drawPetriNetSvg() {
    const svg = document.getElementById("petriNetSvg");
    if (!svg) return;
    svg.innerHTML = "";
    
    // SVG definitions for arrow marker
    const defs = document.createElementNS("http://www.w3.org/2000/svg", "defs");
    const marker = document.createElementNS("http://www.w3.org/2000/svg", "marker");
    marker.setAttribute("id", "arrow");
    marker.setAttribute("viewBox", "0 0 10 10");
    marker.setAttribute("refX", "22"); // Matches place radius plus offsets
    marker.setAttribute("refY", "5");
    marker.setAttribute("markerWidth", "6");
    marker.setAttribute("markerHeight", "6");
    marker.setAttribute("orient", "auto-start-reverse");
    const markerPath = document.createElementNS("http://www.w3.org/2000/svg", "path");
    markerPath.setAttribute("d", "M 0 2 L 8 5 L 0 8 z");
    markerPath.setAttribute("fill", "#64748b");
    marker.appendChild(markerPath);
    defs.appendChild(marker);
    svg.appendChild(defs);

    // Draw Arcs
    for (const tId in net.transitions) {
        const trans = net.transitions[tId];
        // Incoming arcs (Places -> Transition)
        trans.inputs.forEach(inp => {
            const place = net.places[inp];
            const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
            path.setAttribute("d", `M ${place.x} ${place.y} L ${trans.x} ${trans.y}`);
            path.setAttribute("class", "petri-arc");
            svg.appendChild(path);
        });
        // Outgoing arcs (Transition -> Places) with marker
        trans.outputs.forEach(out => {
            const place = net.places[out];
            const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
            path.setAttribute("d", `M ${trans.x} ${trans.y} L ${place.x} ${place.y}`);
            path.setAttribute("class", "petri-arc");
            path.setAttribute("marker-end", "url(#arrow)");
            svg.appendChild(path);
        });
    }

    // Draw Transitions
    for (const tId in net.transitions) {
        const trans = net.transitions[tId];
        const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
        rect.setAttribute("id", `trans_${tId}`);
        rect.setAttribute("x", trans.x - 15);
        rect.setAttribute("y", trans.y - 20);
        rect.setAttribute("width", "30");
        rect.setAttribute("height", "40");
        rect.setAttribute("rx", "4");
        rect.setAttribute("class", "petri-trans");
        svg.appendChild(rect);

        // Label
        const label = document.createElementNS("http://www.w3.org/2000/svg", "text");
        label.setAttribute("x", trans.x);
        label.setAttribute("y", trans.y + 35);
        label.setAttribute("class", "trans-label");
        label.textContent = trans.label;
        svg.appendChild(label);
    }

    // Draw Places
    for (const pId in net.places) {
        const place = net.places[pId];
        const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
        circle.setAttribute("id", `place_${pId}`);
        circle.setAttribute("cx", place.x);
        circle.setAttribute("cy", place.y);
        circle.setAttribute("r", "18");
        circle.setAttribute("class", "petri-place");
        svg.appendChild(circle);

        // Token count text
        const text = document.createElementNS("http://www.w3.org/2000/svg", "text");
        text.setAttribute("id", `place_${pId}_tokens`);
        text.setAttribute("x", place.x);
        text.setAttribute("y", place.y + 5);
        text.setAttribute("class", "place-token-text");
        text.textContent = "0";
        svg.appendChild(text);

        // Label
        const label = document.createElementNS("http://www.w3.org/2000/svg", "text");
        label.setAttribute("x", place.x);
        label.setAttribute("y", place.y - 25);
        label.setAttribute("class", "place-label");
        label.textContent = place.name;
        svg.appendChild(label);
    }

    // Draw replayToken for animation
    const replayToken = document.createElementNS("http://www.w3.org/2000/svg", "circle");
    replayToken.setAttribute("id", "replayToken");
    replayToken.setAttribute("r", "6");
    replayToken.setAttribute("fill", "var(--color-warning)");
    replayToken.style.display = "none";
    svg.appendChild(replayToken);
    
    updateLabelsAndHighlights();
}

function updateLabelsAndHighlights() {
    for (const pId in net.places) {
        const text = document.getElementById(`place_${pId}_tokens`);
        const circle = document.getElementById(`place_${pId}`);
        const tokenCount = currentMarking[pId] || 0;
        
        if (text) text.textContent = tokenCount;
        if (circle) {
            if (tokenCount >= 1) {
                circle.classList.add("active");
            } else {
                circle.classList.remove("active");
            }
        }
    }
    
    // Highlight enabled transitions
    for (const tId in net.transitions) {
        const trans = net.transitions[tId];
        const rect = document.getElementById(`trans_${tId}`);
        if (!rect) continue;
        
        if (isTransitionEnabled(trans, currentMarking)) {
            rect.classList.add("enabled");
        } else {
            rect.classList.remove("enabled");
        }
    }
}

// --- 5. Token Game Replay SVG Animation ---
function animateToken(fromX, fromY, midX, midY, toX, toY, duration, callback) {
    const token = document.getElementById("replayToken");
    if (!token) {
        callback();
        return;
    }
    
    token.setAttribute("cx", fromX);
    token.setAttribute("cy", fromY);
    token.style.display = "block";
    
    const startTime = performance.now();
    
    function update(time) {
        const elapsed = time - startTime;
        const progress = Math.min(elapsed / duration, 1.0);
        
        let x, y;
        if (progress < 0.5) {
            // Stage 1: Place to Transition
            const p = progress * 2;
            x = fromX + (midX - fromX) * p;
            y = fromY + (midY - fromY) * p;
        } else {
            // Stage 2: Transition to Place
            const p = (progress - 0.5) * 2;
            x = midX + (toX - midX) * p;
            y = midY + (toY - midY) * p;
        }
        
        token.setAttribute("cx", x);
        token.setAttribute("cy", y);
        
        if (progress < 1.0) {
            requestAnimationFrame(update);
        } else {
            token.style.display = "none";
            callback();
        }
    }
    
    requestAnimationFrame(update);
}

// --- 6. A* Alignment Solver Logic ---
class AlignmentState {
    constructor(marking, traceIndex, g, h, parent = null, move = null) {
        this.marking = { ...marking };
        this.traceIndex = traceIndex;
        this.g = g;
        this.h = h;
        this.f = g + h;
        this.parent = parent;
        this.move = move;
    }
    
    getKey() {
        const markingKey = Object.keys(this.marking)
            .sort()
            .map(p => `${p}:${this.marking[p]}`)
            .join(",");
        return `${this.traceIndex}|${markingKey}`;
    }
}

const placeDistances = {
    p_start: 5,
    p_created: 4,
    p_received: 3,
    p_audited: 2,
    p_approved: 1,
    p_end: 0
};

function getHeuristic(marking, traceIndex, traceLength) {
    let d = 0;
    for (const p in marking) {
        if (marking[p] > 0) {
            d += marking[p] * (placeDistances[p] || 0);
        }
    }
    const remLog = traceLength - traceIndex;
    return Math.abs(d - remLog);
}

function isTransitionEnabled(trans, marking) {
    for (const inp of trans.inputs) {
        if ((marking[inp] || 0) < 1) return false;
    }
    return true;
}

function fireTransition(trans, marking) {
    const nextMarking = { ...marking };
    for (const inp of trans.inputs) {
        nextMarking[inp] = (nextMarking[inp] || 0) - 1;
    }
    for (const out of trans.outputs) {
        nextMarking[out] = (nextMarking[out] || 0) + 1;
    }
    return nextMarking;
}

function solveAlignment(trace) {
    const traceLength = trace.length;
    const initialMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };
    
    const startState = new AlignmentState(
        initialMarking,
        0,
        0,
        getHeuristic(initialMarking, 0, traceLength)
    );
    
    const openList = [startState];
    const closedSet = new Set();
    let expandedCount = 0;
    
    while (openList.length > 0) {
        openList.sort((a, b) => a.f - b.f || a.g - b.g);
        const curr = openList.shift();
        
        expandedCount++;
        
        // Final state: sink place p_end contains 1 token, all others empty
        const isGoalMarking = curr.marking.p_end === 1 && 
            Object.keys(curr.marking).every(p => p === 'p_end' || curr.marking[p] === 0);
        
        if (isGoalMarking && curr.traceIndex === traceLength) {
            const path = [];
            let state = curr;
            while (state.parent !== null) {
                path.unshift(state.move);
                state = state.parent;
            }
            const worstCost = traceLength + 5;
            const fitness = 1 - curr.g / worstCost;
            return {
                alignment: path,
                expandedCount,
                queueSize: openList.length,
                fitness: Math.max(0, fitness)
            };
        }
        
        const key = curr.getKey();
        if (closedSet.has(key)) continue;
        closedSet.add(key);
        
        // 1. Synchronous Moves
        if (curr.traceIndex < traceLength) {
            const currentEvent = trace[curr.traceIndex];
            for (const tId in net.transitions) {
                const trans = net.transitions[tId];
                if (trans.label === currentEvent && isTransitionEnabled(trans, curr.marking)) {
                    const nextMarking = fireTransition(trans, curr.marking);
                    const h = getHeuristic(nextMarking, curr.traceIndex + 1, traceLength);
                    const nextState = new AlignmentState(
                        nextMarking,
                        curr.traceIndex + 1,
                        curr.g, // Sync move cost = 0
                        h,
                        curr,
                        { type: 'sync', transition: tId, label: trans.label, eventIndex: curr.traceIndex }
                    );
                    openList.push(nextState);
                }
            }
        }
        
        // 2. Moves on Model
        for (const tId in net.transitions) {
            const trans = net.transitions[tId];
            if (isTransitionEnabled(trans, curr.marking)) {
                const nextMarking = fireTransition(trans, curr.marking);
                const h = getHeuristic(nextMarking, curr.traceIndex, traceLength);
                const nextState = new AlignmentState(
                    nextMarking,
                    curr.traceIndex,
                    curr.g + 1, // Model move cost = 1
                    h,
                    curr,
                    { type: 'model', transition: tId, label: trans.label }
                );
                openList.push(nextState);
            }
        }
        
        // 3. Moves on Log
        if (curr.traceIndex < traceLength) {
            const currentEvent = trace[curr.traceIndex];
            const h = getHeuristic(curr.marking, curr.traceIndex + 1, traceLength);
            const nextState = new AlignmentState(
                curr.marking,
                curr.traceIndex + 1,
                curr.g + 1, // Log move cost = 1
                h,
                curr,
                { type: 'log', label: currentEvent, eventIndex: curr.traceIndex }
            );
            openList.push(nextState);
        }
    }
    
    return { alignment: [], expandedCount, queueSize: 0, fitness: 0 };
}

// --- 7. Simulated Log Stream Generator ---
function generateTrace() {
    traceCounter++;
    const traceId = `C-${1000 + traceCounter}`;
    const rand = Math.random();
    
    let activities = [];
    let isIntendedViolation = false;
    
    // Check drift toggle and type selector
    const driftToggle = document.getElementById("driftToggle");
    const driftTypeSelector = document.getElementById("driftTypeSelector");
    
    const activeDrift = (driftToggle && driftToggle.checked);
    const driftProfile = (driftTypeSelector ? driftTypeSelector.value : "skip_approve");

    if (activeDrift && driftProfile === "skip_approve") {
        // Control-Flow Drift: Skip Approval Payment transition
        activities = ["Create_Order", "Receive_Goods", "Audit_Invoice", "Close_Case"];
        isIntendedViolation = true;
    } else {
        if (rand < 0.88) {
            // Normal fully compliant sequence
            activities = ["Create_Order", "Receive_Goods", "Audit_Invoice", "Approve_Payment", "Close_Case"];
        } else if (rand < 0.94) {
            // Vacuous satisfaction / Incomplete trace
            activities = ["Create_Order", "Receive_Goods"];
        } else {
            // Spontaneous violation: Bypassed invoice audit transition
            activities = ["Create_Order", "Receive_Goods", "Approve_Payment", "Close_Case"];
            isIntendedViolation = true;
        }
    }
    
    return {
        traceId,
        activities,
        isIntendedViolation
    };
}

function setupNextTrace() {
    currentTrace = generateTrace();
    currentEventIndex = 0;
    
    currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };
    updateLabelsAndHighlights();
}

// --- 8. LTL DECLARE Rules Monitor Renderer ---
function renderDeclareMonitor() {
    const container = document.getElementById("declareRulesContainer");
    if (!container) return;
    
    container.innerHTML = "";
    
    for (const ruleStr in declareRuleStats) {
        const stats = declareRuleStats[ruleStr];
        const card = document.createElement("div");
        card.className = "glass-card";
        card.style.padding = "10px";
        card.style.display = "flex";
        card.style.flexDirection = "column";
        card.style.gap = "4px";
        card.style.backgroundColor = "var(--bg-card)";
        card.style.border = "1px solid var(--border-color)";
        card.style.borderRadius = "6px";
        
        let statusText = stats.status;
        let statusClass = "cyan";
        if (stats.status === "VIOLATED") {
            statusClass = "crimson";
        } else if (stats.status === "FULFILLED") {
            if (stats.activations === 0) {
                statusText = "VACUOUS";
                statusClass = "cyan";
            } else {
                statusClass = "emerald";
            }
        } else if (stats.status === "PENDING") {
            statusClass = "amber";
        }
        
        card.innerHTML = `
            <div style="display: flex; justify-content: space-between; align-items: center;">
                <span class="font-mono text-primary" style="font-weight: 600; font-size: 0.7rem; color: var(--color-primary);">${ruleStr}</span>
                <span class="status-badge ${statusClass}" style="font-size: 0.65rem; padding: 1px 4px; border-radius: 4px;">${statusText}</span>
            </div>
            <div style="display: flex; justify-content: space-between; font-size: 0.65rem; color: var(--color-text-muted); margin-top: 4px;">
                <span>Activations: <strong style="color: var(--color-text-main); font-family: var(--font-mono);">${stats.activations}</strong></span>
                <span>Satisfied: <strong style="color: var(--color-text-main); font-family: var(--font-mono);">${stats.satisfactions}</strong></span>
                <span>Violations: <strong style="color: var(--color-text-main); font-family: var(--font-mono);">${stats.violations}</strong></span>
            </div>
        `;
        container.appendChild(card);
    }
}

// --- 9. Cryptographic Chained Ledger Logic ---
async function appendToLedger(traceId, fitness) {
    const prevBlock = ledger[ledger.length - 1];
    const prevHash = prevBlock ? prevBlock.hash : "0000000000000000000000000000000000000000000000000000000000000000";
    const timestamp = new Date().toISOString();
    const blockId = ledger.length + 1;
    
    // Block chaining formula: Hash = SHA-256(BlockId || Timestamp || TraceID || Fitness || PrevHash)
    const payload = `${blockId}|${timestamp}|${traceId}|${fitness.toFixed(4)}|${prevHash}`;
    const hash = await calculateSHA256(payload);
    
    const block = {
        blockId,
        timestamp,
        traceId,
        fitness,
        prevHash,
        hash
    };
    
    ledger.push(block);
    renderLedger();
}

function renderLedger() {
    const container = document.getElementById("ledgerContainer");
    if (!container) return;
    
    const isIntact = isLedgerIntact;
    const shieldClass = isIntact ? "emerald" : "crimson";
    const shieldText = isIntact ? "INTACT" : "TAMPERED";
    
    container.innerHTML = `
        <div class="panel-title" style="display: flex; justify-content: space-between; align-items: center;">
            <span>Cryptographic Process Ledger (SHA-256 Audit Trail)</span>
            <div style="display: flex; gap: 8px; align-items: center;">
                <span class="status-badge ${shieldClass}" id="ledgerShieldBadge" style="font-weight: 700; padding: 2px 8px; border-radius: 4px; font-size: 0.75rem;">LEDGER: ${shieldText}</span>
            </div>
        </div>
        <div style="display: flex; gap: 12px; overflow-x: auto; padding: 8px 0; margin-top: 10px;" id="ledgerChain">
        </div>
    `;
    
    const chainContainer = document.getElementById("ledgerChain");
    if (!chainContainer) return;
    
    // Render last 4 blocks in reverse order
    const visibleBlocks = ledger.slice().reverse().slice(0, 4);
    
    if (visibleBlocks.length === 0) {
        chainContainer.innerHTML = `<div class="text-center text-muted" style="width: 100%; padding: 20px; font-size: 0.75rem;">Awaiting transactions to write block data...</div>`;
        return;
    }
    
    visibleBlocks.forEach(block => {
        const node = document.createElement("div");
        node.className = "blockchain-node glass-card";
        node.style.flex = "0 0 240px";
        node.style.padding = "10px";
        node.style.border = "1px solid var(--border-color)";
        node.style.borderRadius = "8px";
        node.style.backgroundColor = "var(--bg-card)";
        node.innerHTML = `
            <div style="display: flex; justify-content: space-between; font-weight: 600; font-size: 0.75rem; border-bottom: 1px solid var(--border-color); padding-bottom: 4px; margin-bottom: 6px;">
                <span style="color: var(--color-primary);">Block #${block.blockId}</span>
                <span style="color: var(--color-text-muted); font-size: 0.65rem;">${block.timestamp.slice(11, 19)}</span>
            </div>
            <div style="display: grid; grid-template-columns: 70px 1fr; gap: 4px; font-size: 0.68rem; line-height: 1.3;">
                <span style="color: var(--color-text-muted);">Trace ID:</span>
                <span style="font-family: var(--font-mono); font-weight: 700; color: var(--color-text-main);">${block.traceId}</span>
                
                <span style="color: var(--color-text-muted);">Fitness:</span>
                <span style="font-family: var(--font-mono); font-weight: 700; color: ${block.fitness >= 0.95 ? 'var(--color-success)' : 'var(--color-danger)'}">${(block.fitness * 100).toFixed(0)}%</span>
                
                <span style="color: var(--color-text-muted);">Prev Hash:</span>
                <span style="font-family: var(--font-mono); color: var(--color-text-muted); text-overflow: ellipsis; overflow: hidden; white-space: nowrap;">${block.prevHash.substring(0, 8)}...</span>
                
                <span style="color: var(--color-text-muted); font-weight: 700;">Hash:</span>
                <span style="font-family: var(--font-mono); color: var(--color-primary); font-weight: 700; text-overflow: ellipsis; overflow: hidden; white-space: nowrap;" title="${block.hash}">${block.hash.substring(0, 8)}...</span>
            </div>
            <div style="margin-top: 8px; text-align: right;">
                <button class="btn" onclick="tamperLedger(${block.blockId})" style="font-size: 0.6rem; padding: 2px 6px;">Tamper</button>
            </div>
        `;
        chainContainer.appendChild(node);
    });
}

// Global tampering handler
window.tamperLedger = function(blockId) {
    const block = ledger.find(b => b.blockId === blockId);
    if (!block) return;
    
    const newVal = prompt(`Tampering with Block #${blockId}. Modify Trace ID:`, "trace_TAMPERED");
    if (newVal === null) return;
    
    block.traceId = newVal;
    
    // Re-verify integrity
    verifyLedgerIntegrity().then(isIntact => {
        isLedgerIntact = isIntact;
        renderLedger();
        renderMaClaims(); // Re-verify claims
    });
};

async function verifyLedgerIntegrity() {
    if (ledger.length === 0) return true;
    for (let i = 0; i < ledger.length; i++) {
        const block = ledger[i];
        const prevBlock = ledger[i - 1];
        const expectedPrevHash = prevBlock ? prevBlock.hash : "0000000000000000000000000000000000000000000000000000000000000000";
        
        if (block.prevHash !== expectedPrevHash) return false;
        
        const payload = `${block.blockId}|${block.timestamp}|${block.traceId}|${block.fitness.toFixed(4)}|${block.prevHash}`;
        const recalcHash = await calculateSHA256(payload);
        if (block.hash !== recalcHash) return false;
    }
    return true;
}

// --- 10. M&A Diligence Claims Renderer ---
function renderMaClaims() {
    const container = document.getElementById("maClaimsContainer");
    if (!container) return;
    
    container.innerHTML = "";
    
    maClaims.forEach(claim => {
        const verification = claim.verify();
        
        let statusBadgeClass = "amber";
        if (verification.status === "DEFENSIBLE") statusBadgeClass = "emerald";
        else if (verification.status === "REJECTED") statusBadgeClass = "crimson";
        
        const card = document.createElement("div");
        card.className = "glass-card";
        card.style.padding = "10px";
        card.style.display = "flex";
        card.style.flexDirection = "column";
        card.style.gap = "4px";
        card.style.backgroundColor = "var(--bg-card)";
        card.style.border = "1px solid var(--border-color)";
        card.style.borderRadius = "6px";
        
        if (verification.status === "DEFENSIBLE") {
            card.style.boxShadow = "0 0 10px rgba(6, 214, 160, 0.08)";
            card.style.borderColor = "rgba(6, 214, 160, 0.2)";
        } else if (verification.status === "REJECTED") {
            card.style.boxShadow = "0 0 10px rgba(255, 0, 110, 0.08)";
            card.style.borderColor = "rgba(255, 0, 110, 0.2)";
        }
        
        card.innerHTML = `
            <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                <div>
                    <h3 style="font-size: 0.75rem; color: var(--color-text-main); font-weight: 700; margin-bottom: 2px;">${claim.title}</h3>
                    <span style="font-size: 0.6rem; color: var(--color-text-muted); font-family: var(--font-mono);">${claim.slideUuid}</span>
                </div>
                <span class="status-badge ${statusBadgeClass}" style="font-size: 0.65rem; padding: 1px 4px; border-radius: 4px;">${verification.status}</span>
            </div>
            <p style="font-size: 0.68rem; color: var(--color-text-muted); line-height: 1.3; margin-top: 4px;">${claim.description}</p>
            <div style="display: flex; justify-content: space-between; align-items: center; border-top: 1px solid var(--border-color); padding-top: 4px; margin-top: 4px;">
                <span style="font-size: 0.7rem; font-weight: 700; color: var(--color-primary); font-family: var(--font-mono);">$${claim.valueUsd.toLocaleString()} USD</span>
                <span style="font-size: 0.65rem; color: var(--color-text-muted);" class="font-mono">${verification.details}</span>
            </div>
        `;
        container.appendChild(card);
    });
}

// --- 11. Process Drift Monitor Chart Renderer (EWMA Control Chart) ---
function drawDriftChart() {
    const canvas = document.getElementById("ewmaCanvas");
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const w = canvas.width;
    const h = canvas.height;
    
    ctx.clearRect(0, 0, w, h);
    
    const padLeft = 32;
    const padRight = 10;
    const padTop = 15;
    const padBottom = 25;
    
    const chartW = w - padLeft - padRight;
    const chartH = h - padTop - padBottom;
    
    // Draw background grid lines
    ctx.strokeStyle = "rgba(255, 255, 255, 0.05)";
    ctx.lineWidth = 1;
    for (let i = 0; i <= 4; i++) {
        const y = padTop + (chartH * i) / 4;
        ctx.beginPath();
        ctx.moveTo(padLeft, y);
        ctx.lineTo(w - padRight, y);
        ctx.stroke();
        
        ctx.fillStyle = "#64748b";
        ctx.font = "8px 'JetBrains Mono', monospace";
        ctx.textAlign = "right";
        ctx.textBaseline = "middle";
        const val = (1.0 - i * 0.25).toFixed(2);
        ctx.fillText(val, padLeft - 6, y);
    }
    
    // Draw Lower Control Limit (LCL = 0.92)
    const yLcl = padTop + chartH * (1.0 - lcl);
    ctx.strokeStyle = "rgba(255, 0, 110, 0.6)"; // Neon Red
    ctx.lineWidth = 1.5;
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(padLeft, yLcl);
    ctx.lineTo(w - padRight, yLcl);
    ctx.stroke();
    ctx.setLineDash([]); // Reset
    
    ctx.fillStyle = "rgba(255, 0, 110, 0.8)";
    ctx.textAlign = "left";
    ctx.fillText("LCL: 0.920", padLeft + 6, yLcl - 6);
    
    if (history.length === 0) {
        ctx.fillStyle = "#64748b";
        ctx.font = "11px sans-serif";
        ctx.textAlign = "center";
        ctx.fillText("Awaiting case stream traces to compile EWMA control data...", w / 2, h / 2);
        return;
    }
    
    // Calculate mapping bounds (show last 20 traces)
    const visibleCount = 20;
    const visibleStart = Math.max(0, history.length - visibleCount);
    
    const getX = (idx) => {
        if (history.length <= 1) return padLeft + chartW / 2;
        const offset = idx - visibleStart;
        const range = Math.min(history.length, visibleCount) - 1;
        return padLeft + (chartW * offset) / Math.max(1, range);
    };
    
    const getY = (val) => {
        const v = Math.max(0, Math.min(1, val));
        return padTop + chartH * (1.0 - v);
    };
    
    // 1. Shaded area under EWMA
    ctx.beginPath();
    ctx.moveTo(getX(visibleStart), getY(0));
    for (let i = visibleStart; i < history.length; i++) {
        ctx.lineTo(getX(i), getY(history[i].ewma));
    }
    ctx.lineTo(getX(history.length - 1), getY(0));
    ctx.closePath();
    
    const areaGrad = ctx.createLinearGradient(0, padTop, 0, h - padBottom);
    areaGrad.addColorStop(0, "rgba(58, 134, 255, 0.1)");
    areaGrad.addColorStop(1, "rgba(58, 134, 255, 0)");
    ctx.fillStyle = areaGrad;
    ctx.fill();
    
    // 2. Raw trace fitness points
    for (let i = visibleStart; i < history.length; i++) {
        ctx.fillStyle = "rgba(58, 134, 255, 0.5)";
        ctx.beginPath();
        ctx.arc(getX(i), getY(history[i].fitness), 3, 0, 2 * Math.PI);
        ctx.fill();
    }
    
    // 3. EWMA Line plotting
    ctx.beginPath();
    for (let i = visibleStart; i < history.length; i++) {
        const x = getX(i);
        const y = getY(history[i].ewma);
        if (i === visibleStart) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
    }
    
    const currentEwma = history[history.length - 1].ewma;
    const isDrifting = currentEwma < lcl;
    
    ctx.strokeStyle = isDrifting ? "#ff006e" : "#06d6a0"; // Neon red or emerald
    ctx.lineWidth = 2.5;
    ctx.stroke();
    
    // X-axis labels
    ctx.fillStyle = "#64748b";
    ctx.font = "8px monospace";
    ctx.textAlign = "center";
    ctx.textBaseline = "top";
    
    const labelStep = Math.max(1, Math.floor((history.length - visibleStart) / 4));
    for (let i = visibleStart; i < history.length; i += labelStep) {
        const x = getX(i);
        const label = history[i].traceId;
        ctx.fillText(label, x, h - padBottom + 6);
    }
    
    // Force final label
    if ((history.length - 1 - visibleStart) % labelStep !== 0) {
        const x = getX(history.length - 1);
        const label = history[history.length - 1].traceId;
        ctx.fillText(label, x, h - padBottom + 6);
    }
}

// --- 12. Interactive Inspection & Conformance Alignment Renderer ---
function inspectCase(caseId, activities, fitness, duration) {
    document.getElementById("insCaseId").textContent = caseId;
    document.getElementById("insTrace").textContent = "[" + activities.join(", ") + "]";
    document.getElementById("insFitness").textContent = (fitness * 100).toFixed(0) + "%";
    document.getElementById("insDuration").textContent = duration;

    // Run A* search alignment
    const solverResult = solveAlignment(activities);
    
    // Render alignment blocks
    const container = document.getElementById("alignmentContainer");
    if (container) {
        container.innerHTML = "";
        
        const summary = document.createElement("div");
        summary.className = "alignment-results-container";
        summary.innerHTML = `
            <div class="alignment-summary" style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px;">
                <div class="glass-card" style="padding: 6px; text-align: center; border: 1px solid var(--border-color); border-radius: 6px; background-color: var(--bg-card);">
                    <div style="font-size: 0.65rem; color: var(--color-text-muted);">Alignment Fitness</div>
                    <div style="font-size: 1.1rem; font-weight: 800; color: ${solverResult.fitness >= 0.95 ? 'var(--color-success)' : 'var(--color-danger)'}; font-family: var(--font-mono);">
                        ${(solverResult.fitness * 100).toFixed(1)}%
                    </div>
                </div>
                <div class="glass-card" style="padding: 6px; text-align: center; border: 1px solid var(--border-color); border-radius: 6px; background-color: var(--bg-card);">
                    <div style="font-size: 0.65rem; color: var(--color-text-muted);">Alignment Cost</div>
                    <div style="font-size: 1.1rem; font-weight: 800; color: var(--color-warning); font-family: var(--font-mono);">
                        ${solverResult.alignment.reduce((sum, m) => sum + (m.type === 'sync' ? 0 : 1), 0)}
                    </div>
                </div>
            </div>
            <div style="display: flex; flex-direction: column; gap: 6px;" id="alignmentMovesList"></div>
        `;
        container.appendChild(summary);
        
        const movesList = document.getElementById("alignmentMovesList");
        solverResult.alignment.forEach(move => {
            const block = document.createElement("div");
            
            let topText = "";
            let bottomText = "";
            let moveClass = "";
            let cost = 0;
            
            if (move.type === "sync") {
                topText = move.label;
                bottomText = move.label;
                moveClass = "match";
                cost = 0;
            } else if (move.type === "model") {
                topText = "-";
                bottomText = move.label;
                moveClass = "move-model";
                cost = 1;
            } else if (move.type === "log") {
                topText = move.label;
                bottomText = "-";
                moveClass = "move-log";
                cost = 1;
            }
            
            block.className = `solver-step-card ${moveClass}`;
            block.style.display = "flex";
            block.style.flexDirection = "column";
            block.style.gap = "4px";
            block.style.padding = "6px 8px";
            block.style.borderRadius = "4px";
            block.style.borderLeft = "3px solid";
            if (move.type === "sync") block.style.borderLeftColor = "var(--color-success)";
            else if (move.type === "model") block.style.borderLeftColor = "var(--color-warning)";
            else if (move.type === "log") block.style.borderLeftColor = "var(--color-danger)";
            block.style.backgroundColor = "rgba(0, 0, 0, 0.2)";
            
            block.innerHTML = `
                <div style="display: flex; justify-content: space-between; font-weight: 600; font-size: 0.72rem; color: var(--color-text-main);">
                    <span>${move.type.toUpperCase()}: ${move.label}</span>
                    <span style="font-family: var(--font-mono); color: ${cost > 0 ? 'var(--color-warning)' : 'var(--color-success)'}">Cost: ${cost}</span>
                </div>
                <div style="display: grid; grid-template-columns: 45px 1fr; font-size: 0.65rem; font-family: var(--font-mono); color: var(--color-text-muted); margin-top: 2px;">
                    <span>Log:</span><span style="color: var(--color-text-main);">${topText}</span>
                    <span>Model:</span><span style="color: var(--color-text-main);">${bottomText}</span>
                </div>
            `;
            movesList.appendChild(block);
        });
    }

    // Reset step-replay state for Petri Net SVG
    currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };
    updateLabelsAndHighlights();
    
    activeInspectedTrace = {
        activities: activities,
        index: 0
    };
}

// --- 13. End-of-Trace Core Verification Logic ---
async function completeCurrentTrace() {
    totalTraces++;
    
    // 1. Solve optimal alignment using A*
    const solverResult = solveAlignment(currentTrace.activities);
    
    // 2. Validate DECLARE constraints using declare.js validator
    const declareValidator = new DeclareValidator();
    const declareResult = declareValidator.verifyTrace(currentTrace.activities, declareRules);
    
    let traceViolated = false;
    declareResult.results.forEach(res => {
        const stats = declareRuleStats[res.rule];
        if (stats) {
            stats.activations += res.activations.length;
            stats.satisfactions += res.satisfactions.length;
            stats.violations += res.violations.length;
            stats.status = res.status;
            
            if (res.status === "VIOLATED") {
                traceViolated = true;
            }
        }
    });
    
    if (traceViolated) {
        totalViolations++;
    }
    
    renderDeclareMonitor();
    
    // 3. Update drift tracking & EWMA
    const f = solverResult.fitness;
    const lastEwma = history.length > 0 ? history[history.length - 1].ewma : 1.0;
    const nextEwma = lambda * f + (1 - lambda) * lastEwma;
    history.push({ fitness: f, ewma: nextEwma, traceId: currentTrace.traceId });
    
    // Drift alerting
    const alarmBanner = document.getElementById("driftAlarmBanner");
    if (alarmBanner) {
        if (nextEwma < lcl) {
            alarmBanner.textContent = `⚠️ CRITICAL DRIFT DETECTED: Process conformance fitness falls below LCL = 0.920! (EWMA: ${nextEwma.toFixed(3)})`;
            alarmBanner.classList.add("fired");
        } else {
            alarmBanner.textContent = `No active process drifts detected. Running within standard parameters. (EWMA: ${nextEwma.toFixed(3)})`;
            alarmBanner.classList.remove("fired");
        }
    }
    
    drawDriftChart();
    
    // 4. Append cryptographically chained block to ledger
    await appendToLedger(currentTrace.traceId, f);
    
    // Verify blockchain integrity to catch any tampering
    isLedgerIntact = await verifyLedgerIntegrity();
    
    // 5. Update Diligence Claims
    renderMaClaims();

    // 6. Update Live Case Stream Table Feed
    const tbody = document.getElementById("caseListBody");
    if (tbody) {
        if (totalTraces === 1) {
            tbody.innerHTML = ""; // Clear empty state
        }
        
        const row = document.createElement("tr");
        row.id = `case-row-${currentTrace.traceId}`;
        const timeStr = new Date().toLocaleTimeString();
        const fitClass = solverResult.fitness >= 0.95 ? "high" : "low";
        
        row.dataset.traceId = currentTrace.traceId;
        row.dataset.activities = JSON.stringify(currentTrace.activities);
        row.dataset.fitness = solverResult.fitness;
        row.dataset.duration = (currentTrace.activities.length * 1.5).toFixed(1) + "s";
        
        row.innerHTML = `
            <td><strong>${currentTrace.traceId}</strong></td>
            <td><span class="trace-badge">${currentTrace.activities.length} events</span></td>
            <td><span class="fitness-badge ${fitClass}">${(solverResult.fitness * 100).toFixed(0)}%</span></td>
            <td>${timeStr}</td>
        `;
        
        row.addEventListener("click", () => {
            tbody.querySelectorAll("tr").forEach(r => r.classList.remove("active-row"));
            row.classList.add("active-row");
            inspectCase(row.dataset.traceId, JSON.parse(row.dataset.activities), parseFloat(row.dataset.fitness), row.dataset.duration);
        });
        
        tbody.insertBefore(row, tbody.firstChild);
        
        // Cap feed rows at 10 items
        if (tbody.children.length > 10) {
            tbody.removeChild(tbody.lastChild);
        }
    }

    // 7. Update Global Metrics Panel
    updateGlobalMetrics();
}

// --- 14. Simulation Step Trigger (Replays Live Token Flows) ---
function stepSimulation(callback) {
    if (!currentTrace) {
        setupNextTrace();
        if (callback) callback();
        return;
    }
    
    if (currentEventIndex < currentTrace.activities.length) {
        const act = currentTrace.activities[currentEventIndex];
        totalEvents++;
        updateGlobalMetrics();
        
        let tId = null;
        for (const id in net.transitions) {
            if (net.transitions[id].label === act) {
                tId = id;
                break;
            }
        }
        
        if (tId) {
            const trans = net.transitions[tId];
            const inPlaceId = trans.inputs[0];
            const outPlaceId = trans.outputs[0];
            
            const inPlace = net.places[inPlaceId];
            const outPlace = net.places[outPlaceId];
            
            const rect = document.getElementById(`trans_${tId}`);
            let tx = trans.x;
            let ty = trans.y;
            if (rect) {
                tx = parseInt(rect.getAttribute("x")) + 15;
                ty = parseInt(rect.getAttribute("y")) + 20;
            }
            
            const hasToken = currentMarking[inPlaceId] >= 1;
            
            if (hasToken) {
                currentMarking[inPlaceId]--;
            } else {
                // Non-conforming path / Missing token: Flash red on SVG Place
                const circle = document.getElementById(`place_${inPlaceId}`);
                if (circle) {
                    circle.style.fill = "rgba(255, 0, 110, 0.4)";
                    circle.style.stroke = "var(--color-danger)";
                    setTimeout(() => {
                        circle.style.fill = "";
                        circle.style.stroke = "";
                    }, 400);
                }
            }
            
            updateLabelsAndHighlights();
            
            const animDuration = Math.min(speed / 2, 450);
            
            animateToken(inPlace.x, inPlace.y, tx, ty, outPlace.x, outPlace.y, animDuration, () => {
                if (rect) {
                    rect.classList.add("firing");
                    setTimeout(() => rect.classList.remove("firing"), 150);
                }
                
                currentMarking[outPlaceId] = (currentMarking[outPlaceId] || 0) + 1;
                updateLabelsAndHighlights();
                
                currentEventIndex++;
                if (callback) callback();
            });
        } else {
            // Log-only move: increment and skip visually
            currentEventIndex++;
            if (callback) callback();
        }
    } else {
        // Run trace verification and load next
        completeCurrentTrace().then(() => {
            setupNextTrace();
            if (callback) callback();
        });
    }
}

// --- 15. Simulation Playback Controls ---
function startSimulation() {
    if (isRunning) return;
    isRunning = true;
    
    const startBtn = document.getElementById("btnStartSim");
    const pauseBtn = document.getElementById("btnPauseSim");
    if (startBtn) startBtn.classList.add("btn-active");
    if (pauseBtn) pauseBtn.classList.remove("btn-active");
    
    if (!currentTrace) {
        setupNextTrace();
    }
    
    playbackLoop();
}

function pauseSimulation() {
    if (!isRunning) return;
    isRunning = false;
    
    const startBtn = document.getElementById("btnStartSim");
    const pauseBtn = document.getElementById("btnPauseSim");
    if (startBtn) startBtn.classList.remove("btn-active");
    if (pauseBtn) pauseBtn.classList.add("btn-active");
    
    if (playbackTimeout) {
        clearTimeout(playbackTimeout);
        playbackTimeout = null;
    }
}

function playbackLoop() {
    if (!isRunning) return;
    stepSimulation(() => {
        playbackTimeout = setTimeout(playbackLoop, speed);
    });
}

function resetSimulation() {
    pauseSimulation();
    
    history.length = 0;
    ledger.length = 0;
    isLedgerIntact = true;
    
    totalTraces = 0;
    totalEvents = 0;
    totalViolations = 0;
    traceCounter = 0;
    
    const alarmBanner = document.getElementById("driftAlarmBanner");
    if (alarmBanner) {
        alarmBanner.textContent = "No active process drifts detected. Running within standard parameters.";
        alarmBanner.classList.remove("fired");
    }
    
    const container = document.getElementById("ledgerContainer");
    if (container) {
        container.innerHTML = `
            <div class="panel-title" style="display: flex; justify-content: space-between; align-items: center;">
                <span>Cryptographic Process Ledger (SHA-256 Audit Trail)</span>
                <span class="status-badge emerald" id="ledgerShieldBadge" style="font-weight: 700; padding: 2px 8px; border-radius: 4px; font-size: 0.75rem;">LEDGER: INTACT</span>
            </div>
            <div style="display: flex; gap: 12px; overflow-x: auto; padding: 8px 0; margin-top: 10px;" id="ledgerChain">
                <div class="text-center text-muted" style="width: 100%; padding: 20px; font-size: 0.75rem;">Awaiting transactions to write block data...</div>
            </div>
        `;
    }
    
    const solverContainer = document.getElementById("alignmentContainer");
    if (solverContainer) {
        solverContainer.innerHTML = `
            <div class="text-center text-muted" style="padding-top: 50px;">
                Select a simulated case from the feed list to compute A* alignment moves.
            </div>
        `;
    }

    const tbody = document.getElementById("caseListBody");
    if (tbody) {
        tbody.innerHTML = `
            <tr>
                <td colspan="4" class="text-center text-muted">Awaiting simulation data...</td>
            </tr>
        `;
    }
    
    for (const rule in declareRuleStats) {
        declareRuleStats[rule] = { activations: 0, satisfactions: 0, violations: 0, status: "PENDING" };
    }
    
    currentTrace = null;
    currentEventIndex = 0;
    currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };
    activeInspectedTrace = null;

    document.getElementById("insCaseId").textContent = "C-0000";
    document.getElementById("insTrace").textContent = "[]";
    document.getElementById("insFitness").textContent = "0%";
    document.getElementById("insDuration").textContent = "0s";
    
    drawPetriNetSvg();
    renderDeclareMonitor();
    renderMaClaims();
    drawDriftChart();
    setupNextTrace();
    updateGlobalMetrics();
}

// --- 16. UI Event Binding ---
document.addEventListener("DOMContentLoaded", () => {
    // Render central Petri Net SVG
    drawPetriNetSvg();

    // Stream Telemetry HUD simulation loop (FPS, Latency, drops, OBS)
    setInterval(() => {
        const fpsEl = document.getElementById("streamFps");
        const latEl = document.getElementById("streamLatency");
        const dropEl = document.getElementById("streamFrameDrops");
        const obsEl = document.getElementById("obsStatus");
        
        if (fpsEl) {
            const fps = (59.7 + Math.random() * 0.3).toFixed(1);
            fpsEl.textContent = fps;
        }
        if (latEl) {
            const latency = Math.floor(6 + Math.random() * 9);
            latEl.textContent = `${latency}ms`;
        }
        if (dropEl) {
            dropEl.textContent = "0.0%";
        }
        if (obsEl) {
            obsEl.textContent = "CONNECTED";
        }
    }, 1000);
    
    // Play/Pause Stream buttons
    document.getElementById("btnStartSim")?.addEventListener("click", () => {
        startSimulation();
    });
    document.getElementById("btnPauseSim")?.addEventListener("click", () => {
        pauseSimulation();
    });
    
    // Simulate Single Case button
    document.getElementById("btnSingleSim")?.addEventListener("click", () => {
        pauseSimulation();
        // Step trace until current trace is complete, then reset step counter
        function runWholeTrace() {
            if (currentEventIndex < currentTrace.activities.length) {
                stepSimulation(() => {
                    setTimeout(runWholeTrace, speed / 2);
                });
            } else {
                stepSimulation(); // Completes the trace and sets up next
            }
        }
        runWholeTrace();
    });
    
    // Reset Statistics button
    document.getElementById("btnClearStats")?.addEventListener("click", () => {
        resetSimulation();
    });
    
    // Step Trace (Inspect Step Replay) button
    document.getElementById("btnStepReplay")?.addEventListener("click", () => {
        if (!activeInspectedTrace || activeInspectedTrace.index >= activeInspectedTrace.activities.length) {
            if (activeInspectedTrace) {
                activeInspectedTrace.index = 0;
                currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };
                updateLabelsAndHighlights();
            }
            return;
        }
        
        const act = activeInspectedTrace.activities[activeInspectedTrace.index];
        let tId = null;
        for (const id in net.transitions) {
            if (net.transitions[id].label === act) {
                tId = id;
                break;
            }
        }
        
        if (tId) {
            const trans = net.transitions[tId];
            const inPlaceId = trans.inputs[0];
            const outPlaceId = trans.outputs[0];
            
            const inPlace = net.places[inPlaceId];
            const outPlace = net.places[outPlaceId];
            
            const rect = document.getElementById(`trans_${tId}`);
            let tx = trans.x;
            let ty = trans.y;
            if (rect) {
                tx = parseInt(rect.getAttribute("x")) + 15;
                ty = parseInt(rect.getAttribute("y")) + 20;
            }
            
            const hasToken = currentMarking[inPlaceId] >= 1;
            if (hasToken) {
                currentMarking[inPlaceId]--;
            } else {
                // Flash Place Red on missing token
                const circle = document.getElementById(`place_${inPlaceId}`);
                if (circle) {
                    circle.style.fill = "rgba(255, 0, 110, 0.4)";
                    circle.style.stroke = "var(--color-danger)";
                    setTimeout(() => {
                        circle.style.fill = "";
                        circle.style.stroke = "";
                    }, 400);
                }
            }
            
            updateLabelsAndHighlights();
            
            animateToken(inPlace.x, inPlace.y, tx, ty, outPlace.x, outPlace.y, 400, () => {
                if (rect) {
                    rect.classList.add("firing");
                    setTimeout(() => rect.classList.remove("firing"), 150);
                }
                
                currentMarking[outPlaceId] = (currentMarking[outPlaceId] || 0) + 1;
                updateLabelsAndHighlights();
                activeInspectedTrace.index++;
                
                if (activeInspectedTrace.index === activeInspectedTrace.activities.length) {
                    setTimeout(() => {
                        if (currentMarking.p_end === 1) {
                            currentMarking.p_end = 0;
                            updateLabelsAndHighlights();
                        }
                    }, 400);
                }
            });
        } else {
            // Log move only
            activeInspectedTrace.index++;
        }
    });
    
    // Reset Trace (Inspect Step Replay Reset) button
    document.getElementById("btnResetReplay")?.addEventListener("click", () => {
        if (activeInspectedTrace) {
            activeInspectedTrace.index = 0;
        }
        currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };
        updateLabelsAndHighlights();
    });

    // Drift checkbox and selector control
    const driftToggle = document.getElementById("driftToggle");
    const driftTypeSelector = document.getElementById("driftTypeSelector");
    driftToggle?.addEventListener("change", (e) => {
        if (driftTypeSelector) {
            driftTypeSelector.disabled = !e.target.checked;
        }
    });
    
    // Initial Render Orchestration
    renderDeclareMonitor();
    renderMaClaims();
    drawDriftChart();
    setupNextTrace();
    updateGlobalMetrics();
});
