/// FfiSafetyChecker performs boundary safety checks for FFI inputs.
pub struct FfiSafetyChecker;

impl FfiSafetyChecker {
    /// Checks for pointer arithmetic overflow or overflowing addition.
    pub fn check_overflow(ptr: *const u8, len: usize) -> bool {
        let start = ptr as usize;
        if let Some(end) = start.checked_add(len) {
            // Additional check: end must be >= start
            end >= start
        } else {
            false
        }
    }

    /// Checks if a pointer aligns with the matching requirement.
    pub fn check_alignment(ptr: *const u8, alignment: usize) -> bool {
        if alignment == 0 || !alignment.is_power_of_two() {
            return false;
        }
        (ptr as usize).is_multiple_of(alignment)
    }

    /// Checks if the pointer region is completely contained within the global arena.
    pub fn check_arena_containment(ptr: *const u8, len: usize) -> bool {
        if !Self::check_overflow(ptr, len) {
            return false;
        }
        crate::allocator::validate_pointer(ptr, len)
    }

    /// Checks that the pointer region lies completely within either the permanent
    /// or transient partition of the global arena, and does not cross the boundary.
    pub fn check_partition_boundaries(ptr: *const u8, len: usize) -> bool {
        if !Self::check_overflow(ptr, len) {
            return false;
        }
        let start_addr = ptr as usize;
        let end_addr = start_addr + len;

        if let Some(bounds) = crate::allocator::get_arena_boundaries() {
            let perm_start = bounds.base_addr;
            let perm_end = perm_start + bounds.transient_start;
            let trans_start = perm_end;
            let trans_end = perm_start + bounds.ceiling;

            // Must be completely inside permanent OR completely inside transient
            let in_permanent = start_addr >= perm_start && end_addr <= perm_end;
            let in_transient = start_addr >= trans_start && end_addr <= trans_end;

            in_permanent || in_transient
        } else {
            false
        }
    }

    /// Verifies that two pointer ranges do not overlap.
    pub fn check_disjoint(ptr1: *const u8, len1: usize, ptr2: *const u8, len2: usize) -> bool {
        if !Self::check_overflow(ptr1, len1) || !Self::check_overflow(ptr2, len2) {
            return false;
        }
        let start1 = ptr1 as usize;
        let end1 = start1 + len1;
        let start2 = ptr2 as usize;
        let end2 = start2 + len2;

        end1 <= start2 || end2 <= start1
    }

    /// Run all safety checks on a single slice pointer range.
    pub fn check_slice(ptr: *const u8, len: usize, alignment: usize) -> bool {
        Self::check_overflow(ptr, len)
            && Self::check_alignment(ptr, alignment)
            && Self::check_arena_containment(ptr, len)
            && Self::check_partition_boundaries(ptr, len)
    }
}
