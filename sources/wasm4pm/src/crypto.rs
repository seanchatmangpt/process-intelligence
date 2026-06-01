// Complete, self-contained cryptographic algorithms: SHA-256 and ChaCha20

pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    len: u64,
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
    state: [u32; 16],
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
    stack: [([u32; 8], u32); 54],
    stack_len: usize,
    chunk_buffer: [u8; 1024],
    chunk_buffer_len: usize,
    chunk_counter: u64,
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
    let num_blocks = (chunk_bytes.len() + 63) / 64;
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
