/**
 * ledger.js
 * Cryptographic Event-Chain Explorer & Conformance Replay Receipt Verification System
 * 
 * Provides:
 * - SHA-256 hashing of workflow event sequences
 * - Cryptographic linking of events (blockchain hash-chaining)
 * - ECDSA P-256 key pair generation, signing, and verification of Conformance Receipts
 * - Interactive ledger visualizer rendering with tampering simulations and verification logs
 * 
 * References:
 * - Conformance Replay Receipt Schema: file:///Users/sac/process-intelligence/experiments/replay_receipt_sample.md
 * - Conformance Replay Doctrine: file:///Users/sac/process-intelligence/experiments/petri_conformance_sample.md
 * - Provenance Placements: file:///Users/sac/process-intelligence/standards/prov-o_provenance_placement.md
 */

const ProcessLedger = (() => {
    // Session keypair for ECDSA P-256 signatures
    let sessionKeyPair = null;

    /**
     * Calculates the SHA-256 hash of a string using Web Crypto API.
     */
    async function sha256(message) {
        const msgBuffer = new TextEncoder().encode(message);
        const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    }

    /**
     * Deterministically serializes an object to a string.
     * Ensures consistent signatures regardless of key ordering.
     */
    function serializeDeterministic(obj) {
        if (typeof obj !== 'object' || obj === null) {
            return JSON.stringify(obj);
        }
        if (Array.isArray(obj)) {
            return '[' + obj.map(serializeDeterministic).join(',') + ']';
        }
        const sortedKeys = Object.keys(obj).sort();
        return '{' + sortedKeys.map(k => JSON.stringify(k) + ':' + serializeDeterministic(obj[k])).join(',') + '}';
    }

    /**
     * Formats a block payload to be hashed deterministically.
     */
    function formatBlockString(block) {
        return `${block.index}|${block.timestamp}|${block.caseId}|${block.activity}|${block.resource}|${block.previousHash}|${serializeDeterministic(block.payload)}`;
    }

    /**
     * Initializes the ECDSA P-256 session keys.
     */
    async function initKeys() {
        if (!sessionKeyPair) {
            sessionKeyPair = await window.crypto.subtle.generateKey(
                {
                    name: "ECDSA",
                    namedCurve: "P-256"
                },
                true,
                ["sign", "verify"]
            );
        }
        return sessionKeyPair;
    }

    /**
     * Exports a public key in raw format to a hexadecimal string.
     */
    async function exportPublicKeyHex(publicKey) {
        const raw = await window.crypto.subtle.exportKey("raw", publicKey);
        return Array.from(new Uint8Array(raw)).map(b => b.toString(16).padStart(2, '0')).join('');
    }

    /**
     * Imports a public key from a hexadecimal string.
     */
    async function importPublicKeyFromHex(hexString) {
        const bytes = new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));
        return await window.crypto.subtle.importKey(
            "raw",
            bytes,
            {
                name: "ECDSA",
                namedCurve: "P-256"
            },
            true,
            ["verify"]
        );
    }

    /**
     * Creates an event chain from a raw list of event objects.
     */
    async function createChain(events) {
        const chain = [];
        let previousHash = "0000000000000000000000000000000000000000000000000000000000000000";

        for (let i = 0; i < events.length; i++) {
            const ev = events[i];
            const block = {
                index: i,
                timestamp: ev.timestamp || new Date().toISOString(),
                caseId: ev.caseId || "unknown_case",
                activity: ev.activity,
                resource: ev.resource || "system",
                payload: ev.payload || {},
                previousHash: previousHash,
                hash: ""
            };
            const blockStr = formatBlockString(block);
            block.hash = await sha256(blockStr);
            previousHash = block.hash;
            chain.push(block);
        }
        return chain;
    }

    /**
     * Verifies the cryptographic integrity of an event chain.
     * Identifies exactly which blocks are broken or tampered.
     */
    async function verifyChain(chain) {
        const results = [];
        let isChainBroken = false;

        for (let i = 0; i < chain.length; i++) {
            const block = chain[i];
            const calculatedHash = await sha256(formatBlockString(block));
            const isHashValid = (block.hash === calculatedHash);
            
            let isPrevHashValid = true;
            if (i > 0) {
                isPrevHashValid = (block.previousHash === chain[i - 1].hash);
            } else {
                isPrevHashValid = (block.previousHash === "0000000000000000000000000000000000000000000000000000000000000000");
            }

            const blockHealthy = isHashValid && isPrevHashValid && !isChainBroken;
            if (!blockHealthy) {
                isChainBroken = true; // Chain is broken from this point forward
            }

            results.push({
                index: i,
                hashValid: isHashValid,
                prevHashValid: isPrevHashValid,
                isValid: blockHealthy,
                calculatedHash: calculatedHash
            });
        }
        return {
            isValid: !isChainBroken,
            blockStatuses: results
        };
    }

    /**
     * Generates and cryptographically signs a Conformance Replay Receipt.
     */
    async function generateReceipt(chain, modelSha256, replayResults) {
        const keys = await initKeys();
        const pubKeyHex = await exportPublicKeyHex(keys.publicKey);
        
        const logSha256 = chain.length > 0 ? chain[chain.length - 1].hash : "0000000000000000000000000000000000000000000000000000000000000000";

        const receipt = {
            receipt_id: "rec_conformance_" + Math.random().toString(36).substring(2, 10) + "_" + Date.now().toString().slice(-5),
            timestamp: new Date().toISOString(),
            execution_authority: {
                engine_identifier: "wasm4pm-core-js-v2.1.0",
                wasm_module_sha256: "4a7b744ce58b88cd28148b5dfbe984f932e650b2a8f98db832cdde32bbd42a9d"
            },
            input_artifacts: {
                model_sha256: modelSha256 || "81f7dca25ba3594074888c74547b0e70796a2082f9cda3b2c12a843e620581ba9",
                log_sha256: logSha256
            },
            replay_results: {
                fitness_score: replayResults.fitness_score ?? 1.0,
                missing_tokens: replayResults.missing_tokens ?? 0,
                remaining_tokens: replayResults.remaining_tokens ?? 0,
                produced_tokens: replayResults.produced_tokens ?? 0,
                consumed_tokens: replayResults.consumed_tokens ?? 0
            }
        };

        // Sign the receipt content
        const serialized = serializeDeterministic(receipt);
        const encoder = new TextEncoder();
        const dataBytes = encoder.encode(serialized);
        
        const signatureBuffer = await window.crypto.subtle.sign(
            {
                name: "ECDSA",
                hash: { name: "SHA-256" }
            },
            keys.privateKey,
            dataBytes
        );

        const signatureBytesHex = Array.from(new Uint8Array(signatureBuffer))
            .map(b => b.toString(16).padStart(2, '0'))
            .join('');

        receipt.cryptographic_signature = {
            public_key: pubKeyHex,
            signature_bytes: signatureBytesHex
        };

        return receipt;
    }

    /**
     * Cryptographically verifies a Conformance Replay Receipt.
     * Validates both the signature and trace integrity against the active log.
     */
    async function verifyReceipt(receipt, chain) {
        const logs = [];
        logs.push(`[SYSTEM] Starting verification protocol for Receipt ID: ${receipt.receipt_id}`);

        try {
            const { cryptographic_signature, ...receiptData } = receipt;
            if (!cryptographic_signature || !cryptographic_signature.public_key || !cryptographic_signature.signature_bytes) {
                logs.push(`[ERROR] Verification aborted. Receipt missing cryptographic signature component.`);
                return { valid: false, logs, reason: "Missing cryptographic signature component." };
            }

            // 1. Verify Event-Chain Hash linkage to the Receipt
            const computedLogSha256 = chain.length > 0 ? chain[chain.length - 1].hash : "0000000000000000000000000000000000000000000000000000000000000000";
            const receiptLogSha256 = receipt.input_artifacts.log_sha256;
            logs.push(`[STEP 1] Validating trace boundary hash mapping...`);
            logs.push(`  - Expected Log Hash (Receipt): ${receiptLogSha256}`);
            logs.push(`  - Computed Log Hash (Active Chain): ${computedLogSha256}`);
            
            if (computedLogSha256 !== receiptLogSha256) {
                logs.push(`[ERROR] Integrity failure: Active trace final hash does not match receipt input artifacts. The log has been modified after receipt generation.`);
                return { valid: false, logs, reason: "Trace final hash mismatch. The event log has been modified." };
            }
            logs.push(`[SUCCESS] Trace hash boundary mapping verified.`);

            // 2. Verify Internal Chain Linkage
            logs.push(`[STEP 2] Performing full event-chain structural validation...`);
            const chainStatus = await verifyChain(chain);
            if (!chainStatus.isValid) {
                logs.push(`[ERROR] Structural failure: Internal hash chain is broken or tampered.`);
                return { valid: false, logs, reason: "Event chain internal structure is broken." };
            }
            logs.push(`[SUCCESS] Full event-chain structural integrity verified (0 tampering points detected).`);

            // 3. Cryptographically Verify Signature
            logs.push(`[STEP 3] Rebuilding deterministic serialization for signature validation...`);
            const serialized = serializeDeterministic(receiptData);
            const encoder = new TextEncoder();
            const dataBytes = encoder.encode(serialized);

            logs.push(`[STEP 4] Importing public key: ${cryptographic_signature.public_key.substring(0, 16)}...`);
            const pubKey = await importPublicKeyFromHex(cryptographic_signature.public_key);
            
            const sigBytes = new Uint8Array(
                cryptographic_signature.signature_bytes.match(/.{1,2}/g).map(b => parseInt(b, 16))
            );

            logs.push(`[STEP 5] Performing ECDSA P-256 verification...`);
            const signatureValid = await window.crypto.subtle.verify(
                {
                    name: "ECDSA",
                    hash: { name: "SHA-256" }
                },
                pubKey,
                sigBytes,
                dataBytes
            );

            if (signatureValid) {
                logs.push(`[SUCCESS] Cryptographic signature is mathematically authentic.`);
                logs.push(`[SYSTEM] Conformance Replay Receipt is 100% VALID. Provenance Placement established.`);
                return { valid: true, logs, reason: "Receipt verified successfully." };
            } else {
                logs.push(`[ERROR] Signature validation failed. Key authorization or signature payload mismatch.`);
                return { valid: false, logs, reason: "Invalid signature bytes." };
            }
        } catch (err) {
            logs.push(`[CRITICAL] Verification engine threw exception: ${err.message}`);
            return { valid: false, logs, reason: `Exception: ${err.message}` };
        }
    }

    /**
     * Renders the Cryptographic Event-Chain visualizer interface.
     */
    function renderLedgerExplorer(container, chain, verificationResult, activeReceipt, options = {}) {
        const { onTamper, onRestore, onSignReceipt, onVerifyReceipt } = options;
        
        // Clear container
        container.innerHTML = "";

        const wrapper = document.createElement("div");
        wrapper.className = "ledger-explorer-wrapper";

        // Title and metrics header
        const header = document.createElement("div");
        header.className = "ledger-header";
        
        const chainHealthy = verificationResult.isValid;
        header.innerHTML = `
            <div class="ledger-title-group">
                <h3>Cryptographic Event-Chain Explorer</h3>
                <div class="ledger-subtitle">
                    Verifiable cryptographic log chain leveraging SHA-256 links. 
                    Reference: <a href="file:///Users/sac/process-intelligence/experiments/replay_receipt_sample.md" target="_blank" class="std-link">replay_receipt_sample.md</a>
                </div>
            </div>
            <div class="ledger-status-badge ${chainHealthy ? 'healthy' : 'tampered'}">
                <span class="status-icon">${chainHealthy ? '✓' : '⚠'}</span>
                <span>${chainHealthy ? 'Ledger Intact' : 'Ledger Tampered'}</span>
            </div>
        `;
        wrapper.appendChild(header);

        // Chain Blocks View
        const blocksContainer = document.createElement("div");
        blocksContainer.className = "ledger-blocks-container";
        
        chain.forEach((block, index) => {
            const blockStatus = verificationResult.blockStatuses[index] || { isValid: true, hashValid: true, prevHashValid: true };
            const isBlockTampered = !blockStatus.isValid;

            const blockCard = document.createElement("div");
            blockCard.className = `ledger-block-card ${isBlockTampered ? 'tampered' : 'healthy'}`;
            
            blockCard.innerHTML = `
                <div class="block-index">BLOCK #${block.index}</div>
                <div class="block-main">
                    <div class="block-activity-row">
                        <span class="activity-label">Activity:</span>
                        <span class="activity-value text-glow">${block.activity}</span>
                    </div>
                    <div class="block-metadata">
                        <div><strong>Case ID:</strong> ${block.caseId}</div>
                        <div><strong>Resource:</strong> ${block.resource}</div>
                        <div><strong>Timestamp:</strong> ${new Date(block.timestamp).toLocaleTimeString()}</div>
                    </div>
                    <div class="block-hash-chain">
                        <div class="hash-row">
                            <span class="hash-label">Prev Hash:</span>
                            <span class="hash-value text-muted" title="${block.previousHash}">${block.previousHash.slice(0, 16)}...</span>
                        </div>
                        <div class="hash-row">
                            <span class="hash-label">Block Hash:</span>
                            <span class="hash-value ${blockStatus.hashValid ? 'text-success' : 'text-danger'}" title="${block.hash}">
                                ${block.hash.slice(0, 16)}...
                            </span>
                        </div>
                    </div>
                    <div class="block-payload">
                        <div class="payload-header" onclick="this.nextElementSibling.classList.toggle('expanded')">
                            <span>Payload Context</span>
                            <span class="chevron">▼</span>
                        </div>
                        <div class="payload-content">
                            <pre>${JSON.stringify(block.payload, null, 2)}</pre>
                        </div>
                    </div>
                </div>
                <div class="block-actions">
                    <button class="btn btn-sm btn-tamper" data-index="${index}">Tamper Data</button>
                </div>
            `;

            // Setup tamper action
            const tamperBtn = blockCard.querySelector(".btn-tamper");
            tamperBtn.addEventListener("click", () => {
                if (onTamper) onTamper(index);
            });

            blocksContainer.appendChild(blockCard);

            // Append arrow if not last block
            if (index < chain.length - 1) {
                const arrow = document.createElement("div");
                arrow.className = `ledger-chain-arrow ${isBlockTampered ? 'broken' : 'healthy'}`;
                arrow.innerHTML = `
                    <div class="arrow-line"></div>
                    <div class="arrow-head">▶</div>
                `;
                blocksContainer.appendChild(arrow);
            }
        });

        wrapper.appendChild(blocksContainer);

        // Control Panel (Tamper / Restore)
        const controlPanel = document.createElement("div");
        controlPanel.className = "ledger-controls-row";
        controlPanel.innerHTML = `
            <button class="btn btn-secondary btn-restore" ${chainHealthy ? 'disabled' : ''}>
                Restore Ledger Integrity
            </button>
            <div class="info-bubble">
                ${chainHealthy 
                    ? "✓ Every block hash matches its event content, and points properly to the preceding block's hash." 
                    : "⚠ The chain has been modified. Replay Receipts generated under this state will fail verification."}
            </div>
        `;
        
        controlPanel.querySelector(".btn-restore").addEventListener("click", () => {
            if (onRestore) onRestore();
        });
        wrapper.appendChild(controlPanel);

        // Receipt Section
        const receiptSection = document.createElement("div");
        receiptSection.className = "ledger-receipt-section";
        receiptSection.innerHTML = `
            <div class="section-divider"></div>
            <div class="receipt-layout-grid">
                <div class="receipt-generation-panel">
                    <h4>Conformance Replay Receipt Builder</h4>
                    <p class="receipt-desc">
                        Certify the results of your token game execution by sealing them in a cryptographic receipt. 
                        Reference: <a href="file:///Users/sac/process-intelligence/experiments/petri_conformance_sample.md" target="_blank" class="std-link">petri_conformance_sample.md</a>
                    </p>
                    <div class="receipt-actions">
                        <button class="btn btn-primary btn-generate-receipt">Generate & Sign Receipt</button>
                    </div>
                    ${activeReceipt ? `
                        <div class="receipt-display-box">
                            <div class="receipt-json-header">
                                <span>SIGNED RECEIPT JSON</span>
                                <span class="receipt-badge">ECDSA P-256</span>
                            </div>
                            <pre class="receipt-json-body">${JSON.stringify(activeReceipt, null, 2)}</pre>
                        </div>
                    ` : `
                        <div class="receipt-placeholder">
                            No active receipt. Click "Generate & Sign Receipt" to create a cryptographically signed proof.
                        </div>
                    `}
                </div>
                
                <div class="receipt-verification-panel">
                    <h4>Receipt Verification Engine</h4>
                    <p class="receipt-desc">
                        Load and run the cryptographic verification pipeline to validate the authenticity of the active receipt.
                    </p>
                    <div class="receipt-actions">
                        <button class="btn btn-success btn-verify-receipt" ${!activeReceipt ? 'disabled' : ''}>
                            Verify Active Receipt
                        </button>
                    </div>
                    <div class="verification-console">
                        <div class="console-title">VERIFICATION ENGINE OUTPUT LOGS</div>
                        <div class="console-logs" id="verificationConsoleLogs">
                            <span class="text-muted">Console idle. Awaiting receipt verification request...</span>
                        </div>
                    </div>
                </div>
            </div>
        `;

        receiptSection.querySelector(".btn-generate-receipt").addEventListener("click", () => {
            if (onSignReceipt) onSignReceipt();
        });

        if (activeReceipt) {
            receiptSection.querySelector(".btn-verify-receipt").addEventListener("click", () => {
                if (onVerifyReceipt) onVerifyReceipt();
            });
        }

        wrapper.appendChild(receiptSection);
        container.appendChild(wrapper);
    }

    return {
        sha256,
        serializeDeterministic,
        createChain,
        verifyChain,
        generateReceipt,
        verifyReceipt,
        renderLedgerExplorer,
        initKeys
    };
})();

// Export globally for browser use
window.ProcessLedger = ProcessLedger;
