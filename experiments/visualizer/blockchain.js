/**
 * blockchain.js
 * Cryptographic Audit Ledger using SHA-256 Event Chaining
 * 
 * Verifies process integrity by hashing events in sequence. Any modification to a
 * past event will break the hash chain, showing a tamper violation in the audit ledger.
 */

class CryptographicAuditChain {
    constructor() {
        this.chain = [];
        this.genesisHash = '0000000000000000000000000000000000000000000000000000000000000000';
    }

    /**
     * Resets the cryptographic audit chain.
     */
    reset() {
        this.chain = [];
    }

    /**
     * Appends an event to the chain, linking it to the previous event's hash.
     * @param {string} caseId - Process case ID.
     * @param {string} activity - Process activity name.
     * @param {string} executor - User or system executing the step.
     * @param {Object} extraData - Optional additional parameters (e.g., duration, compliance).
     * @returns {Object} The created block.
     */
    addEvent(caseId, activity, executor, extraData = {}) {
        const index = this.chain.length;
        const timestamp = new Date().toISOString();
        const prevBlock = index > 0 ? this.chain[index - 1] : null;
        const previousHash = prevBlock ? prevBlock.hash : this.genesisHash;

        const blockData = {
            index,
            timestamp,
            caseId,
            activity,
            executor,
            extraData: { ...extraData },
            previousHash
        };

        const hash = CryptographicAuditChain.hashBlock(blockData);
        const block = { ...blockData, hash };
        
        this.chain.push(block);
        return block;
    }

    /**
     * Verifies the cryptographic integrity of the entire chain.
     * Checks that each block's previousHash matches the prior block's hash,
     * and that the current block's hash is validly computed.
     * @returns {Object} { isValid: boolean, errorBlockIndex: number|null }
     */
    verifyChain() {
        for (let i = 0; i < this.chain.length; i++) {
            const current = this.chain[i];
            const expectedPrevHash = i > 0 ? this.chain[i - 1].hash : this.genesisHash;

            if (current.previousHash !== expectedPrevHash) {
                return { isValid: false, errorBlockIndex: i, reason: 'Previous hash mismatch' };
            }

            const recomputedHash = CryptographicAuditChain.hashBlock(current);
            if (current.hash !== recomputedHash) {
                return { isValid: false, errorBlockIndex: i, reason: 'Tampered block hash' };
            }
        }
        return { isValid: true, errorBlockIndex: null };
    }

    /**
     * Artificially tampers with a block in the chain to demonstrate validation failures.
     */
    tamperBlock(index, field, newValue) {
        if (index < 0 || index >= this.chain.length) return false;
        
        const block = this.chain[index];
        if (field === 'activity') {
            block.activity = newValue;
        } else if (field === 'executor') {
            block.executor = newValue;
        } else if (field === 'timestamp') {
            block.timestamp = newValue;
        } else if (field === 'hash') {
            block.hash = newValue;
        } else {
            block.extraData[field] = newValue;
        }
        return true;
    }

    /**
     * Compute SHA-256 hash of a block data object.
     */
    static hashBlock(block) {
        // Stringify key fields of the block deterministically
        const dataString = JSON.stringify({
            index: block.index,
            timestamp: block.timestamp,
            caseId: block.caseId,
            activity: block.activity,
            executor: block.executor,
            extraData: block.extraData,
            previousHash: block.previousHash
        });

        return CryptographicAuditChain.sha256(dataString);
    }

    /**
     * Pure JS implementation of SHA-256 (synchronous and self-contained).
     */
    static sha256(ascii) {
        function rightRotate(value, amount) {
            return (value >>> amount) | (value << (32 - amount));
        }

        const mathPow = Math.pow;
        const maxWord = mathPow(2, 32);
        const lengthProperty = 'length';
        let i, j; // Used as a loop index.

        const words = [];
        const asciiLength = ascii[lengthProperty];
        
        // Initial hash values
        let h0 = 0x6a09e667;
        let h1 = 0xbb67ae85;
        let h2 = 0x3c6ef372;
        let h3 = 0xa54ff53a;
        let h4 = 0x510e527f;
        let h5 = 0x9b05688c;
        let h6 = 0x1f83d9ab;
        let h7 = 0x5be0cd19;

        // Fractional parts of prime numbers
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

        // Pre-processing
        const bits = asciiLength * 8;
        let s = ascii + "\x80";
        while (s[lengthProperty] % 64 !== 56) {
            s += "\x00";
        }
        
        for (i = 0; i < s[lengthProperty]; i++) {
            words[i >> 2] |= s.charCodeAt(i) << (24 - (i % 4) * 8);
        }
        
        words[words[lengthProperty]] = ((bits / maxWord) | 0);
        words[words[lengthProperty]] = (bits | 0);

        // Process message in 512-bit blocks (16 words at a time)
        for (i = 0; i < words[lengthProperty]; i += 16) {
            const w = [];
            let a = h0;
            let b = h1;
            let c = h2;
            let d = h3;
            let e = h4;
            let f = h5;
            let g = h6;
            let h_val = h7;

            for (j = 0; j < 64; j++) {
                if (j < 16) {
                    w[j] = words[i + j];
                } else {
                    const s0 = rightRotate(w[j - 15], 7) ^ rightRotate(w[j - 15], 18) ^ (w[j - 15] >>> 3);
                    const s1 = rightRotate(w[j - 2], 17) ^ rightRotate(w[j - 2], 19) ^ (w[j - 2] >>> 10);
                    w[j] = (w[j - 16] + s0 + w[j - 7] + s1) | 0;
                }

                const s1 = rightRotate(e, 6) ^ rightRotate(e, 11) ^ rightRotate(e, 25);
                const ch = (e & f) ^ (~e & g);
                const temp1 = (h_val + s1 + ch + k[j] + (w[j] || 0)) | 0;
                const s0 = rightRotate(a, 2) ^ rightRotate(a, 13) ^ rightRotate(a, 22);
                const maj = (a & b) ^ (a & c) ^ (b & c);
                const temp2 = (s0 + maj) | 0;

                h_val = g;
                g = f;
                f = e;
                e = (d + temp1) | 0;
                d = c;
                c = b;
                b = a;
                a = (temp1 + temp2) | 0;
            }

            h0 = (h0 + a) | 0;
            h1 = (h1 + b) | 0;
            h2 = (h2 + c) | 0;
            h3 = (h3 + d) | 0;
            h4 = (h4 + e) | 0;
            h5 = (h5 + f) | 0;
            h6 = (h6 + g) | 0;
            h7 = (h7 + h_val) | 0;
        }

        const hashWords = [h0, h1, h2, h3, h4, h5, h6, h7];
        let hashStr = '';
        for (i = 0; i < 8; i++) {
            let hex = (hashWords[i] >>> 0).toString(16);
            while (hex.length < 8) {
                hex = '0' + hex;
            }
            hashStr += hex;
        }
        return hashStr;
    }
}

// Export for usage in ESModules or global window object
if (typeof module !== 'undefined' && module.exports) {
    module.exports = CryptographicAuditChain;
} else {
    window.CryptographicAuditChain = CryptographicAuditChain;
}
