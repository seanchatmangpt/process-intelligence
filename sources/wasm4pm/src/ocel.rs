// Zero-Copy OCEL 2.0 Binary Parser and Index Traversal Spec

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcelError {
    InvalidMagic,
    InvalidVersion,
    OutOfBounds,
    Utf8Error,
    NullPointer,
    DanglingReference,
    EmptyEventObjects,
    CycleDetected,
    TemporalAnomaly,
    UnknownObjectType,
}


#[derive(Debug)]
pub struct ZeroCopyOcel<'a> {
    data: &'a [u8],
    events_count: u32,
    events_offset: u32,
    objects_count: u32,
    objects_offset: u32,
    e2o_offset: u32,
    o2o_offset: u32,
    string_table_offset: u32,
    string_table_size: u32,
}

impl<'a> ZeroCopyOcel<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, OcelError> {
        if data.len() < 40 {
            return Err(OcelError::OutOfBounds);
        }

        // 1. Validate magic "OCEL" = [0x4F, 0x43, 0x45, 0x4C] in big-endian or little-endian.
        // Let's enforce big-endian magic: 0x4F43454C
        let magic = u32::from_be_bytes(data[0..4].try_into().unwrap());
        if magic != 0x4F43454C {
            return Err(OcelError::InvalidMagic);
        }

        // 2. Validate version
        let version = u32::from_le_bytes(data[4..8].try_into().unwrap());
        if version != 2 {
            return Err(OcelError::InvalidVersion);
        }

        // 3. Read offsets
        let events_offset = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let events_count = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let objects_offset = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let objects_count = u32::from_le_bytes(data[20..24].try_into().unwrap());
        let e2o_offset = u32::from_le_bytes(data[24..28].try_into().unwrap());
        let o2o_offset = u32::from_le_bytes(data[28..32].try_into().unwrap());
        let string_table_offset = u32::from_le_bytes(data[32..36].try_into().unwrap());
        let string_table_size = u32::from_le_bytes(data[36..40].try_into().unwrap());

        // Proactive boundary check: ensure all sections fit in data
        let check_bound = |offset: u32, size: usize| -> Result<(), OcelError> {
            let offset_usize = offset as usize;
            let end = offset_usize.checked_add(size).ok_or(OcelError::OutOfBounds)?;
            if end > data.len() {
                Err(OcelError::OutOfBounds)
            } else {
                Ok(())
            }
        };

        // Events section size: 24 bytes per event
        let events_size = (events_count as usize).checked_mul(24).ok_or(OcelError::OutOfBounds)?;
        check_bound(events_offset, events_size)?;
        
        // Objects section size: 12 bytes per object
        let objects_size = (objects_count as usize).checked_mul(12).ok_or(OcelError::OutOfBounds)?;
        check_bound(objects_offset, objects_size)?;
        
        // String table size
        check_bound(string_table_offset, string_table_size as usize)?;

        // Index tables contain entry arrays first (8 bytes per entry)
        let e2o_size = (events_count as usize).checked_mul(8).ok_or(OcelError::OutOfBounds)?;
        check_bound(e2o_offset, e2o_size)?;
        
        let o2o_size = (objects_count as usize).checked_mul(8).ok_or(OcelError::OutOfBounds)?;
        check_bound(o2o_offset, o2o_size)?;

        let ocel = Self {
            data,
            events_count,
            events_offset,
            objects_count,
            objects_offset,
            e2o_offset,
            o2o_offset,
            string_table_offset,
            string_table_size,
        };

        ocel.validate()?;

        Ok(ocel)
    }

    pub fn events_count(&self) -> u32 {
        self.events_count
    }

    pub fn objects_count(&self) -> u32 {
        self.objects_count
    }

    // Get string from string table offset
    pub fn get_string(&self, offset: u32) -> Result<&'a str, OcelError> {
        if offset >= self.string_table_size {
            return Err(OcelError::OutOfBounds);
        }
        let abs_offset = (self.string_table_offset as usize)
            .checked_add(offset as usize)
            .ok_or(OcelError::OutOfBounds)?;
        
        // Ensure we can read length (4 bytes)
        let end_len = abs_offset.checked_add(4).ok_or(OcelError::OutOfBounds)?;
        if end_len > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let len = u32::from_le_bytes(self.data[abs_offset..end_len].try_into().unwrap()) as usize;
        
        // Strict boundary check: the string must reside entirely within the string table
        let end_offset_in_table = offset.checked_add(4)
            .and_then(|val| val.checked_add(len as u32))
            .ok_or(OcelError::OutOfBounds)?;
        if end_offset_in_table > self.string_table_size {
            return Err(OcelError::OutOfBounds);
        }

        let end_slice = end_len.checked_add(len).ok_or(OcelError::OutOfBounds)?;
        if end_slice > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }

        let slice = &self.data[end_len..end_slice];
        std::str::from_utf8(slice).map_err(|_| OcelError::Utf8Error)
    }

    // Fetch Event metadata
    pub fn get_event_id(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = (index as usize)
            .checked_mul(24)
            .and_then(|val| (self.events_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;
        if offset.checked_add(4).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let id_offset = u32::from_le_bytes(self.data[offset..offset + 4].try_into().unwrap());
        self.get_string(id_offset)
    }

    pub fn get_event_activity(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = (index as usize)
            .checked_mul(24)
            .and_then(|val| (self.events_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;
        if offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let act_offset = u32::from_le_bytes(self.data[offset + 4..offset + 8].try_into().unwrap());
        self.get_string(act_offset)
    }

    pub fn get_event_timestamp(&self, index: u32) -> Result<i64, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = (index as usize)
            .checked_mul(24)
            .and_then(|val| (self.events_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;
        if offset.checked_add(16).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let timestamp = i64::from_le_bytes(self.data[offset + 8..offset + 16].try_into().unwrap());
        Ok(timestamp)
    }

    pub fn get_event_type(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = (index as usize)
            .checked_mul(24)
            .and_then(|val| (self.events_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;
        if offset.checked_add(20).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let type_offset = u32::from_le_bytes(self.data[offset + 16..offset + 20].try_into().unwrap());
        self.get_string(type_offset)
    }

    // Fetch Object metadata
    pub fn get_object_id(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.objects_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = (index as usize)
            .checked_mul(12)
            .and_then(|val| (self.objects_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;
        if offset.checked_add(4).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let id_offset = u32::from_le_bytes(self.data[offset..offset + 4].try_into().unwrap());
        self.get_string(id_offset)
    }

    pub fn get_object_type(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.objects_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = (index as usize)
            .checked_mul(12)
            .and_then(|val| (self.objects_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;
        if offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let type_offset = u32::from_le_bytes(self.data[offset + 4..offset + 8].try_into().unwrap());
        self.get_string(type_offset)
    }

    // Traversal: Event-to-Object (E2O) index
    pub fn get_event_objects(&self, event_idx: u32) -> Result<&'a [u32], OcelError> {
        if event_idx >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        
        let entry_offset = (event_idx as usize)
            .checked_mul(8)
            .and_then(|val| (self.e2o_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;

        if entry_offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        
        let array_offset = u32::from_le_bytes(self.data[entry_offset..entry_offset + 4].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(self.data[entry_offset + 4..entry_offset + 8].try_into().unwrap()) as usize;

        let abs_start = (self.e2o_offset as usize)
            .checked_add(array_offset)
            .ok_or(OcelError::OutOfBounds)?;
        let count_bytes = count.checked_mul(4).ok_or(OcelError::OutOfBounds)?;
        let abs_end = abs_start.checked_add(count_bytes).ok_or(OcelError::OutOfBounds)?;

        if abs_end > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }

        // Transmute/parse the u32 slice safely
        let slice = &self.data[abs_start..abs_end];
        let ptr = slice.as_ptr() as *const u32;
        
        // Ensure proper alignment for safe dereferencing
        if !(ptr as usize).is_multiple_of(4) {
            // If the buffer is not u32 aligned, we must not transmute.
            // Since we enforce strict zero-copy alignment layout, the host must align it,
            // otherwise we could trigger undefined behavior on some hardware.
            return Err(OcelError::InvalidMagic); // or generic alignment error
        }

        let u32_slice = unsafe { std::slice::from_raw_parts(ptr, count) };
        Ok(u32_slice)
    }

    // Traversal: Object-to-Object (O2O) index
    pub fn get_object_related_objects(&self, object_idx: u32) -> Result<&'a [u32], OcelError> {
        if object_idx >= self.objects_count {
            return Err(OcelError::OutOfBounds);
        }

        let entry_offset = (object_idx as usize)
            .checked_mul(8)
            .and_then(|val| (self.o2o_offset as usize).checked_add(val))
            .ok_or(OcelError::OutOfBounds)?;

        if entry_offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }

        let array_offset = u32::from_le_bytes(self.data[entry_offset..entry_offset + 4].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(self.data[entry_offset + 4..entry_offset + 8].try_into().unwrap()) as usize;

        let abs_start = (self.o2o_offset as usize)
            .checked_add(array_offset)
            .ok_or(OcelError::OutOfBounds)?;
        let count_bytes = count.checked_mul(4).ok_or(OcelError::OutOfBounds)?;
        let abs_end = abs_start.checked_add(count_bytes).ok_or(OcelError::OutOfBounds)?;

        if abs_end > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }

        let slice = &self.data[abs_start..abs_end];
        let ptr = slice.as_ptr() as *const u32;
        
        if !(ptr as usize).is_multiple_of(4) {
            return Err(OcelError::InvalidMagic);
        }

        let u32_slice = unsafe { std::slice::from_raw_parts(ptr, count) };
        Ok(u32_slice)
    }

    // Zero-Copy Bitmask Projection for Sub-DFGs
    // Ensures memory footprint is constant (O(1) allocations) by taking pre-allocated buffers
    pub fn compute_projected_dfg(
        &self,
        bitmask: &[u64],
        dfg_matrix: &mut [u32], // flat array of size activity_count * activity_count
        activity_offsets: &[u32], // sorted unique activity offsets in the string table
        last_event_for_object: &mut [i32], // scratch space of size objects_count, initialized to -1
    ) -> Result<(), OcelError> {
        // Clear the DFG matrix
        dfg_matrix.fill(0);
        // Clear the scratch space
        last_event_for_object.fill(-1);

        let act_count = activity_offsets.len();
        let expected_matrix_len = act_count.checked_mul(act_count).ok_or(OcelError::OutOfBounds)?;
        if dfg_matrix.len() < expected_matrix_len {
            return Err(OcelError::OutOfBounds);
        }
        if last_event_for_object.len() < self.objects_count as usize {
            return Err(OcelError::OutOfBounds);
        }

        // Helper to find the index of an activity offset
        let find_act_idx = |offset: u32| -> Option<usize> {
            activity_offsets.binary_search(&offset).ok()
        };

        // Scan all events
        for event_idx in 0..self.events_count {
            // Check if this event is active in the bitmask
            let word_idx = event_idx as usize / 64;
            let bit_idx = event_idx as usize % 64;
            if word_idx >= bitmask.len() {
                break;
            }
            if (bitmask[word_idx] & (1 << bit_idx)) == 0 {
                continue; // Event is masked out
            }

            // Get event activity offset
            let offset = (event_idx as usize)
                .checked_mul(24)
                .and_then(|val| (self.events_offset as usize).checked_add(val))
                .ok_or(OcelError::OutOfBounds)?;
            if offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
                return Err(OcelError::OutOfBounds);
            }
            let act_offset = u32::from_le_bytes(self.data[offset + 4..offset + 8].try_into().unwrap());
            let act_idx = match find_act_idx(act_offset) {
                Some(idx) => idx,
                None => continue, // Unknown activity (should not happen if activity_offsets is complete)
            };
 
            // Get related objects
            let related_objs = self.get_event_objects(event_idx)?;
            for &obj_idx in related_objs {
                if obj_idx as usize >= last_event_for_object.len() {
                    return Err(OcelError::OutOfBounds);
                }
                let prev_event_idx = last_event_for_object[obj_idx as usize];
                if prev_event_idx >= 0 {
                    // There was a previous active event for this object.
                    // Get its activity index
                    let prev_offset = (prev_event_idx as usize)
                        .checked_mul(24)
                        .and_then(|val| (self.events_offset as usize).checked_add(val))
                        .ok_or(OcelError::OutOfBounds)?;
                    if prev_offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
                        return Err(OcelError::OutOfBounds);
                    }
                    let prev_act_offset = u32::from_le_bytes(self.data[prev_offset + 4..prev_offset + 8].try_into().unwrap());
                    if let Some(prev_act_idx) = find_act_idx(prev_act_offset) {
                        // Increment transition frequency in DFG matrix: (prev_act_idx -> act_idx)
                        let matrix_idx = prev_act_idx.checked_mul(act_count)
                            .and_then(|val| val.checked_add(act_idx))
                            .ok_or(OcelError::OutOfBounds)?;
                        if matrix_idx < dfg_matrix.len() {
                            dfg_matrix[matrix_idx] = dfg_matrix[matrix_idx].saturating_add(1);
                        }
                    }
                }
                // Update last active event for this object
                last_event_for_object[obj_idx as usize] = event_idx as i32;
            }
        }
 
        Ok(())
    }

    // Zero-Copy Multi-Perspective DFG Projection
    // Projects active events onto a specific object type, recording process paths strictly for that type
    pub fn compute_multi_perspective_dfg(
        &self,
        bitmask: &[u64],
        target_object_type: &str,
        dfg_matrix: &mut [u32], // flat array of size activity_count * activity_count
        activity_offsets: &[u32], // sorted unique activity offsets in the string table
        last_event_for_object: &mut [i32], // scratch space of size objects_count, initialized to -1
    ) -> Result<(), OcelError> {
        // Clear the DFG matrix
        dfg_matrix.fill(0);
        // Clear the scratch space
        last_event_for_object.fill(-1);

        let act_count = activity_offsets.len();
        let expected_matrix_len = act_count.checked_mul(act_count).ok_or(OcelError::OutOfBounds)?;
        if dfg_matrix.len() < expected_matrix_len {
            return Err(OcelError::OutOfBounds);
        }
        if last_event_for_object.len() < self.objects_count as usize {
            return Err(OcelError::OutOfBounds);
        }

        // Helper to find the index of an activity offset
        let find_act_idx = |offset: u32| -> Option<usize> {
            activity_offsets.binary_search(&offset).ok()
        };

        // Scan all events
        for event_idx in 0..self.events_count {
            // Check if this event is active in the bitmask
            let word_idx = event_idx as usize / 64;
            let bit_idx = event_idx as usize % 64;
            if word_idx >= bitmask.len() {
                break;
            }
            if (bitmask[word_idx] & (1 << bit_idx)) == 0 {
                continue; // Event is masked out
            }

            // Get event activity offset
            let offset = (event_idx as usize)
                .checked_mul(24)
                .and_then(|val| (self.events_offset as usize).checked_add(val))
                .ok_or(OcelError::OutOfBounds)?;
            if offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
                return Err(OcelError::OutOfBounds);
            }
            let act_offset = u32::from_le_bytes(self.data[offset + 4..offset + 8].try_into().unwrap());
            let act_idx = match find_act_idx(act_offset) {
                Some(idx) => idx,
                None => continue,
            };

            // Get related objects
            let related_objs = self.get_event_objects(event_idx)?;
            for &obj_idx in related_objs {
                if obj_idx as usize >= last_event_for_object.len() {
                    return Err(OcelError::OutOfBounds);
                }

                // Filter by the requested object type
                let obj_type = self.get_object_type(obj_idx)?;
                if obj_type != target_object_type {
                    continue;
                }

                let prev_event_idx = last_event_for_object[obj_idx as usize];
                if prev_event_idx >= 0 {
                    // There was a previous active event for this object.
                    // Get its activity index
                    let prev_offset = (prev_event_idx as usize)
                        .checked_mul(24)
                        .and_then(|val| (self.events_offset as usize).checked_add(val))
                        .ok_or(OcelError::OutOfBounds)?;
                    if prev_offset.checked_add(8).ok_or(OcelError::OutOfBounds)? > self.data.len() {
                        return Err(OcelError::OutOfBounds);
                    }
                    let prev_act_offset = u32::from_le_bytes(self.data[prev_offset + 4..prev_offset + 8].try_into().unwrap());
                    if let Some(prev_act_idx) = find_act_idx(prev_act_offset) {
                        // Increment transition frequency in DFG matrix: (prev_act_idx -> act_idx)
                        let matrix_idx = prev_act_idx.checked_mul(act_count)
                            .and_then(|val| val.checked_add(act_idx))
                            .ok_or(OcelError::OutOfBounds)?;
                        if matrix_idx < dfg_matrix.len() {
                            dfg_matrix[matrix_idx] = dfg_matrix[matrix_idx].saturating_add(1);
                        }
                    }
                }
                // Update last active event for this object
                last_event_for_object[obj_idx as usize] = event_idx as i32;
            }
        }

        Ok(())
    }

    /// Validates all OCEDO/OCPQ structural and semantic invariants.
    pub fn validate(&self) -> Result<(), OcelError> {
        // 1. Metadata String Validity Check:
        // Ensure all events and objects have valid string metadata.
        for event_idx in 0..self.events_count {
            let _ = self.get_event_id(event_idx)?;
            let _ = self.get_event_activity(event_idx)?;
            let _ = self.get_event_type(event_idx)?;
        }
        for object_idx in 0..self.objects_count {
            let _ = self.get_object_id(object_idx)?;
            let _ = self.get_object_type(object_idx)?;
        }

        // 2. E2O Integrity & Cardinality Check:
        // Every event must link to at least one object (EmptyEventObjects check)
        // Every link must reference a valid object (DanglingReference check)
        for event_idx in 0..self.events_count {
            let related_objs = self.get_event_objects(event_idx)?;
            if related_objs.is_empty() {
                return Err(OcelError::EmptyEventObjects);
            }
            for &obj_idx in related_objs {
                if obj_idx >= self.objects_count {
                    return Err(OcelError::DanglingReference);
                }
            }
        }

        // 3. O2O Referential Integrity Check:
        // Every link must reference a valid object (DanglingReference check)
        for object_idx in 0..self.objects_count {
            let related_objs = self.get_object_related_objects(object_idx)?;
            for &related_idx in related_objs {
                if related_idx >= self.objects_count {
                    return Err(OcelError::DanglingReference);
                }
            }
        }

        // 4. Graph Acyclicity Check (O2O DAG):
        let mut visited = vec![0u8; self.objects_count as usize]; // 0=unvisited, 1=visiting, 2=visited
        for object_idx in 0..self.objects_count {
            if visited[object_idx as usize] == 0 {
                self.check_o2o_cycle(object_idx, &mut visited)?;
            }
        }

        // 5. Temporal Monotonicity Check:
        // For any sequence of events acting on a shared object, event timestamps must be non-decreasing.
        let mut last_event_time = vec![i64::MIN; self.objects_count as usize];
        for event_idx in 0..self.events_count {
            let time = self.get_event_timestamp(event_idx)?;
            let related_objs = self.get_event_objects(event_idx)?;
            for &obj_idx in related_objs {
                if last_event_time[obj_idx as usize] > time {
                    return Err(OcelError::TemporalAnomaly);
                }
                last_event_time[obj_idx as usize] = time;
            }
        }

        Ok(())
    }

    fn check_o2o_cycle(&self, obj_idx: u32, visited: &mut [u8]) -> Result<(), OcelError> {
        visited[obj_idx as usize] = 1;
        let related = self.get_object_related_objects(obj_idx)?;
        for &rel_idx in related {
            match visited[rel_idx as usize] {
                1 => return Err(OcelError::CycleDetected),
                0 => self.check_o2o_cycle(rel_idx, visited)?,
                _ => {}
            }
        }
        visited[obj_idx as usize] = 2;
        Ok(())
    }
}

