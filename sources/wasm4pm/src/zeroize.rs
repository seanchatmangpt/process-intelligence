use std::sync::atomic::{compiler_fence, Ordering};
use crate::crypto::{Sha256, Sha512, ChaCha20};

pub trait Zeroize {
    fn zeroize(&mut self);
}

pub fn volatile_zero_slice(slice: &mut [u8]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    for i in 0..len {
        unsafe {
            std::ptr::write_volatile(ptr.add(i), 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

pub fn volatile_zero_u32_slice(slice: &mut [u32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    for i in 0..len {
        unsafe {
            std::ptr::write_volatile(ptr.add(i), 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

pub fn volatile_zero_u64_slice(slice: &mut [u64]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    for i in 0..len {
        unsafe {
            std::ptr::write_volatile(ptr.add(i), 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

impl Zeroize for Sha256 {
    fn zeroize(&mut self) {
        volatile_zero_u32_slice(&mut self.state);
        volatile_zero_slice(&mut self.buffer);
        unsafe {
            std::ptr::write_volatile(&mut self.len, 0);
        }
        compiler_fence(Ordering::SeqCst);
    }
}

impl Zeroize for Sha512 {
    fn zeroize(&mut self) {
        volatile_zero_u64_slice(&mut self.state);
        volatile_zero_slice(&mut self.buffer);
        unsafe {
            std::ptr::write_volatile(&mut self.len, 0);
        }
        compiler_fence(Ordering::SeqCst);
    }
}

impl Zeroize for ChaCha20 {
    fn zeroize(&mut self) {
        volatile_zero_u32_slice(&mut self.state);
    }
}
