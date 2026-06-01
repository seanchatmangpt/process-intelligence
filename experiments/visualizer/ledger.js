/**
 * ==========================================================================
 * SHA-256 Tamper-Evident Process Ledger
 * ==========================================================================
 */

// Pure JavaScript SHA-256 Implementation
function sha256(ascii) {
  function rightRotate(value, amount) {
    return (value >>> amount) | (value << (32 - amount));
  }
  
  var h = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
  var k = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
  ];

  var msg = ascii;
  var l = msg.length;
  var words = [];
  for (var i = 0; i < l; i++) {
    words[i >> 2] |= (msg.charCodeAt(i) & 0xff) << (24 - (i % 4) * 8);
  }
  
  var bitLen = l * 8;
  words[bitLen >> 5] |= 0x80 << (24 - (bitLen % 32));
  
  var blockCount = ((bitLen + 64) >> 9) + 1;
  var wordCount = blockCount * 16;
  while (words.length < wordCount) {
    words.push(0);
  }
  words[wordCount - 2] = Math.floor(bitLen / 0x100000000);
  words[wordCount - 1] = bitLen & 0xffffffff;
  
  for (var chunk = 0; chunk < words.length; chunk += 16) {
    var w = new Array(64);
    for (var i = 0; i < 16; i++) {
      w[i] = words[chunk + i];
    }
    for (var i = 16; i < 64; i++) {
      var s0 = rightRotate(w[i - 15], 7) ^ rightRotate(w[i - 15], 18) ^ (w[i - 15] >>> 3);
      var s1 = rightRotate(w[i - 2], 17) ^ rightRotate(w[i - 2], 19) ^ (w[i - 2] >>> 10);
      w[i] = (w[i - 16] + s0 + w[i - 7] + s1) | 0;
    }
    
    var a = h[0], b = h[1], c = h[2], d = h[3], e = h[4], f = h[5], g = h[6], h_val = h[7];
    
    for (var i = 0; i < 64; i++) {
      var S1 = rightRotate(e, 6) ^ rightRotate(e, 11) ^ rightRotate(e, 25);
      var ch = (e & f) ^ ((~e) & g);
      var temp1 = (h_val + S1 + ch + k[i] + w[i]) | 0;
      
      var S0 = rightRotate(a, 2) ^ rightRotate(a, 13) ^ rightRotate(a, 22);
      var maj = (a & b) ^ (a & c) ^ (b & c);
      var temp2 = (S0 + maj) | 0;
      
      h_val = g;
      g = f;
      f = e;
      e = (d + temp1) | 0;
      d = c;
      c = b;
      b = a;
      a = (temp1 + temp2) | 0;
    }
    
    h[0] = (h[0] + a) | 0;
    h[1] = (h[1] + b) | 0;
    h[2] = (h[2] + c) | 0;
    h[3] = (h[3] + d) | 0;
    h[4] = (h[4] + e) | 0;
    h[5] = (h[5] + f) | 0;
    h[6] = (h[6] + g) | 0;
    h[7] = (h[7] + h_val) | 0;
  }
  
  var hex = "";
  for (var i = 0; i < 8; i++) {
    var val = h[i];
    if (val < 0) val += 0x100000000;
    var str = val.toString(16);
    while (str.length < 8) str = "0" + str;
    hex += str;
  }
  return hex;
}

class Block {
  constructor(index, timestamp, caseId, activity, payload, prevHash) {
    this.index = index;
    this.timestamp = timestamp;
    this.caseId = caseId;
    this.activity = activity;
    this.payload = payload; // JS Object
    this.prevHash = prevHash;
    this.hash = this.calculateHash();
  }

  calculateHash() {
    const dataStr = 
      this.index + 
      this.timestamp + 
      this.caseId + 
      this.activity + 
      JSON.stringify(this.payload) + 
      this.prevHash;
    return sha256(dataStr);
  }
}

class ProcessLedger {
  constructor() {
    this.chain = [];
    this.onIntegrityChanged = null;
    this.onLogMessage = null;
    this.init();
  }

  init() {
    this.chain = [];
    // Create genesis block
    const genesis = new Block(
      0,
      new Date().toLocaleTimeString(),
      "C-GENESIS",
      "Initialize Ledger",
      { note: "Process Intelligence Blockchain Started" },
      "0".repeat(64)
    );
    this.chain.push(genesis);
    this.log("Genesis Block established.");
  }

  log(msg) {
    if (this.onLogMessage) {
      this.onLogMessage(`[Ledger] ${msg}`);
    }
  }

  addEvent(caseId, activity, payload = {}) {
    const prevBlock = this.chain[this.chain.length - 1];
    const newBlock = new Block(
      this.chain.length,
      new Date().toLocaleTimeString(),
      caseId,
      activity,
      payload,
      prevBlock.hash
    );
    this.chain.push(newBlock);
    this.log(`Block #${newBlock.index} added [${caseId} - ${activity}]`);
    this.verifyAndRender();
  }

  tamperBlock(index, newActivity) {
    if (index < 0 || index >= this.chain.length) return;
    this.chain[index].activity = newActivity;
    this.log(`🚨 TAMPER ALERT: Block #${index} modified to "${newActivity}"!`);
    this.verifyAndRender();
  }

  repairChain() {
    this.log("⚙️ Attempting cryptographic ledger repair...");
    for (let i = 1; i < this.chain.length; i++) {
      this.chain[i].prevHash = this.chain[i - 1].hash;
      this.chain[i].hash = this.chain[i].calculateHash();
    }
    this.log("✅ Cryptographic chain fully re-signed and repaired.");
    this.verifyAndRender();
  }

  validateChain() {
    for (let i = 0; i < this.chain.length; i++) {
      const block = this.chain[i];
      // Check stored hash matches computed hash
      if (block.hash !== block.calculateHash()) {
        return { isValid: false, failedIndex: i, reason: "Hash mismatch (data modified)" };
      }
      // Check chaining
      if (i > 0) {
        const prevBlock = this.chain[i - 1];
        if (block.prevHash !== prevBlock.hash) {
          return { isValid: false, failedIndex: i, reason: "Previous hash pointer broken" };
        }
      }
    }
    return { isValid: true, failedIndex: null, reason: null };
  }

  verifyAndRender() {
    const { isValid, failedIndex } = this.validateChain();
    
    // Update global indicators
    const integritySpan = document.getElementById("metric-completed-cases") ? document.getElementById("metric-ledger-integrity") : null;
    const visualStatus = document.getElementById("ledger-visual-status");
    const stateBadge = document.getElementById("ledger-state-badge");
    const repairBtn = document.getElementById("btn-ledger-restore");
    const countSpan = document.getElementById("ledger-block-count");
    const latestHashSpan = document.getElementById("ledger-latest-hash");
    
    if (countSpan) countSpan.textContent = this.chain.length;
    if (latestHashSpan) {
      const lastHash = this.chain[this.chain.length - 1].hash;
      latestHashSpan.textContent = lastHash.substring(0, 16) + "...";
      latestHashSpan.title = lastHash;
    }

    if (integritySpan) {
      if (isValid) {
        integritySpan.textContent = "VALID";
        integritySpan.className = "metric-value status-text-success";
        if (visualStatus) visualStatus.className = "metric-visual gold";
        if (stateBadge) {
          stateBadge.textContent = "VALID";
          stateBadge.className = "badge badge-success";
        }
        if (repairBtn) repairBtn.style.display = "none";
      } else {
        integritySpan.textContent = "CORRUPTED";
        integritySpan.className = "metric-value status-text-tampered";
        if (visualStatus) visualStatus.className = "metric-visual gold tampered-visual";
        if (stateBadge) {
          stateBadge.textContent = "TAMPERED";
          stateBadge.className = "badge badge-danger";
        }
        if (repairBtn) repairBtn.style.display = "inline-block";
      }
    }

    if (this.onIntegrityChanged) {
      this.onIntegrityChanged(isValid, failedIndex);
    }

    this.render();
  }

  render() {
    const container = document.getElementById("ledger-chain-view");
    if (!container) return;
    container.innerHTML = "";

    const { isValid, failedIndex } = this.validateChain();
    const isBlockValid = (index) => {
      if (isValid) return true;
      return index < failedIndex;
    };

    this.chain.forEach((block, index) => {
      const blockEl = document.createElement("div");
      const valid = isBlockValid(index);
      blockEl.className = `ledger-block ${valid ? 'valid' : 'invalid'}`;
      
      blockEl.innerHTML = `
        <div class="block-header">
          <span class="block-id">BLOCK #${block.index}</span>
          <span class="block-time">${block.timestamp}</span>
        </div>
        <div class="block-body">
          <div class="block-activity" title="${block.activity}">${block.activity}</div>
          <div class="block-case">${block.caseId}</div>
          <span class="block-hash-label">Prev Hash</span>
          <div class="block-hash">${block.prevHash.substring(0, 8)}...</div>
          <span class="block-hash-label">Hash</span>
          <div class="block-hash" style="color: ${valid ? '#10b981' : '#ef4444'}">${block.hash.substring(0, 8)}...</div>
        </div>
        <div class="block-actions">
          ${block.index > 0 ? `<button class="btn-tamper" data-index="${block.index}">Tamper Block</button>` : ''}
        </div>
      `;
      
      container.appendChild(blockEl);
    });

    // Scroll to end of ledger container
    container.scrollLeft = container.scrollWidth;

    // Attach listeners
    container.querySelectorAll(".btn-tamper").forEach(btn => {
      btn.addEventListener("click", (e) => {
        const idx = parseInt(e.target.getAttribute("data-index"));
        const act = prompt("Modify block activity to simulate attack:", "Unauthorized Refund");
        if (act !== null) {
          this.tamperBlock(idx, act);
        }
      });
    });
  }
}

// Global initialization
window.ledgerEngine = new ProcessLedger();
document.getElementById("btn-ledger-verify")?.addEventListener("click", () => {
  const result = window.ledgerEngine.validateChain();
  if (result.isValid) {
    alert("✅ Cryptographic validation successful: Ledger is intact.");
  } else {
    alert(`❌ LEDGER COMPROMISED: Failed validation at block #${result.failedIndex}. Reason: ${result.reason}`);
  }
});
document.getElementById("btn-ledger-restore")?.addEventListener("click", () => {
  window.ledgerEngine.repairChain();
});
