use std::sync::Mutex;
use std::alloc::Layout;

// Hard ceiling of 100MB as per audits/audit-execution-boundaries.md
pub const DEFAULT_MEMORY_CEILING: usize = 100 * 1024 * 1024; // 100 MB
pub const ABSOLUTE_HEAP_LIMIT: usize = 1024 * 1024 * 1024; // 1024 MB from lifecycle map

#[derive(Debug, Clone, Copy)]
pub enum AllocError {
    OutOfMemory,
    NotInitialized,
    InvalidAlignment,
    CeilingExceeded,
}

pub struct DoubleBufferedArena {
    buffer: Vec<u8>,
    permanent_cursor: usize,
    transient_cursor: usize,
    transient_start: usize,
    ceiling: usize,
    aligned_offset: usize,
}

impl DoubleBufferedArena {
    pub fn new(ceiling: usize) -> Self {
        let mut buffer = Vec::with_capacity(ceiling + 8);
        buffer.resize(ceiling + 8, 0);
        let base_addr = buffer.as_ptr() as usize;
        let aligned_addr = (base_addr + 7) & !7;
        let aligned_offset = aligned_addr - base_addr;
        let transient_start = ceiling / 2; // Split 50/50 for permanent and transient allocations
        Self {
            buffer,
            permanent_cursor: 8,
            transient_cursor: transient_start,
            transient_start,
            ceiling,
            aligned_offset,
        }
    }

    pub fn alloc_permanent(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
        let align = layout.align();
        let size = layout.size();

        if !align.is_power_of_two() {
            return Err(AllocError::InvalidAlignment);
        }

        let aligned_cursor = (self.permanent_cursor + align - 1) & !(align - 1);
        if aligned_cursor + size > self.transient_start {
            return Err(AllocError::OutOfMemory);
        }

        self.permanent_cursor = aligned_cursor + size;
        let ptr = unsafe { self.buffer.as_mut_ptr().add(self.aligned_offset + aligned_cursor) };
        Ok(ptr)
    }

    pub fn alloc_transient(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
        let align = layout.align();
        let size = layout.size();

        if !align.is_power_of_two() {
            return Err(AllocError::InvalidAlignment);
        }

        let aligned_cursor = (self.transient_cursor + align - 1) & !(align - 1);
        if aligned_cursor + size > self.ceiling {
            return Err(AllocError::OutOfMemory);
        }

        self.transient_cursor = aligned_cursor + size;
        let ptr = unsafe { self.buffer.as_mut_ptr().add(self.aligned_offset + aligned_cursor) };
        Ok(ptr)
    }

    pub fn reset_transient(&mut self) {
        // Zero-fill the transient space upon reset to avoid residual data leaks
        let start = self.transient_start;
        let end = self.transient_cursor;
        if end > start {
            let abs_start = self.aligned_offset + start;
            let abs_end = self.aligned_offset + end;
            self.buffer[abs_start..abs_end].fill(0);
        }
        self.transient_cursor = self.transient_start;
    }

    pub fn reset_all(&mut self) {
        self.buffer.fill(0);
        self.permanent_cursor = 8;
        self.transient_cursor = self.transient_start;
    }

    pub fn shred(&mut self, prng_bytes: &mut dyn FnMut() -> [u8; 64]) {
        // Pass 1-3: Cryptographically secure random overwrites using volatile writes
        for _ in 0..3 {
            let mut offset = 0;
            while offset < self.buffer.len() {
                let bytes = prng_bytes();
                let remaining = self.buffer.len() - offset;
                let chunk_size = std::cmp::min(64, remaining);
                let chunk_ptr = unsafe { self.buffer.as_mut_ptr().add(offset) };
                for i in 0..chunk_size {
                    unsafe {
                        std::ptr::write_volatile(chunk_ptr.add(i), bytes[i]);
                    }
                }
                offset += chunk_size;
            }
        }
        
        // Pass 4: Final zeroization to wipe cryptographic residuals (volatile_zero_slice)
        crate::zeroize::volatile_zero_slice(&mut self.buffer);

        // Ensure cursors are also zeroed using volatile writes
        unsafe {
            std::ptr::write_volatile(&mut self.permanent_cursor, 0);
            std::ptr::write_volatile(&mut self.transient_cursor, 0);
        }
    }

    pub fn raw_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn contains_ptr(&self, ptr: *const u8, len: usize) -> bool {
        let start = (self.buffer.as_ptr() as usize) + self.aligned_offset;
        let end = start + self.ceiling;
        let target_start = ptr as usize;
        let target_end = target_start + len;

        target_start >= start && target_end <= end && target_end >= target_start
    }

    pub fn get_relative_offset(&self, ptr: *const u8) -> Option<u32> {
        let start = (self.buffer.as_ptr() as usize) + self.aligned_offset;
        let target = ptr as usize;
        if target >= start && target < start + self.ceiling {
            Some((target - start) as u32)
        } else {
            None
        }
    }

    pub fn get_absolute_ptr(&self, offset: u32) -> Option<*mut u8> {
        if (offset as usize) < self.ceiling {
            Some(unsafe { self.buffer.as_ptr().add(self.aligned_offset + offset as usize) as *mut u8 })
        } else {
            None
        }
    }
}

// Global thread-safe allocator singleton
static GLOBAL_ARENA: Mutex<Option<DoubleBufferedArena>> = Mutex::new(None);

pub fn init_global_arena(ceiling: usize) -> Result<(), AllocError> {
    if ceiling > ABSOLUTE_HEAP_LIMIT {
        return Err(AllocError::CeilingExceeded);
    }
    let mut guard = GLOBAL_ARENA.lock().unwrap();
    *guard = Some(DoubleBufferedArena::new(ceiling));
    Ok(())
}

pub fn alloc_permanent(layout: Layout) -> Result<*mut u8, AllocError> {
    let mut guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref mut arena) = *guard {
        arena.alloc_permanent(layout)
    } else {
        Err(AllocError::NotInitialized)
    }
}

pub fn alloc_transient(layout: Layout) -> Result<*mut u8, AllocError> {
    let mut guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref mut arena) = *guard {
        arena.alloc_transient(layout)
    } else {
        Err(AllocError::NotInitialized)
    }
}

pub fn reset_transient() {
    let mut guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref mut arena) = *guard {
        arena.reset_transient();
    }
}

pub fn reset_all() {
    let mut guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref mut arena) = *guard {
        arena.reset_all();
    }
}

pub fn shred_global_arena(prng_bytes: &mut dyn FnMut() -> [u8; 64]) {
    let mut guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref mut arena) = *guard {
        arena.shred(prng_bytes);
    }
}

pub fn get_global_arena_raw_buffer() -> Vec<u8> {
    let guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref arena) = *guard {
        arena.raw_buffer().to_vec()
    } else {
        vec![]
    }
}

pub fn fill_global_arena_raw_buffer(val: u8) {
    let mut guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref mut arena) = *guard {
        arena.buffer.fill(val);
    }
}


pub fn validate_pointer(ptr: *const u8, len: usize) -> bool {
    let guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref arena) = *guard {
        arena.contains_ptr(ptr, len)
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArenaBoundaries {
    pub base_addr: usize,
    pub transient_start: usize,
    pub ceiling: usize,
}

pub fn get_arena_boundaries() -> Option<ArenaBoundaries> {
    let guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref arena) = *guard {
        let base_addr = (arena.buffer.as_ptr() as usize) + arena.aligned_offset;
        Some(ArenaBoundaries {
            base_addr,
            transient_start: arena.transient_start,
            ceiling: arena.ceiling,
        })
    } else {
        None
    }
}

pub fn get_relative_offset(ptr: *const u8) -> Option<u32> {
    let guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref arena) = *guard {
        arena.get_relative_offset(ptr)
    } else {
        None
    }
}

pub fn get_absolute_ptr(offset: u32) -> Option<*mut u8> {
    let guard = GLOBAL_ARENA.lock().unwrap();
    if let Some(ref arena) = *guard {
        arena.get_absolute_ptr(offset)
    } else {
        None
    }
}
