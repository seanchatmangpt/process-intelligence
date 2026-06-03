// Complete, self-contained cryptographic algorithms: SHA-256, SHA-512, ChaCha20, BLAKE3, Curve25519, Ed25519, and JCS verification.

pub struct Sha256 {
    pub state: [u32; 8],
    pub buffer: [u8; 64],
    pub len: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    pub fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buffer: [0; 64],
            len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        let buffer_len = (self.len % 64) as usize;
        self.len += data.len() as u64;

        if buffer_len > 0 {
            let fill = 64 - buffer_len;
            if data.len() >= fill {
                self.buffer[buffer_len..64].copy_from_slice(&data[..fill]);
                let chunk = self.buffer;
                self.transform(&chunk);
                data = &data[fill..];
            } else {
                self.buffer[buffer_len..buffer_len + data.len()].copy_from_slice(data);
                return;
            }
        }

        while data.len() >= 64 {
            let chunk: &[u8; 64] = data[..64].try_into().unwrap();
            self.transform(chunk);
            data = &data[64..];
        }

        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        let buffer_len = (self.len % 64) as usize;
        self.buffer[buffer_len] = 0x80;
        
        if buffer_len + 1 > 56 {
            self.buffer[buffer_len + 1..64].fill(0);
            let chunk = self.buffer;
            self.transform(&chunk);
            self.buffer[..56].fill(0);
        } else {
            self.buffer[buffer_len + 1..56].fill(0);
        }

        let bits = (self.len * 8).to_be_bytes();
        self.buffer[56..64].copy_from_slice(&bits);
        let chunk = self.buffer;
        self.transform(&chunk);

        let mut out = [0u8; 32];
        for i in 0..8 {
            out[i * 4..i * 4 + 4].copy_from_slice(&self.state[i].to_be_bytes());
        }
        out
    }

    fn transform(&mut self, chunk: &[u8; 64]) {
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];

        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

pub struct ChaCha20 {
    pub state: [u32; 16],
}

impl ChaCha20 {
    pub fn new(key: &[u8; 32], nonce: &[u8; 12]) -> Self {
        let mut state = [0u32; 16];
        state[0] = 0x61737865; // "exsa" constants
        state[1] = 0x3320646e; // "nd 3"
        state[2] = 0x79622d32; // "2-by"
        state[3] = 0x6b206574; // "te k"
        
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                key[i * 4],
                key[i * 4 + 1],
                key[i * 4 + 2],
                key[i * 4 + 3],
            ]);
        }
        state[12] = 0; // counter
        for i in 0..3 {
            state[13 + i] = u32::from_le_bytes([
                nonce[i * 4],
                nonce[i * 4 + 1],
                nonce[i * 4 + 2],
                nonce[i * 4 + 3],
            ]);
        }
        Self { state }
    }

    pub fn next_block(&mut self) -> [u8; 64] {
        let mut x = self.state;
        for _ in 0..10 {
            // Column rounds
            quarter_round(&mut x, 0, 4, 8, 12);
            quarter_round(&mut x, 1, 5, 9, 13);
            quarter_round(&mut x, 2, 6, 10, 14);
            quarter_round(&mut x, 3, 7, 11, 15);
            // Diagonal rounds
            quarter_round(&mut x, 0, 5, 10, 15);
            quarter_round(&mut x, 1, 6, 11, 12);
            quarter_round(&mut x, 2, 7, 8, 13);
            quarter_round(&mut x, 3, 4, 9, 14);
        }

        let mut out = [0u8; 64];
        for i in 0..16 {
            let val = x[i].wrapping_add(self.state[i]);
            let bytes = val.to_le_bytes();
            out[i * 4..i * 4 + 4].copy_from_slice(&bytes);
        }
        self.state[12] = self.state[12].wrapping_add(1);
        out
    }
}

fn quarter_round(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    x[a] = x[a].wrapping_add(x[b]); x[d] ^= x[a]; x[d] = x[d].rotate_left(16);
    x[c] = x[c].wrapping_add(x[d]); x[b] ^= x[c]; x[b] = x[b].rotate_left(12);
    x[a] = x[a].wrapping_add(x[b]); x[d] ^= x[a]; x[d] = x[d].rotate_left(8);
    x[c] = x[c].wrapping_add(x[d]); x[b] ^= x[c]; x[b] = x[b].rotate_left(7);
}

// --- BLAKE3 Cryptographic Hash Algorithm ---

const BLAKE3_IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const CHUNK_START: u32 = 1;
const CHUNK_END: u32 = 2;
const PARENT: u32 = 4;
const ROOT: u32 = 8;

#[derive(Clone, Copy)]
pub struct Blake3 {
    pub stack: [([u32; 8], u32); 54],
    pub stack_len: usize,
    pub chunk_buffer: [u8; 1024],
    pub chunk_buffer_len: usize,
    pub chunk_counter: u64,
}

impl Default for Blake3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Blake3 {
    pub fn new() -> Self {
        Self {
            stack: [([0; 8], 0); 54],
            stack_len: 0,
            chunk_buffer: [0; 1024],
            chunk_buffer_len: 0,
            chunk_counter: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        while !data.is_empty() {
            if self.chunk_buffer_len == 1024 {
                let cv = hash_chunk(&self.chunk_buffer, self.chunk_counter, 0);
                self.push_entry(cv, 0);
                self.chunk_counter += 1;
                self.chunk_buffer_len = 0;
            }
            let want = 1024 - self.chunk_buffer_len;
            let take = std::cmp::min(want, data.len());
            self.chunk_buffer[self.chunk_buffer_len..self.chunk_buffer_len + take].copy_from_slice(&data[..take]);
            self.chunk_buffer_len += take;
            data = &data[take..];
        }
    }

    fn push_entry(&mut self, mut cv: [u32; 8], mut height: u32) {
        while self.stack_len > 0 && self.stack[self.stack_len - 1].1 == height {
            let (left_cv, _) = self.stack[self.stack_len - 1];
            self.stack_len -= 1;
            cv = parent_cv(&left_cv, &cv, 0);
            height += 1;
        }
        self.stack[self.stack_len] = (cv, height);
        self.stack_len += 1;
    }

    pub fn finalize(self) -> [u8; 32] {
        if self.stack_len == 0 {
            let cv = hash_chunk(&self.chunk_buffer[..self.chunk_buffer_len], self.chunk_counter, ROOT);
            let mut out = [0u8; 32];
            for i in 0..8 {
                out[i * 4..i * 4 + 4].copy_from_slice(&cv[i].to_le_bytes());
            }
            return out;
        }
        let mut current_cv = hash_chunk(&self.chunk_buffer[..self.chunk_buffer_len], self.chunk_counter, 0);
        for i in (0..self.stack_len).rev() {
            let flags = if i == 0 { ROOT } else { 0 };
            current_cv = parent_cv(&self.stack[i].0, &current_cv, flags);
        }
        let mut out = [0u8; 32];
        for i in 0..8 {
            out[i * 4..i * 4 + 4].copy_from_slice(&current_cv[i].to_le_bytes());
        }
        out
    }
}

fn blake3_g(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, x: u32, y: u32) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(x);
    state[d] = (state[d] ^ state[a]).rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(12);
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(y);
    state[d] = (state[d] ^ state[a]).rotate_right(8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(7);
}

fn blake3_permute(m: &mut [u32; 16]) {
    let mut permuted = [0u32; 16];
    permuted[0] = m[2];
    permuted[1] = m[6];
    permuted[2] = m[3];
    permuted[3] = m[10];
    permuted[4] = m[7];
    permuted[5] = m[0];
    permuted[6] = m[4];
    permuted[7] = m[13];
    permuted[8] = m[1];
    permuted[9] = m[11];
    permuted[10] = m[12];
    permuted[11] = m[5];
    permuted[12] = m[9];
    permuted[13] = m[14];
    permuted[14] = m[15];
    permuted[15] = m[8];
    *m = permuted;
}

fn blake3_compress(
    chaining_value: &[u32; 8],
    block_words: &[u32; 16],
    counter: u64,
    block_len: u32,
    flags: u32,
) -> [u32; 16] {
    let mut state = [
        chaining_value[0], chaining_value[1], chaining_value[2], chaining_value[3],
        chaining_value[4], chaining_value[5], chaining_value[6], chaining_value[7],
        BLAKE3_IV[0], BLAKE3_IV[1], BLAKE3_IV[2], BLAKE3_IV[3],
        counter as u32, (counter >> 32) as u32, block_len, flags
    ];

    let mut m = *block_words;

    for _ in 0..7 {
        blake3_g(&mut state, 0, 4, 8, 12, m[0], m[1]);
        blake3_g(&mut state, 1, 5, 9, 13, m[2], m[3]);
        blake3_g(&mut state, 2, 6, 10, 14, m[4], m[5]);
        blake3_g(&mut state, 3, 7, 11, 15, m[6], m[7]);
        blake3_g(&mut state, 0, 5, 10, 15, m[8], m[9]);
        blake3_g(&mut state, 1, 6, 11, 12, m[10], m[11]);
        blake3_g(&mut state, 2, 7, 8, 13, m[12], m[13]);
        blake3_g(&mut state, 3, 4, 9, 14, m[14], m[15]);

        blake3_permute(&mut m);
    }

    for i in 0..8 {
        state[i] ^= chaining_value[i];
        state[i + 8] ^= BLAKE3_IV[i];
    }

    state
}

fn parent_cv(
    left_child: &[u32; 8],
    right_child: &[u32; 8],
    flags: u32,
) -> [u32; 8] {
    let mut block_words = [0u32; 16];
    block_words[0..8].copy_from_slice(left_child);
    block_words[8..16].copy_from_slice(right_child);
    let out = blake3_compress(&BLAKE3_IV, &block_words, 0, 64, flags | PARENT);
    let mut cv = [0u32; 8];
    cv.copy_from_slice(&out[0..8]);
    cv
}

fn hash_chunk(
    chunk_bytes: &[u8],
    chunk_counter: u64,
    flags: u32,
) -> [u32; 8] {
    let num_blocks = chunk_bytes.len().div_ceil(64);
    let num_blocks = if num_blocks == 0 { 1 } else { num_blocks };
    let mut cv = BLAKE3_IV;
    for i in 0..num_blocks {
        let start = i * 64;
        let end = std::cmp::min(chunk_bytes.len(), start + 64);
        let block_len = (end - start) as u32;
        let mut block = [0u8; 64];
        block[..block_len as usize].copy_from_slice(&chunk_bytes[start..end]);
        let mut block_words = [0u32; 16];
        for j in 0..16 {
            block_words[j] = u32::from_le_bytes([
                block[j * 4],
                block[j * 4 + 1],
                block[j * 4 + 2],
                block[j * 4 + 3],
            ]);
        }
        let mut block_flags = flags;
        if i == 0 {
            block_flags |= CHUNK_START;
        }
        if i == num_blocks - 1 {
            block_flags |= CHUNK_END;
        }
        let out = blake3_compress(&cv, &block_words, chunk_counter, block_len, block_flags);
        cv.copy_from_slice(&out[0..8]);
    }
    cv
}

// --- SHA-512 Cryptographic Hash Algorithm ---

pub struct Sha512 {
    pub state: [u64; 8],
    pub buffer: [u8; 128],
    pub len: u64,
}

impl Default for Sha512 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha512 {
    pub fn new() -> Self {
        Self {
            state: [
                0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
                0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
            ],
            buffer: [0; 128],
            len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        let buffer_len = (self.len % 128) as usize;
        self.len += data.len() as u64;

        if buffer_len > 0 {
            let fill = 128 - buffer_len;
            if data.len() >= fill {
                self.buffer[buffer_len..128].copy_from_slice(&data[..fill]);
                let chunk = self.buffer;
                self.transform(&chunk);
                data = &data[fill..];
            } else {
                self.buffer[buffer_len..buffer_len + data.len()].copy_from_slice(data);
                return;
            }
        }

        while data.len() >= 128 {
            let chunk: &[u8; 128] = data[..128].try_into().unwrap();
            self.transform(chunk);
            data = &data[128..];
        }

        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
        }
    }

    pub fn finalize(mut self) -> [u8; 64] {
        let buffer_len = (self.len % 128) as usize;
        self.buffer[buffer_len] = 0x80;
        
        if buffer_len + 1 > 112 {
            self.buffer[buffer_len + 1..128].fill(0);
            let chunk = self.buffer;
            self.transform(&chunk);
            self.buffer[..112].fill(0);
        } else {
            self.buffer[buffer_len + 1..112].fill(0);
        }

        let bits = (self.len * 8).to_be_bytes();
        self.buffer[112..120].copy_from_slice(&[0; 8]);
        self.buffer[120..128].copy_from_slice(&bits);
        let chunk = self.buffer;
        self.transform(&chunk);

        let mut out = [0u8; 64];
        for i in 0..8 {
            out[i * 8..i * 8 + 8].copy_from_slice(&self.state[i].to_be_bytes());
        }
        out
    }

    fn transform(&mut self, chunk: &[u8; 128]) {
        const K: [u64; 80] = [
            0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
            0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
            0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
            0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
            0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
            0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
            0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
            0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
            0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
            0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
            0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
            0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
            0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
            0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
            0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
            0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
            0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
            0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
            0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
            0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
        ];

        let mut w = [0u64; 80];
        for i in 0..16 {
            w[i] = u64::from_be_bytes(chunk[i * 8..i * 8 + 8].try_into().unwrap());
        }
        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

// --- Field Arithmetic modulo p = 2^255 - 19 ---

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldElement(pub [u64; 4]);

#[allow(clippy::should_implement_trait, clippy::needless_range_loop)]
impl FieldElement {
    pub const P: Self = FieldElement([
        0xffffffffffffffed, 0xffffffffffffffff,
        0xffffffffffffffff, 0x7fffffffffffffff
    ]);

    pub fn zero() -> Self { FieldElement([0, 0, 0, 0]) }
    pub fn one() -> Self { FieldElement([1, 0, 0, 0]) }

    pub fn add(self, other: Self) -> Self {
        let mut res = [0u64; 4];
        let mut carry = 0u128;
        for i in 0..4 {
            let sum = (self.0[i] as u128) + (other.0[i] as u128) + carry;
            res[i] = sum as u64;
            carry = sum >> 64;
        }
        if carry > 0 || res_gte_p(res) {
            let mut borrow = 0u128;
            for i in 0..4 {
                let diff = (res[i] as u128).wrapping_sub(Self::P.0[i] as u128).wrapping_sub(borrow);
                res[i] = diff as u64;
                borrow = (diff >> 64) & 1;
            }
        }
        FieldElement(res)
    }

    pub fn sub(self, other: Self) -> Self {
        let mut res = [0u64; 4];
        let mut borrow = 0u128;
        for i in 0..4 {
            let diff = (self.0[i] as u128).wrapping_sub(other.0[i] as u128).wrapping_sub(borrow);
            res[i] = diff as u64;
            borrow = (diff >> 64) & 1;
        }
        if borrow > 0 {
            let mut carry = 0u128;
            for i in 0..4 {
                let sum = (res[i] as u128) + (Self::P.0[i] as u128) + carry;
                res[i] = sum as u64;
                carry = sum >> 64;
            }
        }
        FieldElement(res)
    }

    pub fn mul(self, other: Self) -> Self {
        let mut product = [0u64; 8];
        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                let val = (product[i + j] as u128) + (self.0[i] as u128) * (other.0[j] as u128) + carry;
                product[i + j] = val as u64;
                carry = val >> 64;
            }
            product[i + 4] = carry as u64;
        }
        reduce_512(product)
    }

    pub fn pow_p_minus_2(self) -> Self {
        let exp: [u64; 4] = [0xffffffffffffffeb, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff];
        let mut res = FieldElement::one();
        let mut base = self;
        for &word in &exp {
            let mut w = word;
            for _ in 0..64 {
                if w & 1 == 1 {
                    res = res.mul(base);
                }
                base = base.mul(base);
                w >>= 1;
            }
        }
        res
    }
}

fn res_gte_p(res: [u64; 4]) -> bool {
    for i in (0..4).rev() {
        if res[i] > FieldElement::P.0[i] { return true; }
        if res[i] < FieldElement::P.0[i] { return false; }
    }
    true
}

#[allow(clippy::needless_range_loop)]
fn reduce_512(product: [u64; 8]) -> FieldElement {
    let mut high = [0u64; 4];
    let mut carry = (product[3] >> 63) as u128;
    for i in 0..4 {
        let val = product[i + 4] as u128;
        high[i] = ((val << 1) | carry) as u64;
        carry = val >> 63;
    }
    
    let low = [
        product[0],
        product[1],
        product[2],
        product[3] & 0x7fffffffffffffff,
    ];
    
    let mut high_x_19 = [0u64; 5];
    let mut carry_mul = 0u128;
    for i in 0..4 {
        let val = (high[i] as u128) * 19 + carry_mul;
        high_x_19[i] = val as u64;
        carry_mul = val >> 64;
    }
    high_x_19[4] = (carry_mul + carry * 19) as u64;
    
    let mut sum = [0u64; 5];
    let mut carry_add = 0u128;
    for i in 0..4 {
        let val = (low[i] as u128) + (high_x_19[i] as u128) + carry_add;
        sum[i] = val as u64;
        carry_add = val >> 64;
    }
    sum[4] = (high_x_19[4] as u128 + carry_add) as u64;
    
    let sum_high_val = ((sum[3] >> 63) as u128) | ((sum[4] as u128) << 1);
    let sum_high_x_19 = sum_high_val * 19;
    
    let mut final_sum = [0u64; 4];
    final_sum[0] = sum[0];
    final_sum[1] = sum[1];
    final_sum[2] = sum[2];
    final_sum[3] = sum[3] & 0x7fffffffffffffff;
    
    let mut carry = sum_high_x_19;
    for i in 0..4 {
        let val = (final_sum[i] as u128) + carry;
        final_sum[i] = val as u64;
        carry = val >> 64;
    }
    
    if carry > 0 {
        let mut carry_2 = carry * 38;
        for i in 0..4 {
            let val = (final_sum[i] as u128) + carry_2;
            final_sum[i] = val as u64;
            carry_2 = val >> 64;
        }
    }
    
    while res_gte_p(final_sum) {
        let mut borrow = 0u128;
        for i in 0..4 {
            let diff = (final_sum[i] as u128).wrapping_sub(FieldElement::P.0[i] as u128).wrapping_sub(borrow);
            final_sum[i] = diff as u64;
            borrow = (diff >> 64) & 1;
        }
    }
    
    FieldElement(final_sum)
}

// --- Twisted Edwards Curve25519 Operations ---

#[derive(Clone, Copy, Debug)]
pub struct CurvePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
    pub t: FieldElement,
}

#[allow(clippy::should_implement_trait, clippy::needless_range_loop)]
impl CurvePoint {
    pub const D: FieldElement = FieldElement([
        0x75eb4dca135978a3, 0x00700a4d4141d8ab,
        0x8cc740797779e898, 0x52036cee2b6ffe73,
    ]);

    pub const TWO_D: FieldElement = FieldElement([
        0xebd69b9426b2f159, 0x00e0149a8283b156,
        0x198e80f2eef3d130, 0x2406d9dc56dffce7,
    ]);

    pub const SQRT_M1: FieldElement = FieldElement([
        0xc4ee1b274a0ea0b0, 0x2f431806ad2fe478,
        0x2b4d00993dfbd7a7, 0x2b8324804fc1df0b
    ]);

    pub fn generator() -> Self {
        CurvePoint {
            x: FieldElement([
                0xc9562d608f25d51a, 0x692cc7609525a7b2,
                0xc0a4e231fdd6dc5c, 0x216936d3cd6e53fe,
            ]),
            y: FieldElement([
                0x6666666666666658, 0x6666666666666666,
                0x6666666666666666, 0x6666666666666666,
            ]),
            z: FieldElement::one(),
            t: FieldElement([
                0x6dde8ab3a5b7dda3, 0x20f09f80775152f5,
                0x66ea4e8e64abe37d, 0x67875f0fd78b7665,
            ]),
        }
    }

    pub fn identity() -> Self {
        CurvePoint {
            x: FieldElement::zero(),
            y: FieldElement::one(),
            z: FieldElement::one(),
            t: FieldElement::zero(),
        }
    }

    pub fn double(self) -> Self {
        let a = self.x.mul(self.x);
        let b = self.y.mul(self.y);
        let c = FieldElement([2, 0, 0, 0]).mul(self.z.mul(self.z));
        let d = FieldElement::zero().sub(a);
        let x_plus_y = self.x.add(self.y);
        let j = x_plus_y.mul(x_plus_y).sub(a).sub(b);
        let e = j;
        let g = d.add(b);
        let f = g.sub(c);
        let h = d.sub(b);
        CurvePoint {
            x: e.mul(f),
            y: g.mul(h),
            z: f.mul(g),
            t: e.mul(h),
        }
    }

    pub fn add(self, other: Self) -> Self {
        let a = (self.y.sub(self.x)).mul(other.y.sub(other.x));
        let b = (self.y.add(self.x)).mul(other.y.add(other.x));
        let c = Self::TWO_D.mul(self.t).mul(other.t);
        let d = (self.z.mul(FieldElement([2, 0, 0, 0]))).mul(other.z);
        let e = b.sub(a);
        let f = d.sub(c);
        let g = d.add(c);
        let h = b.add(a);
        CurvePoint {
            x: e.mul(f),
            y: g.mul(h),
            z: f.mul(g),
            t: e.mul(h),
        }
    }

    pub fn mul(self, scalar: &[u8; 32]) -> Self {
        let mut r = Self::identity();
        let mut p = self;
        for i in 0..256 {
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            if (scalar[byte_idx] >> bit_idx) & 1 == 1 {
                r = r.add(p);
            }
            p = p.double();
        }
        r
    }

    pub fn decompress(bytes: &[u8; 32]) -> Option<Self> {
        let mut y_limbs = [0u64; 4];
        for i in 0..4 {
            y_limbs[i] = u64::from_le_bytes(bytes[i*8..i*8+8].try_into().unwrap());
        }
        let sign = (y_limbs[3] >> 63) & 1;
        y_limbs[3] &= 0x7fffffffffffffff;
        
        if res_gte_p(y_limbs) {
            return None;
        }
        
        let y = FieldElement(y_limbs);
        let y2 = y.mul(y);
        let u = y2.sub(FieldElement::one());
        let v = Self::D.mul(y2).add(FieldElement::one());
        
        let inv_v = v.pow_p_minus_2();
        let w = u.mul(inv_v);
        
        let mut x = w_pow_2_252_minus_2(w);
        let mut x2 = x.mul(x);
        
        if x2 != w {
            x = x.mul(Self::SQRT_M1);
            x2 = x.mul(x);
            if x2 != w {
                return None;
            }
        }
        
        let x_bytes = x.0[0].to_le_bytes();
        let x_sign = (x_bytes[0] & 1) as u64;
        if x_sign != sign {
            x = FieldElement::zero().sub(x);
        }
        
        let final_x_bytes = x.0[0].to_le_bytes();
        let final_x_sign = (final_x_bytes[0] & 1) as u64;
        if final_x_sign != sign {
            return None;
        }
        
        let t = x.mul(y);
        Some(CurvePoint { x, y, z: FieldElement::one(), t })
    }
}

pub fn w_pow_2_252_minus_2(w: FieldElement) -> FieldElement {
    let mut res = FieldElement::one();
    let mut base = w;
    for i in 0..252 {
        if i > 0 {
            res = res.mul(base);
        }
        base = base.mul(base);
    }
    res
}

// --- Ed25519 Signature Verification according to RFC 8032 ---

pub fn verify_ed25519_signature(
    public_key_bytes: &[u8; 32],
    signature_bytes: &[u8; 64],
    message: &[u8]
) -> bool {
    let r_bytes: &[u8; 32] = signature_bytes[0..32].try_into().unwrap();
    let s_bytes: &[u8; 32] = signature_bytes[32..64].try_into().unwrap();
    
    let r_point = match CurvePoint::decompress(r_bytes) {
        Some(p) => p,
        None => return false,
    };
    
    let pk_point = match CurvePoint::decompress(public_key_bytes) {
        Some(p) => p,
        None => return false,
    };
    
    // Check range of S: [0, L)
    const L_LIMIT: [u64; 4] = [
        0x5812631a5cf5d3ed, 0x14def9dea2f79cd6,
        0x0000000000000000, 0x1000000000000000
    ];
    let mut s_limbs = [0u64; 4];
    for i in 0..4 {
        s_limbs[i] = u64::from_le_bytes(s_bytes[i*8..i*8+8].try_into().unwrap());
    }
    for i in (0..4).rev() {
        if s_limbs[i] > L_LIMIT[i] { return false; }
        if s_limbs[i] < L_LIMIT[i] { break; }
        if i == 0 { return false; } // S == L is illegal
    }
    
    // k = SHA-512(R || PK || Message)
    let mut hasher = Sha512::new();
    hasher.update(r_bytes);
    hasher.update(public_key_bytes);
    hasher.update(message);
    let hash_result = hasher.finalize();
    
    let k_scalar = reduce_sha512_mod_l(&hash_result);
    
    // Check cofactor cleared equation: [8][S]B = [8]R + [8][k]PK
    let sb = CurvePoint::generator().mul(s_bytes);
    let k_pk = pk_point.mul(&k_scalar);
    let r_plus_k_pk = r_point.add(k_pk);
    
    let sb_8 = sb.double().double().double();
    let r_plus_k_pk_8 = r_plus_k_pk.double().double().double();
    
    // Compare Projective coordinates: X1 * Z2 == X2 * Z1 and Y1 * Z2 == Y2 * Z1
    let x1_z2 = sb_8.x.mul(r_plus_k_pk_8.z);
    let x2_z1 = r_plus_k_pk_8.x.mul(sb_8.z);
    let y1_z2 = sb_8.y.mul(r_plus_k_pk_8.z);
    let y2_z1 = r_plus_k_pk_8.y.mul(sb_8.z);
    
    x1_z2 == x2_z1 && y1_z2 == y2_z1
}

fn reduce_sha512_mod_l(hash: &[u8; 64]) -> [u8; 32] {
    let mut val = [0u64; 8];
    for i in 0..8 {
        val[i] = u64::from_le_bytes(hash[i * 8..i * 8 + 8].try_into().unwrap());
    }
    
    const L: [u64; 8] = [
        0x5812631a5cf5d3ed,
        0x14def9dea2f79cd6,
        0x0000000000000000,
        0x1000000000000000,
        0, 0, 0, 0
    ];
    
    for shift in (0..=259).rev() {
        let bit_shift = shift % 64;
        let word_shift = shift / 64;
        
        let mut temp = [0u64; 8];
        let mut carry = 0u64;
        for i in 0..8 {
            let val_i = L[i];
            temp[i] = (val_i << bit_shift) | carry;
            carry = if bit_shift == 0 { 0 } else { val_i >> (64 - bit_shift) };
        }
        
        let mut l_shifted = [0u64; 8];
        for i in 0..8 {
            if i + word_shift < 8 {
                l_shifted[i + word_shift] = temp[i];
            }
        }
        
        if val_gte(val, l_shifted) {
            let mut borrow = 0u128;
            for i in 0..8 {
                let diff = (val[i] as u128).wrapping_sub(l_shifted[i] as u128).wrapping_sub(borrow);
                val[i] = diff as u64;
                borrow = (diff >> 64) & 1;
            }
        }
    }
    
    let mut out = [0u8; 32];
    for i in 0..4 {
        out[i * 8..i * 8 + 8].copy_from_slice(&val[i].to_le_bytes());
    }
    out
}

fn val_gte(a: [u64; 8], b: [u64; 8]) -> bool {
    for i in (0..8).rev() {
        if a[i] > b[i] { return true; }
        if a[i] < b[i] { return false; }
    }
    true
}

// --- JSON Parsing & JCS Canonicalization RFC 8785 Helpers ---

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub fn to_jcs(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
            JsonValue::Number(n) => n.clone(),
            JsonValue::String(s) => {
                let mut escaped = String::new();
                escaped.push('"');
                for c in s.chars() {
                    match c {
                        '"' => escaped.push_str("\\\""),
                        '\\' => escaped.push_str("\\\\"),
                        '\x08' => escaped.push_str("\\b"),
                        '\x0c' => escaped.push_str("\\f"),
                        '\n' => escaped.push_str("\\n"),
                        '\r' => escaped.push_str("\\r"),
                        '\t' => escaped.push_str("\\t"),
                        _ if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
                        _ => escaped.push(c),
                    }
                }
                escaped.push('"');
                escaped
            }
            JsonValue::Array(arr) => {
                let mut res = String::new();
                res.push('[');
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        res.push(',');
                    }
                    res.push_str(&val.to_jcs());
                }
                res.push(']');
                res
            }
            JsonValue::Object(obj) => {
                let mut res = String::new();
                res.push('{');
                for (i, (key, val)) in obj.iter().enumerate() {
                    if i > 0 {
                        res.push(',');
                    }
                    let key_val = JsonValue::String(key.clone());
                    res.push_str(&key_val.to_jcs());
                    res.push(':');
                    res.push_str(&val.to_jcs());
                }
                res.push('}');
                res
            }
        }
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut idx = 0;
    skip_whitespace(&chars, &mut idx);
    let val = parse_value(&chars, &mut idx)?;
    skip_whitespace(&chars, &mut idx);
    if idx < chars.len() {
        return Err("Unexpected trailing characters".to_string());
    }
    Ok(val)
}

fn skip_whitespace(chars: &[char], idx: &mut usize) {
    while *idx < chars.len() && chars[*idx].is_whitespace() {
        *idx += 1;
    }
}

fn parse_value(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    if *idx >= chars.len() {
        return Err("Unexpected EOF".to_string());
    }
    match chars[*idx] {
        '{' => parse_object(chars, idx),
        '[' => parse_array(chars, idx),
        '"' => parse_string(chars, idx),
        't' | 'f' => parse_bool(chars, idx),
        'n' => parse_null(chars, idx),
        '-' | '0'..='9' => parse_number(chars, idx),
        c => Err(format!("Unexpected character: {}", c)),
    }
}

fn parse_object(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    *idx += 1; // skip '{'
    let mut map = std::collections::BTreeMap::new();
    loop {
        skip_whitespace(chars, idx);
        if *idx >= chars.len() {
            return Err("Unterminated object".to_string());
        }
        if chars[*idx] == '}' {
            *idx += 1;
            break;
        }
        if chars[*idx] != '"' {
            return Err("Expected string key".to_string());
        }
        let key_val = parse_string(chars, idx)?;
        let key = match key_val {
            JsonValue::String(k) => k,
            _ => unreachable!(),
        };
        skip_whitespace(chars, idx);
        if *idx >= chars.len() || chars[*idx] != ':' {
            return Err("Expected ':' after key".to_string());
        }
        *idx += 1; // skip ':'
        skip_whitespace(chars, idx);
        let val = parse_value(chars, idx)?;
        map.insert(key, val);
        skip_whitespace(chars, idx);
        if *idx >= chars.len() {
            return Err("Unterminated object".to_string());
        }
        if chars[*idx] == ',' {
            *idx += 1;
        } else if chars[*idx] == '}' {
            *idx += 1;
            break;
        } else {
            return Err("Expected ',' or '}'".to_string());
        }
    }
    Ok(JsonValue::Object(map))
}

fn parse_array(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    *idx += 1; // skip '['
    let mut arr = Vec::new();
    loop {
        skip_whitespace(chars, idx);
        if *idx >= chars.len() {
            return Err("Unterminated array".to_string());
        }
        if chars[*idx] == ']' {
            *idx += 1;
            break;
        }
        let val = parse_value(chars, idx)?;
        arr.push(val);
        skip_whitespace(chars, idx);
        if *idx >= chars.len() {
            return Err("Unterminated array".to_string());
        }
        if chars[*idx] == ',' {
            *idx += 1;
        } else if chars[*idx] == ']' {
            *idx += 1;
            break;
        } else {
            return Err("Expected ',' or ']'".to_string());
        }
    }
    Ok(JsonValue::Array(arr))
}

fn parse_string(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    *idx += 1; // skip '"'
    let mut s = String::new();
    while *idx < chars.len() {
        match chars[*idx] {
            '"' => {
                *idx += 1;
                return Ok(JsonValue::String(s));
            }
            '\\' => {
                *idx += 1;
                if *idx >= chars.len() {
                    return Err("Unterminated string escape".to_string());
                }
                match chars[*idx] {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'b' => s.push('\x08'),
                    'f' => s.push('\x0c'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'u' => {
                        *idx += 1;
                        if *idx + 4 > chars.len() {
                            return Err("Invalid unicode escape".to_string());
                        }
                        let hex: String = chars[*idx..*idx + 4].iter().collect();
                        *idx += 3;
                        let code = u32::from_str_radix(&hex, 16)
                            .map_err(|e| e.to_string())?;
                        let c = std::char::from_u32(code)
                            .ok_or_else(|| "Invalid unicode char".to_string())?;
                        s.push(c);
                    }
                    c => return Err(format!("Unknown escape character: {}", c)),
                }
            }
            c => s.push(c),
        }
        *idx += 1;
    }
    Err("Unterminated string".to_string())
}

fn parse_bool(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    if *idx + 4 <= chars.len() && chars[*idx..*idx+4] == ['t', 'r', 'u', 'e'] {
        *idx += 4;
        Ok(JsonValue::Bool(true))
    } else if *idx + 5 <= chars.len() && chars[*idx..*idx+5] == ['f', 'a', 'l', 's', 'e'] {
        *idx += 5;
        Ok(JsonValue::Bool(false))
    } else {
        Err("Expected boolean".to_string())
    }
}

fn parse_null(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    if *idx + 4 <= chars.len() && chars[*idx..*idx+4] == ['n', 'u', 'l', 'l'] {
        *idx += 4;
        Ok(JsonValue::Null)
    } else {
        Err("Expected null".to_string())
    }
}

fn parse_number(chars: &[char], idx: &mut usize) -> Result<JsonValue, String> {
    let mut num_str = String::new();
    if chars[*idx] == '-' {
        num_str.push('-');
        *idx += 1;
    }
    while *idx < chars.len() {
        let c = chars[*idx];
        if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
            num_str.push(c);
            *idx += 1;
        } else {
            break;
        }
    }
    if num_str.is_empty() {
        return Err("Empty number".to_string());
    }
    Ok(JsonValue::Number(num_str))
}

pub fn verify_jcs_receipt_signature(
    public_key_bytes: &[u8; 32],
    signature_bytes: &[u8; 64],
    raw_json: &str
) -> bool {
    let parsed = match parse_json(raw_json) {
        Ok(val) => val,
        Err(_) => return false,
    };
    
    let unsigned_val = match parsed {
        JsonValue::Object(mut map) => {
            map.remove("validator_signature");
            JsonValue::Object(map)
        }
        _ => return false,
    };
    
    let jcs_str = unsigned_val.to_jcs();
    verify_ed25519_signature(public_key_bytes, signature_bytes, jcs_str.as_bytes())
}
