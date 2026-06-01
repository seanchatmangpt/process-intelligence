/**
 * app.js
 * Process Intelligence Simulation Dashboard - Core Logic Binder
 * 
 * Binds all visualizer components:
 * - Petri net SVG token game player & animation
 * - Priority-queue based A* alignment solver
 * - DECLARE constraint monitor & stats aggregator
 * - EWMA process drift calculator & Canvas chart renderer
 * - Cryptographic ledger ledger with SHA-256 event chaining
 * - M&A Diligence Claims board verification bridge
 */

// --- 1. Petri Net Model and State Space ---
const net = {
    places: {
        p_start: { name: "Start", x: 50, y: 100 },
        p_created: { name: "Created", x: 200, y: 100 },
        p_received: { name: "Received", x: 350, y: 100 },
        p_audited: { name: "Audited", x: 500, y: 100 },
        p_approved: { name: "Approved", x: 650, y: 100 },
        p_end: { name: "End", x: 800, y: 100 }
    },
    transitions: {
        t_create: { label: "Create_Order", inputs: ["p_start"], outputs: ["p_created"] },
        t_receive: { label: "Receive_Goods", inputs: ["p_created"], outputs: ["p_received"] },
        t_audit: { label: "Audit_Invoice", inputs: ["p_received"], outputs: ["p_audited"] },
        t_approve: { label: "Approve_Payment", inputs: ["p_audited"], outputs: ["p_approved"] },
        t_close: { label: "Close_Case", inputs: ["p_approved"], outputs: ["p_end"] }
    }
};

// --- 2. State & Statistics ---
let isRunning = false;
let playbackTimeout = null;
let speed = 1000; // ms per event
let isViolationModeActive = false;

let currentTrace = null;
let currentEventIndex = 0;
let currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };

let totalTraces = 0;
let totalEvents = 0;
let totalViolations = 0;
let traceCounter = 0;

// Drift and Chart Tracking
const history = []; // Holds { fitness, ewma, traceId }
const lambda = 0.2; // EWMA smoothing factor
const lcl = 0.92;   // Lower Control Limit for drift

// Ledger tracking
const ledger = [];
let isLedgerIntact = true;

// DECLARE Rules setup
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
            if (ledger.length < 5) {
                return { status: "PENDING", details: `Ledger blocks: ${ledger.length}/5 (Awaiting size requirement)` };
            }
            return {
                status: isLedgerIntact ? "DEFENSIBLE" : "REJECTED",
                details: isLedgerIntact ? `Ledger verified. Blocks: ${ledger.length}. No tampering.` : "Ledger signature mismatch / tampered!"
            };
        }
    }
];

// --- 3. Cryptographic Helper (SHA-256) ---
async function sha256(message) {
    const msgBuffer = new TextEncoder().encode(message);
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

// --- 4. A* Alignment Solver Core Logic ---
class State {
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
    const initialMarking = {
        p_start: 1,
        p_created: 0,
        p_received: 0,
        p_audited: 0,
        p_approved: 0,
        p_end: 0
    };
    
    const startState = new State(
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
        
        // Goal marking: p_end has 1 token and all other places are empty
        const isGoalMarking = curr.marking.p_end === 1 && 
            Object.keys(curr.marking).every(p => p === 'p_end' || curr.marking[p] === 0);
        
        if (isGoalMarking && curr.traceIndex === traceLength) {
            const path = [];
            let state = curr;
            while (state.parent !== null) {
                path.unshift(state.move);
                state = state.parent;
            }
            // alignment fitness score
            const worstCost = traceLength + 5;
            const fitness = 1 - curr.g / worstCost;
            return {
                alignment: path,
                expandedCount,
                queueSize: openList.length,
                fitness: fitness
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
                    const nextState = new State(
                        nextMarking,
                        curr.traceIndex + 1,
                        curr.g, // Cost of sync move = 0
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
                const nextState = new State(
                    nextMarking,
                    curr.traceIndex,
                    curr.g + 1, // Cost of model move = 1
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
            const nextState = new State(
                curr.marking,
                curr.traceIndex + 1,
                curr.g + 1, // Cost of log move = 1
                h,
                curr,
                { type: 'log', label: currentEvent, eventIndex: curr.traceIndex }
            );
            openList.push(nextState);
        }
    }
    
    return { alignment: [], expandedCount, queueSize: 0, fitness: 0 };
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

function updateLabelsAndHighlights() {
    for (const pId in net.places) {
        const text = document.getElementById(`place_${pId}_tokens`);
        const circle = document.getElementById(`place_${pId}`);
        if (text) {
            text.textContent = currentMarking[pId] || 0;
        }
        if (circle) {
            if ((currentMarking[pId] || 0) >= 1) {
                circle.classList.add("highlight");
            } else {
                circle.classList.remove("highlight");
            }
        }
    }
    
    // Highlight enabled transitions
    for (const tId in net.transitions) {
        const trans = net.transitions[tId];
        const rect = document.getElementById(`trans_${tId}`);
        if (!rect) continue;
        
        if (isTransitionEnabled(trans, currentMarking)) {
            rect.classList.add("highlight");
        } else {
            rect.classList.remove("highlight");
        }
    }
}

// --- 6. Mock Log Stream Generator ---
function generateTrace() {
    traceCounter++;
    const traceId = `trace_${1000 + traceCounter}`;
    const rand = Math.random();
    
    let activities = [];
    let isIntendedViolation = false;
    
    if (isViolationModeActive) {
        if (rand < 0.8) {
            // Bypassed invoice audit transition
            activities = ["Create_Order", "Receive_Goods", "Approve_Payment", "Close_Case"];
            isIntendedViolation = true;
        } else {
            activities = ["Create_Order", "Receive_Goods", "Audit_Invoice", "Approve_Payment", "Close_Case"];
        }
    } else {
        if (rand < 0.88) {
            // Normal fully compliant sequence
            activities = ["Create_Order", "Receive_Goods", "Audit_Invoice", "Approve_Payment", "Close_Case"];
        } else if (rand < 0.94) {
            // Vacuous satisfaction / Incomplete trace
            activities = ["Create_Order", "Receive_Goods"];
        } else {
            // Rare spontaneous violation
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
    
    currentMarking = {
        p_start: 1,
        p_created: 0,
        p_received: 0,
        p_audited: 0,
        p_approved: 0,
        p_end: 0
    };
    
    document.getElementById("currentTraceIdBadge").textContent = `Trace: ${currentTrace.traceId}`;
    updateLabelsAndHighlights();
    
    // Reset solver step list with upcoming sequence preview
    const list = document.getElementById("solverStepList");
    if (list) {
        list.innerHTML = `
            <div class="glass-card solver-step-card" style="border-left-color: var(--neon-cyan);">
                <div style="font-weight: 600; color: var(--neon-cyan);">Upcoming Event Sequence:</div>
                <div style="font-family: var(--font-mono); font-size: 0.75rem; margin-top: 4px;">
                    ${currentTrace.activities.join(" &rarr; ")}
                </div>
                ${currentTrace.isIntendedViolation ? 
                  `<div style="color: var(--crimson-red); font-size: 0.7rem; margin-top: 6px; font-weight: 500;">⚠ Intended Non-Conformant Path (Audit Bypass)</div>` : ''}
            </div>
        `;
    }
}

// --- 7. DECLARE Rule Monitor Renderer ---
function renderDeclareMonitor() {
    const container = document.getElementById("declareRulesContainer");
    if (!container) return;
    
    container.innerHTML = "";
    
    for (const ruleStr in declareRuleStats) {
        const stats = declareRuleStats[ruleStr];
        const card = document.createElement("div");
        card.className = "glass-card";
        card.style.padding = "12px";
        card.style.display = "flex";
        card.style.flexDirection = "column";
        card.style.gap = "6px";
        
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
                <span class="font-mono text-primary" style="font-weight: 600; font-size: 0.75rem;">${ruleStr}</span>
                <span class="status-badge ${statusClass}">${statusText}</span>
            </div>
            <div style="display: flex; justify-content: space-between; font-size: 0.7rem; color: var(--text-muted); margin-top: 4px;">
                <span>Activations: <strong class="text-primary font-mono">${stats.activations}</strong></span>
                <span>Fulfillments: <strong class="text-primary font-mono">${stats.satisfactions}</strong></span>
                <span>Violations: <strong class="text-primary font-mono">${stats.violations}</strong></span>
            </div>
        `;
        container.appendChild(card);
    }
}

// --- 8. Cryptographic Ledger Logic ---
async function appendToLedger(traceId, fitness) {
    const prevBlock = ledger[ledger.length - 1];
    const prevHash = prevBlock ? prevBlock.hash : "0000000000000000000000000000000000000000000000000000000000000000";
    const timestamp = new Date().toISOString();
    const blockId = ledger.length + 1;
    
    // Cryptographic event chaining structure
    const payload = `${blockId}|${timestamp}|${traceId}|${fitness.toFixed(4)}|${prevHash}`;
    const hash = await sha256(payload);
    
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
    const container = document.getElementById("ledgerChain");
    if (!container) return;
    
    container.innerHTML = "";
    
    // Render last 4 blocks in reverse order
    const visibleBlocks = ledger.slice().reverse().slice(0, 4);
    
    visibleBlocks.forEach(block => {
        const node = document.createElement("div");
        node.className = "blockchain-node glass-card";
        node.innerHTML = `
            <div style="display: flex; justify-content: space-between; font-weight: 600; margin-bottom: 4px;">
                <span style="color: var(--neon-cyan);">Block #${block.blockId}</span>
                <span style="color: var(--text-muted); font-size: 0.7rem;">${block.timestamp.slice(11, 19)}</span>
            </div>
            <div style="display: grid; grid-template-columns: 75px 1fr; gap: 4px; line-height: 1.3;">
                <span class="hash-label">Trace ID:</span>
                <span class="font-mono text-primary">${block.traceId}</span>
                
                <span class="hash-label">Fitness:</span>
                <span class="font-mono" style="color: ${block.fitness >= 0.95 ? 'var(--emerald-green)' : 'var(--crimson-red)'}">${block.fitness.toFixed(3)}</span>
                
                <span class="hash-label">Prev Hash:</span>
                <span class="font-mono text-muted" style="text-overflow: ellipsis; overflow: hidden; white-space: nowrap;">${block.prevHash}</span>
                
                <span class="hash-label">Hash:</span>
                <span class="font-mono hash-value">${block.hash}</span>
            </div>
        `;
        container.appendChild(node);
    });
}

async function verifyLedgerIntegrity() {
    if (ledger.length === 0) return true;
    for (let i = 0; i < ledger.length; i++) {
        const block = ledger[i];
        const prevBlock = ledger[i - 1];
        const expectedPrevHash = prevBlock ? prevBlock.hash : "0000000000000000000000000000000000000000000000000000000000000000";
        
        if (block.prevHash !== expectedPrevHash) return false;
        
        const payload = `${block.blockId}|${block.timestamp}|${block.traceId}|${block.fitness.toFixed(4)}|${block.prevHash}`;
        const recalcHash = await sha256(payload);
        if (block.hash !== recalcHash) return false;
    }
    return true;
}

// --- 9. M&A Diligence Claims Renderer ---
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
        card.style.padding = "14px";
        card.style.display = "flex";
        card.style.flexDirection = "column";
        card.style.gap = "8px";
        
        if (verification.status === "DEFENSIBLE") {
            card.style.boxShadow = "0 0 15px rgba(16, 185, 129, 0.08)";
            card.style.borderColor = "rgba(16, 185, 129, 0.2)";
        } else if (verification.status === "REJECTED") {
            card.style.boxShadow = "0 0 15px rgba(239, 68, 68, 0.08)";
            card.style.borderColor = "rgba(239, 68, 68, 0.2)";
        }
        
        card.innerHTML = `
            <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                <div>
                    <h3 style="font-size: 0.85rem; color: var(--text-primary); font-weight: 700;">${claim.title}</h3>
                    <span style="font-size: 0.7rem; color: var(--text-muted); font-family: var(--font-mono);">${claim.slideUuid}</span>
                </div>
                <span class="status-badge ${statusBadgeClass}">${verification.status}</span>
            </div>
            <p style="font-size: 0.75rem; color: var(--text-secondary); line-height: 1.3;">${claim.description}</p>
            <div style="display: flex; justify-content: space-between; align-items: center; border-top: 1px solid var(--border-glass); padding-top: 8px; margin-top: 4px;">
                <span style="font-size: 0.8rem; font-weight: 700; color: var(--neon-cyan); font-family: var(--font-mono);">$${claim.valueUsd.toLocaleString()} USD</span>
                <span style="font-size: 0.7rem; color: var(--text-muted);" class="font-mono">${verification.details}</span>
            </div>
        `;
        container.appendChild(card);
    });
}

// --- 10. Process Drift Monitor Chart Renderer ---
function drawDriftChart() {
    const canvas = document.getElementById("driftCanvas");
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
        ctx.font = "8px 'JetBrains Mono'";
        ctx.textAlign = "right";
        ctx.textBaseline = "middle";
        const val = (1.0 - i * 0.25).toFixed(2);
        ctx.fillText(val, padLeft - 6, y);
    }
    
    // Draw Lower Control Limit (LCL = 0.92)
    const yLcl = padTop + chartH * (1.0 - lcl);
    ctx.strokeStyle = "rgba(239, 68, 68, 0.6)";
    ctx.lineWidth = 1.5;
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(padLeft, yLcl);
    ctx.lineTo(w - padRight, yLcl);
    ctx.stroke();
    ctx.setLineDash([]); // Reset
    
    ctx.fillStyle = "rgba(239, 68, 68, 0.8)";
    ctx.textAlign = "left";
    ctx.fillText("LCL: 0.920", padLeft + 6, yLcl - 6);
    
    if (history.length === 0) {
        ctx.fillStyle = "#64748b";
        ctx.font = "11px 'Inter'";
        ctx.textAlign = "center";
        ctx.fillText("No data yet. Start simulation...", w / 2, h / 2);
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
    areaGrad.addColorStop(0, "rgba(0, 242, 254, 0.1)");
    areaGrad.addColorStop(1, "rgba(0, 242, 254, 0)");
    ctx.fillStyle = areaGrad;
    ctx.fill();
    
    // 2. Raw trace fitness points
    for (let i = visibleStart; i < history.length; i++) {
        ctx.fillStyle = "rgba(0, 242, 254, 0.5)";
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
    
    ctx.strokeStyle = isDrifting ? "#ef4444" : "#10b981";
    ctx.lineWidth = 2.5;
    ctx.shadowColor = isDrifting ? "rgba(239, 68, 68, 0.4)" : "rgba(16, 185, 129, 0.4)";
    ctx.shadowBlur = 6;
    ctx.stroke();
    ctx.shadowBlur = 0; // Reset
    
    // X-axis labels
    ctx.fillStyle = "#64748b";
    ctx.font = "8px 'JetBrains Mono'";
    ctx.textAlign = "center";
    ctx.textBaseline = "top";
    
    const labelStep = Math.max(1, Math.floor((history.length - visibleStart) / 4));
    for (let i = visibleStart; i < history.length; i += labelStep) {
        const x = getX(i);
        const label = history[i].traceId.replace("trace_", "");
        ctx.fillText(label, x, h - padBottom + 6);
    }
    
    // Force final label
    if ((history.length - 1 - visibleStart) % labelStep !== 0) {
        const x = getX(history.length - 1);
        const label = history[history.length - 1].traceId.replace("trace_", "");
        ctx.fillText(label, x, h - padBottom + 6);
    }
}

// --- 11. Core Replay Loop Stepper ---
function appendSolverRunningPreview(act) {
    const list = document.getElementById("solverStepList");
    if (!list) return;
    
    if (currentEventIndex === 0) {
        list.innerHTML = "";
    }
    
    const card = document.createElement("div");
    card.className = "solver-step-card";
    card.style.borderLeftColor = "var(--neon-cyan)";
    card.innerHTML = `
        <div style="display: flex; justify-content: space-between;">
            <span>Replayed: <strong>${act}</strong></span>
            <span style="color: var(--text-muted); font-size: 0.75rem;">Step ${currentEventIndex + 1}</span>
        </div>
    `;
    list.appendChild(card);
    list.scrollTop = list.scrollHeight;
}

function renderAlignmentResult(solverResult, trace) {
    const list = document.getElementById("solverStepList");
    if (!list) return;
    
    list.innerHTML = "";
    
    document.getElementById("solverExpandedNodes").textContent = solverResult.expandedCount;
    document.getElementById("solverQueueSize").textContent = solverResult.queueSize;
    
    const badge = document.getElementById("currentTraceFitnessBadge");
    if (badge) {
        badge.textContent = `Fitness: ${solverResult.fitness.toFixed(3)}`;
        badge.className = `status-badge ${solverResult.fitness >= 0.95 ? 'emerald' : 'crimson'}`;
    }
    
    solverResult.alignment.forEach(move => {
        const card = document.createElement("div");
        
        if (move.type === "sync") {
            card.className = "solver-step-card match";
            card.innerHTML = `
                <div style="display: flex; justify-content: space-between; font-weight: 600;">
                    <span>Synchronous Move: ${move.label}</span>
                    <span style="color: var(--emerald-green);">Cost: 0</span>
                </div>
                <div style="font-size: 0.7rem; color: var(--text-muted); margin-top: 2px;">
                    Model transition <strong>${move.transition}</strong> matched event at log index ${move.eventIndex}.
                </div>
            `;
        } else if (move.type === "model") {
            card.className = "solver-step-card move-model";
            card.innerHTML = `
                <div style="display: flex; justify-content: space-between; font-weight: 600;">
                    <span>Move on Model: ${move.label}</span>
                    <span style="color: var(--amber-orange);">Cost: +1</span>
                </div>
                <div style="font-size: 0.7rem; color: var(--text-muted); margin-top: 2px;">
                    Model fired transition <strong>${move.transition}</strong>, but activity was missing in log. (Bypass/Skip)
                </div>
            `;
        } else if (move.type === "log") {
            card.className = "solver-step-card move-log";
            card.innerHTML = `
                <div style="display: flex; justify-content: space-between; font-weight: 600;">
                    <span>Move on Log: ${move.label}</span>
                    <span style="color: var(--crimson-red);">Cost: +1</span>
                </div>
                <div style="font-size: 0.7rem; color: var(--text-muted); margin-top: 2px;">
                    Log executed event at index ${move.eventIndex}, but model was not in a state to fire this transition.
                </div>
            `;
        }
        list.appendChild(card);
    });
}

async function completeCurrentTrace() {
    totalTraces++;
    document.getElementById("statTraces").textContent = totalTraces;
    
    // 1. Solve optimal alignment using A*
    const solverResult = solveAlignment(currentTrace.activities);
    renderAlignmentResult(solverResult, currentTrace);
    
    // 2. Validate DECLARE constraints
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
        document.getElementById("statViolations").textContent = totalViolations;
    }
    
    renderDeclareMonitor();
    
    // 3. Update drift tracking & EWMA
    const f = solverResult.fitness;
    const lastEwma = history.length > 0 ? history[history.length - 1].ewma : 1.0;
    const nextEwma = lambda * f + (1 - lambda) * lastEwma;
    history.push({ fitness: f, ewma: nextEwma, traceId: currentTrace.traceId });
    
    const avgFitness = history.reduce((sum, h) => sum + h.fitness, 0) / history.length;
    document.getElementById("statFitness").textContent = avgFitness.toFixed(2);
    
    // Drift alerting
    const driftAlert = document.getElementById("driftAlertPanel");
    if (nextEwma < lcl) {
        driftAlert.style.display = "flex";
    } else {
        driftAlert.style.display = "none";
    }
    
    drawDriftChart();
    
    // 4. Append cryptographically chained block to ledger
    await appendToLedger(currentTrace.traceId, f);
    
    // Verify blockchain integrity to catch any tampering
    isLedgerIntact = await verifyLedgerIntegrity();
    
    // 5. Update Diligence Claims
    renderMaClaims();
}

function stepSimulation(callback) {
    if (!currentTrace) {
        setupNextTrace();
        if (callback) callback();
        return;
    }
    
    if (currentEventIndex < currentTrace.activities.length) {
        const act = currentTrace.activities[currentEventIndex];
        totalEvents++;
        document.getElementById("statEvents").textContent = totalEvents;
        
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
            const tx = parseInt(rect.getAttribute("x")) + 15;
            const ty = parseInt(rect.getAttribute("y")) + 20;
            
            // Replay token flow & consumption checks
            const hasToken = currentMarking[inPlaceId] >= 1;
            
            if (hasToken) {
                currentMarking[inPlaceId]--;
            } else {
                // Missing token bypass: Flash red on SVG
                const circle = document.getElementById(`place_${inPlaceId}`);
                if (circle) {
                    circle.style.fill = "rgba(239, 68, 68, 0.4)";
                    circle.style.stroke = "var(--crimson-red)";
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
                
                appendSolverRunningPreview(act);
                currentEventIndex++;
                if (callback) callback();
            });
        } else {
            // Unmapped action - treated as log move
            appendSolverRunningPreview(`[DEVIATION] ${act}`);
            currentEventIndex++;
            if (callback) callback();
        }
    } else {
        // Run end-of-trace analysis and load next
        completeCurrentTrace().then(() => {
            setupNextTrace();
            if (callback) callback();
        });
    }
}

// --- 12. Simulation Execution Control ---
function startSimulation() {
    if (isRunning) return;
    isRunning = true;
    
    const playBtn = document.getElementById("btnPlayPause");
    playBtn.innerHTML = `<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg> Pause`;
    playBtn.className = "btn-cyan pulse";
    
    if (!currentTrace) {
        setupNextTrace();
    }
    
    playbackLoop();
}

function pauseSimulation() {
    if (!isRunning) return;
    isRunning = false;
    
    const playBtn = document.getElementById("btnPlayPause");
    playBtn.innerHTML = `<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg> Play`;
    playBtn.className = "btn-cyan";
    
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
    
    totalTraces = 0;
    totalEvents = 0;
    totalViolations = 0;
    traceCounter = 0;
    
    document.getElementById("statTraces").textContent = "0";
    document.getElementById("statEvents").textContent = "0";
    document.getElementById("statFitness").textContent = "1.00";
    document.getElementById("statViolations").textContent = "0";
    
    document.getElementById("driftAlertPanel").style.display = "none";
    document.getElementById("ledgerChain").innerHTML = "";
    document.getElementById("solverStepList").innerHTML = `
        <div class="glass-card solver-step-card" style="text-align: center; justify-content: center; padding: 30px; color: var(--text-muted);">
            Waiting for simulation event stream to start...
        </div>
    `;
    
    for (const rule in declareRuleStats) {
        declareRuleStats[rule] = { activations: 0, satisfactions: 0, violations: 0, status: "PENDING" };
    }
    
    currentTrace = null;
    currentEventIndex = 0;
    currentMarking = { p_start: 1, p_created: 0, p_received: 0, p_audited: 0, p_approved: 0, p_end: 0 };
    
    updateLabelsAndHighlights();
    renderDeclareMonitor();
    renderMaClaims();
    drawDriftChart();
}

// --- 13. UI Event Binding ---
document.addEventListener("DOMContentLoaded", () => {
    // Clock tick
    setInterval(() => {
        const timeSpan = document.getElementById("systemTime");
        if (timeSpan) {
            const now = new Date();
            timeSpan.textContent = now.toISOString().replace('T', ' ').substring(0, 19);
        }
    }, 1000);
    
    // Bind buttons
    document.getElementById("btnPlayPause").addEventListener("click", () => {
        if (isRunning) pauseSimulation();
        else startSimulation();
    });
    
    document.getElementById("btnStep").addEventListener("click", () => {
        pauseSimulation();
        stepSimulation();
    });
    
    document.getElementById("btnReset").addEventListener("click", () => {
        resetSimulation();
    });
    
    // Speed Slider
    const sliderSpeed = document.getElementById("sliderSpeed");
    const speedLabel = document.getElementById("speedLabel");
    sliderSpeed.addEventListener("input", (e) => {
        speed = parseInt(e.target.value);
        speedLabel.textContent = `${speed}ms`;
    });
    
    // Violation mode trigger
    const btnToggleViolation = document.getElementById("btnToggleViolation");
    const violationStatus = document.getElementById("violationStatus");
    btnToggleViolation.addEventListener("click", () => {
        isViolationModeActive = !isViolationModeActive;
        if (isViolationModeActive) {
            violationStatus.textContent = "ON";
            violationStatus.className = "status-badge crimson pulse";
            btnToggleViolation.textContent = "Disable Conformance Drift";
        } else {
            violationStatus.textContent = "OFF";
            violationStatus.className = "status-badge crimson";
            btnToggleViolation.textContent = "Trigger Conformance Drift";
        }
        
        // If simulation is running, it will automatically start outputting violations
    });
    
    // Initialization render
    renderDeclareMonitor();
    renderMaClaims();
    drawDriftChart();
    setupNextTrace();
});
