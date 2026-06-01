/**
 * petri.js
 * Petri Net Token-Game Simulator & SVG Graph Renderer
 * 
 * Exposes:
 * - PetriNetSimulator: Core token game engine computing marking changes, missing/remaining tokens, and conformance fitness.
 * - PetriNetVisualizer: High-performance SVG renderer that paints places, transitions, and arcs, and runs premium two-stage token animations.
 * 
 * References:
 * - Conformance Replay Doctrine: file:///Users/sac/process-intelligence/experiments/petri_conformance_sample.md
 * - Workflow Net Soundness Requirements: file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md
 * - Petri Net Placement Standards: file:///Users/sac/process-intelligence/standards/petri_net_placement.md
 */

class PetriNetSimulator {
    constructor() {
        // Define the sound Workflow Net representing the evidence collection and validation pipeline.
        // Places: i (Start), Queue, Verifying, Attesting, Receipts, o (End).
        this.net = {
            places: {
                'i': { id: 'i', label: 'Start', x: 60, y: 150 },
                'Queue': { id: 'Queue', label: 'Queue', x: 220, y: 150 },
                'Verifying': { id: 'Verifying', label: 'Verifying', x: 400, y: 150 },
                'Attesting': { id: 'Attesting', label: 'Attesting', x: 580, y: 150 },
                'Receipts': { id: 'Receipts', label: 'Receipts', x: 740, y: 150 },
                'o': { id: 'o', label: 'End', x: 860, y: 150 }
            },
            transitions: {
                't_receive': {
                    id: 't_receive',
                    label: 'Receive Order',
                    x: 140,
                    y: 125,
                    width: 40,
                    height: 50,
                    preset: ['i'],
                    postset: ['Queue']
                },
                't_check': {
                    id: 't_check',
                    label: 'Check Inventory',
                    x: 310,
                    y: 75,
                    width: 50,
                    height: 40,
                    preset: ['Queue'],
                    postset: ['Verifying']
                },
                't_skip': {
                    id: 't_skip',
                    label: 'tau_skip', // Silent transition (costs 0, auto-fired to bypass check)
                    x: 310,
                    y: 185,
                    width: 50,
                    height: 40,
                    preset: ['Queue'],
                    postset: ['Verifying']
                },
                't_approve': {
                    id: 't_approve',
                    label: 'Approve',
                    x: 490,
                    y: 125,
                    width: 50,
                    height: 50,
                    preset: ['Verifying'],
                    postset: ['Attesting']
                },
                't_ship': {
                    id: 't_ship',
                    label: 'Ship',
                    x: 660,
                    y: 125,
                    width: 50,
                    height: 50,
                    preset: ['Attesting'],
                    postset: ['Receipts']
                },
                't_end': {
                    id: 't_end',
                    label: 'End',
                    x: 800,
                    y: 125,
                    width: 30,
                    height: 50,
                    preset: ['Receipts'],
                    postset: ['o']
                }
            },
            arcs: [
                { from: 'i', to: 't_receive' },
                { from: 't_receive', to: 'Queue' },
                { from: 'Queue', to: 't_check' },
                { from: 't_check', to: 'Verifying' },
                { from: 'Queue', to: 't_skip' },
                { from: 't_skip', to: 'Verifying' },
                { from: 'Verifying', to: 't_approve' },
                { from: 't_approve', to: 'Attesting' },
                { from: 'Attesting', to: 't_ship' },
                { from: 't_ship', to: 'Receipts' },
                { from: 'Receipts', to: 't_end' },
                { from: 't_end', to: 'o' }
            ],
            initialMarking: { 'i': 1 },
            finalMarking: { 'o': 1 }
        };

        this.reset();
    }

    /**
     * Resets the simulator to the initial state.
     */
    reset() {
        this.marking = { ...this.net.initialMarking };
        
        // Replay stats conforming to petri_conformance_sample.md
        this.tokensProduced = Object.values(this.net.initialMarking).reduce((a, b) => a + b, 0);
        this.tokensConsumed = 0;
        this.tokensMissing = 0;
        this.tokensRemaining = 0;
        
        this.history = [];
        this.isFinished = false;
    }

    /**
     * Returns a list of currently enabled transition IDs.
     */
    getEnabledTransitions() {
        const enabled = [];
        for (const [tid, t] of Object.entries(this.net.transitions)) {
            let isEnabled = true;
            for (const pid of t.preset) {
                if ((this.marking[pid] || 0) < 1) {
                    isEnabled = false;
                    break;
                }
            }
            if (isEnabled) {
                enabled.push(tid);
            }
        }
        return enabled;
    }

    /**
     * Finds a transition ID by its label.
     */
    findTransitionByLabel(label) {
        for (const [tid, t] of Object.entries(this.net.transitions)) {
            if (t.label === label) {
                return tid;
            }
        }
        return null;
    }

    /**
     * Fires a transition, updating marking and replay statistics.
     * @param {string} tid - Transition ID
     * @returns {Object} Fire step diagnostic detailing preset, postset, and missing tokens.
     */
    fireTransition(tid) {
        const t = this.net.transitions[tid];
        if (!t) throw new Error(`Transition ${tid} not found`);

        const stepReport = {
            transition: tid,
            label: t.label,
            missingBeforeFire: [],
            presetBeforeMarking: { ...this.marking }
        };

        // 1. Consume tokens from preset (resolve missing tokens if any)
        for (const pid of t.preset) {
            if ((this.marking[pid] || 0) < 1) {
                // Missing token logic
                this.tokensMissing++;
                // Artificial injection to allow fire (counts as produced too)
                this.tokensProduced++; 
                this.marking[pid] = (this.marking[pid] || 0) + 1;
                stepReport.missingBeforeFire.push(pid);
            }
            
            // Consume
            this.marking[pid]--;
            this.tokensConsumed++;
            if (this.marking[pid] <= 0) {
                delete this.marking[pid];
            }
        }

        // 2. Produce tokens in postset
        for (const pid of t.postset) {
            this.marking[pid] = (this.marking[pid] || 0) + 1;
            this.tokensProduced++;
        }

        this.history.push({
            transitionId: tid,
            label: t.label,
            markingAfter: { ...this.marking }
        });

        return stepReport;
    }

    /**
     * Attempts to process a trace event (by activity name).
     * Automatically handles silent transitions to bypass blocks where possible.
     * @param {string} activityName - Event label in log trace
     * @returns {Array<Object>} Fired transition steps (can be multiple if silent steps fired)
     */
    stepEvent(activityName) {
        const firedSteps = [];

        // 1. Find target transition matching this activity label
        const tid = this.findTransitionByLabel(activityName);
        if (!tid) {
            // Activity is not in model (e.g. deviant illegal Refund)
            // In alignment terms, this is a move-on-log. In token game replay, we do not fire anything.
            return [];
        }

        // 2. Check if the target is already enabled
        const enabled = this.getEnabledTransitions();
        if (enabled.includes(tid)) {
            const report = this.fireTransition(tid);
            firedSteps.push(report);
            return firedSteps;
        }

        // 3. Try to enable it by firing silent (tau) transitions
        // For our sample net: t_skip is silent and can bypass Check Inventory
        if (activityName === 'Approve') {
            const queueHasToken = (this.marking['Queue'] || 0) > 0;
            const verifyingIsEmpty = (this.marking['Verifying'] || 0) === 0;
            
            if (queueHasToken && verifyingIsEmpty) {
                // Fire silent bypass first
                const silentReport = this.fireTransition('t_skip');
                firedSteps.push(silentReport);
                
                // Now fire the target transition
                const targetReport = this.fireTransition(tid);
                firedSteps.push(targetReport);
                return firedSteps;
            }
        }

        // 4. Force fire target transition (will record missing tokens)
        const forceReport = this.fireTransition(tid);
        firedSteps.push(forceReport);
        return firedSteps;
    }

    /**
     * Ends the replay trace and performs final marking validation against finalMarking.
     * Consumes final tokens and determines remaining tokens.
     */
    terminateReplay() {
        if (this.isFinished) return;
        this.isFinished = true;

        // Compare current marking against final marking ([o: 1])
        for (const [pid, expectedCount] of Object.entries(this.net.finalMarking)) {
            const currentCount = this.marking[pid] || 0;
            if (currentCount >= expectedCount) {
                // Consume expected
                this.marking[pid] -= expectedCount;
                this.tokensConsumed += expectedCount;
                if (this.marking[pid] <= 0) {
                    delete this.marking[pid];
                }
            } else {
                // Missing final token
                const diff = expectedCount - currentCount;
                this.tokensMissing += diff;
                this.tokensConsumed += expectedCount; // Final consume counts as expected
                this.marking[pid] = 0;
                delete this.marking[pid];
            }
        }

        // Any leftovers in the net are remaining tokens
        this.tokensRemaining = Object.values(this.marking).reduce((a, b) => a + b, 0);
    }

    /**
     * Calculates the conformance fitness of the trace replayed.
     * Formula matches: f = 0.5 * (1 - m/c) + 0.5 * (1 - r/p)
     */
    calculateFitness() {
        const m = this.tokensMissing;
        const c = this.tokensConsumed || 1; // Prevent division by zero
        const r = this.tokensRemaining;
        const p = this.tokensProduced || 1;

        const fitness = 0.5 * (1 - m / c) + 0.5 * (1 - r / p);
        return Math.max(0, Math.min(1, fitness)); // Bound between 0.0 and 1.0
    }

    /**
     * Runs a complete trace replayer synchronously and outputs results.
     * @param {Array<string>} trace - Event activity sequence
     */
    replayTrace(trace) {
        this.reset();
        const stepLogs = [];
        
        for (const act of trace) {
            const steps = this.stepEvent(act);
            stepLogs.push(...steps);
        }
        
        this.terminateReplay();

        return {
            trace_id: "trace_" + Math.random().toString(36).substring(2, 9),
            activity_sequence: trace,
            tokens_produced: this.tokensProduced,
            tokens_consumed: this.tokensConsumed,
            tokens_missing: this.tokensMissing,
            tokens_remaining: this.tokensRemaining,
            fitness: this.calculateFitness(),
            steps: stepLogs
        };
    }
}

class PetriNetVisualizer {
    /**
     * @param {HTMLElement} container - Canvas wrapper div element to replace canvas with SVG
     * @param {PetriNetSimulator} simulator - The Petri net simulator engine
     */
    constructor(container, simulator) {
        this.container = container;
        this.simulator = simulator;
        this.svg = null;
        this.speedMs = 500; // Animation tick speed

        this.initSVG();
    }

    /**
     * Initializes the SVG element in the container, hiding any old canvas.
     */
    initSVG() {
        // Find and hide canvas if it exists
        const oldCanvas = this.container.querySelector('canvas');
        if (oldCanvas) {
            oldCanvas.style.display = 'none';
        }

        // Remove any old SVG
        const oldSvg = this.container.querySelector('svg');
        if (oldSvg) {
            oldSvg.remove();
        }

        // Create new SVG
        const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("class", "net-svg");
        svg.setAttribute("viewBox", "0 0 940 300");
        svg.style.width = "100%";
        svg.style.height = "100%";

        // Add Marker definition for Arc Arrows
        const defs = document.createElementNS("http://www.w3.org/2000/svg", "defs");
        const marker = document.createElementNS("http://www.w3.org/2000/svg", "marker");
        marker.setAttribute("id", "arrow");
        marker.setAttribute("viewBox", "0 0 10 10");
        marker.setAttribute("refX", "8");
        marker.setAttribute("refY", "5");
        marker.setAttribute("markerWidth", "6");
        marker.setAttribute("markerHeight", "6");
        marker.setAttribute("orient", "auto-start-reverse");

        const markerPath = document.createElementNS("http://www.w3.org/2000/svg", "path");
        markerPath.setAttribute("d", "M 0 1.5 L 8 5 L 0 8.5 z");
        markerPath.setAttribute("fill", "#64748b");
        marker.appendChild(markerPath);
        defs.appendChild(marker);
        svg.appendChild(defs);

        this.svg = svg;
        this.container.appendChild(svg);

        // Draw structural elements
        this.drawStructure();
        this.updateMarkingDisplay();
    }

    /**
     * Sets the animation speed in milliseconds.
     */
    setSpeed(ms) {
        this.speedMs = ms;
    }

    /**
     * Calculates the intersection point of a line with the boundary of a Place (circle) or Transition (rect).
     */
    getConnectionPoint(fromNode, toNode, isFromTransition, isToTransition) {
        const x1 = fromNode.x;
        const y1 = fromNode.y;
        const x2 = toNode.x;
        const y2 = toNode.y;

        const dx = x2 - x1;
        const dy = y2 - y1;
        const len = Math.sqrt(dx * dx + dy * dy);
        if (len === 0) return { x1, y1, x2, y2 };

        const ux = dx / len;
        const uy = dy / len;

        // Radius for place circle
        const r = 22;

        let startX = x1;
        let startY = y1;
        let endX = x2;
        let endY = y2;

        // 1. Calculate Start Point
        if (isFromTransition) {
            // From transition rect (approximated border offset)
            const w = fromNode.width || 50;
            const h = fromNode.height || 50;
            // Project offset on rect edges
            const scaleX = Math.abs(ux) > 0.001 ? (w / 2) / Math.abs(ux) : Infinity;
            const scaleY = Math.abs(uy) > 0.001 ? (h / 2) / Math.abs(uy) : Infinity;
            const scale = Math.min(scaleX, scaleY);
            startX = x1 + ux * scale;
            startY = y1 + uy * scale;
        } else {
            // From place circle
            startX = x1 + ux * r;
            startY = y1 + uy * r;
        }

        // 2. Calculate End Point
        if (isToTransition) {
            // To transition rect (approximated border offset)
            const w = toNode.width || 50;
            const h = toNode.height || 50;
            const scaleX = Math.abs(ux) > 0.001 ? (w / 2) / Math.abs(ux) : Infinity;
            const scaleY = Math.abs(uy) > 0.001 ? (h / 2) / Math.abs(uy) : Infinity;
            const scale = Math.min(scaleX, scaleY);
            endX = x2 - ux * scale;
            endY = y2 - uy * scale;
        } else {
            // To place circle
            endX = x2 - ux * r;
            endY = y2 - uy * r;
        }

        return { startX, startY, endX, endY };
    }

    /**
     * Draws the static places, transitions, arcs, and text labels.
     */
    drawStructure() {
        const net = this.simulator.net;

        // 1. Draw Arcs
        net.arcs.forEach(arc => {
            const fromPlace = net.places[arc.from];
            const fromTrans = net.transitions[arc.from];
            const toPlace = net.places[arc.to];
            const toTrans = net.transitions[arc.to];

            const nodeFrom = fromPlace || fromTrans;
            const nodeTo = toPlace || toTrans;

            const isFromTransition = !!fromTrans;
            const isToTransition = !!toTrans;

            const pts = this.getConnectionPoint(nodeFrom, nodeTo, isFromTransition, isToTransition);

            const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
            path.setAttribute("class", "net-arc");
            
            // Build curved arc if connecting to check or skip to make them separate beautifully
            if (arc.to === 't_check') {
                const ctrlX = (pts.startX + pts.endX) / 2 - 10;
                const ctrlY = (pts.startY + pts.endY) / 2 - 15;
                path.setAttribute("d", `M ${pts.startX} ${pts.startY} Q ${ctrlX} ${ctrlY} ${pts.endX} ${pts.endY}`);
            } else if (arc.to === 't_skip') {
                const ctrlX = (pts.startX + pts.endX) / 2 - 10;
                const ctrlY = (pts.startY + pts.endY) / 2 + 15;
                path.setAttribute("d", `M ${pts.startX} ${pts.startY} Q ${ctrlX} ${ctrlY} ${pts.endX} ${pts.endY}`);
            } else if (arc.from === 't_check') {
                const ctrlX = (pts.startX + pts.endX) / 2 + 10;
                const ctrlY = (pts.startY + pts.endY) / 2 - 15;
                path.setAttribute("d", `M ${pts.startX} ${pts.startY} Q ${ctrlX} ${ctrlY} ${pts.endX} ${pts.endY}`);
            } else if (arc.from === 't_skip') {
                const ctrlX = (pts.startX + pts.endX) / 2 + 10;
                const ctrlY = (pts.startY + pts.endY) / 2 + 15;
                path.setAttribute("d", `M ${pts.startX} ${pts.startY} Q ${ctrlX} ${ctrlY} ${pts.endX} ${pts.endY}`);
            } else {
                // Straight path
                path.setAttribute("d", `M ${pts.startX} ${pts.startY} L ${pts.endX} ${pts.endY}`);
            }

            path.setAttribute("id", `arc-${arc.from}-${arc.to}`);
            this.svg.appendChild(path);
        });

        // 2. Draw Places
        Object.entries(net.places).forEach(([pid, p]) => {
            const group = document.createElementNS("http://www.w3.org/2000/svg", "g");
            group.setAttribute("id", `place-group-${pid}`);

            // Outer place circle
            const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
            circle.setAttribute("class", "net-place");
            circle.setAttribute("cx", p.x);
            circle.setAttribute("cy", p.y);
            circle.setAttribute("r", "22");
            group.appendChild(circle);

            // Place Label
            const labelText = document.createElementNS("http://www.w3.org/2000/svg", "text");
            labelText.setAttribute("x", p.x);
            labelText.setAttribute("y", p.y + 40);
            labelText.setAttribute("text-anchor", "middle");
            labelText.setAttribute("fill", "#cbd5e1");
            labelText.style.fontSize = "11px";
            labelText.style.fontWeight = "600";
            labelText.style.letterSpacing = "0.02em";
            labelText.textContent = p.label;
            group.appendChild(labelText);

            // Container for tokens inside the place
            const tokenContainer = document.createElementNS("http://www.w3.org/2000/svg", "g");
            tokenContainer.setAttribute("class", "place-tokens");
            group.appendChild(tokenContainer);

            this.svg.appendChild(group);
        });

        // 3. Draw Transitions
        Object.entries(net.transitions).forEach(([tid, t]) => {
            const group = document.createElementNS("http://www.w3.org/2000/svg", "g");
            group.setAttribute("id", `trans-group-${tid}`);

            // Transition Rect
            const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
            rect.setAttribute("class", "net-transition");
            rect.setAttribute("x", t.x - t.width / 2);
            rect.setAttribute("y", t.y - t.height / 2);
            rect.setAttribute("width", t.width);
            rect.setAttribute("height", t.height);
            rect.setAttribute("rx", "4");

            // Custom styling for silent transitions
            if (t.label.startsWith('tau')) {
                rect.style.strokeDasharray = "4,4";
                rect.style.opacity = "0.75";
                rect.style.fill = "#0b0f19";
            }
            group.appendChild(rect);

            // Transition Label
            const labelText = document.createElementNS("http://www.w3.org/2000/svg", "text");
            labelText.setAttribute("x", t.x);
            // Position above or below depending on alignment
            const isTopBranch = t.id === 't_check';
            labelText.setAttribute("y", isTopBranch ? t.y - 12 : t.y + t.height / 2 + 15);
            labelText.setAttribute("text-anchor", "middle");
            labelText.setAttribute("fill", t.label.startsWith('tau') ? "#64748b" : "#cbd5e1");
            labelText.style.fontSize = "10px";
            labelText.style.fontWeight = "500";
            
            // Format labels beautifully (greek letter tau for silent bypasses)
            labelText.textContent = t.label.startsWith('tau') ? "τ (Bypass Check)" : t.label;
            group.appendChild(labelText);

            this.svg.appendChild(group);
        });
    }

    /**
     * Updates the SVG display of tokens in places according to the current markings.
     */
    updateMarkingDisplay() {
        const net = this.simulator.net;
        const currentMarking = this.simulator.marking;

        Object.entries(net.places).forEach(([pid, p]) => {
            const tokenGroup = this.svg.querySelector(`#place-group-${pid} .place-tokens`);
            if (!tokenGroup) return;

            // Clear previous tokens
            tokenGroup.innerHTML = "";

            const count = currentMarking[pid] || 0;
            if (count === 1) {
                // Render one clean central dot
                const dot = document.createElementNS("http://www.w3.org/2000/svg", "circle");
                dot.setAttribute("class", "net-token");
                dot.setAttribute("cx", p.x);
                dot.setAttribute("cy", p.y);
                dot.setAttribute("r", "5");
                tokenGroup.appendChild(dot);
            } else if (count > 1 && count <= 3) {
                // Render small cluster of dots
                const offsets = [
                    { dx: -4, dy: -2 },
                    { dx: 4, dy: -2 },
                    { dx: 0, dy: 5 }
                ];
                for (let i = 0; i < count; i++) {
                    const dot = document.createElementNS("http://www.w3.org/2000/svg", "circle");
                    dot.setAttribute("class", "net-token");
                    dot.setAttribute("cx", p.x + offsets[i].dx);
                    dot.setAttribute("cy", p.y + offsets[i].dy);
                    dot.setAttribute("r", "4");
                    tokenGroup.appendChild(dot);
                }
            } else if (count > 3) {
                // Render numeric badge for large markings
                const badgeBg = document.createElementNS("http://www.w3.org/2000/svg", "circle");
                badgeBg.setAttribute("cx", p.x);
                badgeBg.setAttribute("cy", p.y);
                badgeBg.setAttribute("r", "10");
                badgeBg.setAttribute("fill", "#00f2fe");
                tokenGroup.appendChild(badgeBg);

                const badgeText = document.createElementNS("http://www.w3.org/2000/svg", "text");
                badgeText.setAttribute("x", p.x);
                badgeText.setAttribute("y", p.y + 3);
                badgeText.setAttribute("text-anchor", "middle");
                badgeText.setAttribute("fill", "#080c14");
                badgeText.style.fontSize = "9px";
                badgeText.style.fontWeight = "800";
                badgeText.textContent = count;
                tokenGroup.appendChild(badgeText);
            }
        });
    }

    /**
     * Executes the premium two-stage token firing animation.
     * Stage 1: Tokens slide from preset places to transition center.
     * Stage 2: Transition flashes `.firing`.
     * Stage 3: Tokens slide from transition center to postset places.
     * 
     * @param {string} tid - Transition ID
     * @param {Function} callback - Triggered when the entire animation lifecycle completes
     */
    animateTransitionFiring(tid, callback) {
        const net = this.simulator.net;
        const t = net.transitions[tid];
        if (!t) {
            if (callback) callback();
            return;
        }

        const animSpeed = this.speedMs * 0.45; // Split speed between stage 1 & 2
        const flashDuration = this.speedMs * 0.1;

        const transGroup = this.svg.querySelector(`#trans-group-${tid} rect`);
        
        // --- STAGE 1: Animate Tokens moving from PRESETS to TRANSITION ---
        const stage1Tokens = [];
        t.preset.forEach(pid => {
            const p = net.places[pid];
            // Render animating token circle
            const tCircle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
            tCircle.setAttribute("class", "net-token");
            tCircle.setAttribute("r", "5");
            tCircle.setAttribute("cx", p.x);
            tCircle.setAttribute("cy", p.y);
            this.svg.appendChild(tCircle);
            stage1Tokens.push(tCircle);
        });

        // Trigger transition start on next frame
        requestAnimationFrame(() => {
            stage1Tokens.forEach(tCircle => {
                tCircle.style.transition = `cx ${animSpeed}ms cubic-bezier(0.25, 0.46, 0.45, 0.94), cy ${animSpeed}ms cubic-bezier(0.25, 0.46, 0.45, 0.94)`;
                tCircle.setAttribute("cx", t.x);
                tCircle.setAttribute("cy", t.y);
            });
        });

        // --- STAGE 2: Highlight Transition (Flash Firing) ---
        setTimeout(() => {
            // Clean up stage 1 tokens
            stage1Tokens.forEach(tok => tok.remove());

            // Add firing indicator class
            if (transGroup) {
                transGroup.classList.add('firing');
            }

            // Flash transition
            setTimeout(() => {
                if (transGroup) {
                    transGroup.classList.remove('firing');
                }

                // --- STAGE 3: Animate Tokens moving from TRANSITION to POSTSETS ---
                const stage3Tokens = [];
                t.postset.forEach(pid => {
                    const p = net.places[pid];
                    const tCircle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
                    tCircle.setAttribute("class", "net-token");
                    tCircle.setAttribute("r", "5");
                    tCircle.setAttribute("cx", t.x);
                    tCircle.setAttribute("cy", t.y);
                    this.svg.appendChild(tCircle);
                    stage3Tokens.push({ el: tCircle, targetX: p.x, targetY: p.y });
                });

                requestAnimationFrame(() => {
                    stage3Tokens.forEach(item => {
                        item.el.style.transition = `cx ${animSpeed}ms cubic-bezier(0.25, 0.46, 0.45, 0.94), cy ${animSpeed}ms cubic-bezier(0.25, 0.46, 0.45, 0.94)`;
                        item.el.setAttribute("cx", item.targetX);
                        item.el.setAttribute("cy", item.targetY);
                    });
                });

                // Complete and refresh markings
                setTimeout(() => {
                    stage3Tokens.forEach(item => item.el.remove());
                    this.updateMarkingDisplay();
                    if (callback) callback();
                }, animSpeed);

            }, flashDuration);

        }, animSpeed);
    }
}

// Export modules for app controller usage
window.PetriNetSimulator = PetriNetSimulator;
window.PetriNetVisualizer = PetriNetVisualizer;
