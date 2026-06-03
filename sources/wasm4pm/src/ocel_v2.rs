// Zero-Copy OCEL 2.0 Binary Parser V2 and Flattener
// Grounded in OCEDO/OCPQ invariants and loss-aware projection laws.

use crate::ocel::OcelError;

#[derive(Debug)]
pub struct ZeroCopyOcelV2<'a> {
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

impl<'a> ZeroCopyOcelV2<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, OcelError> {
        if data.len() < 40 {
            return Err(OcelError::OutOfBounds);
        }

        // 1. Validate magic "OCEL"
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

        // Boundary checks
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

        // Strict conformance validation of OCEDO/OCPQ invariants
        ocel.validate()?;

        Ok(ocel)
    }

    pub fn events_count(&self) -> u32 {
        self.events_count
    }

    pub fn objects_count(&self) -> u32 {
        self.objects_count
    }

    pub fn get_string(&self, offset: u32) -> Result<&'a str, OcelError> {
        if offset >= self.string_table_size {
            return Err(OcelError::OutOfBounds);
        }
        let abs_offset = (self.string_table_offset as usize)
            .checked_add(offset as usize)
            .ok_or(OcelError::OutOfBounds)?;
        
        let end_len = abs_offset.checked_add(4).ok_or(OcelError::OutOfBounds)?;
        if end_len > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let len = u32::from_le_bytes(self.data[abs_offset..end_len].try_into().unwrap()) as usize;
        
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

        let slice = &self.data[abs_start..abs_end];
        let ptr = slice.as_ptr() as *const u32;
        
        if !(ptr as usize).is_multiple_of(4) {
            return Err(OcelError::InvalidMagic);
        }

        let u32_slice = unsafe { std::slice::from_raw_parts(ptr, count) };
        Ok(u32_slice)
    }

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

// =========================================================================
// FLATTENER & LOSS POLICY IMPLEMENTATION
// =========================================================================

#[derive(Debug, Clone)]
pub struct FlatEvent {
    pub event_id: String,
    pub activity: String,
    pub timestamp: i64,
    pub event_type: String,
}

#[derive(Debug, Clone)]
pub struct FlatCase {
    pub case_id: String,
    pub events: Vec<FlatEvent>,
}

#[derive(Debug, Clone)]
pub struct FlatLog {
    pub cases: Vec<FlatCase>,
}

#[derive(Debug, Clone)]
pub struct LossStructuralChanges {
    pub discarded_o2o_links_count: usize,
    pub pruned_e2o_links_count: usize,
    pub duplicate_events_created: usize,
    pub pruned_events_count: usize,
}

#[derive(Debug, Clone)]
pub struct LossReport {
    pub loss_report_id: String,
    pub timestamp: String,
    pub source_format: String,
    pub target_format: String,
    pub structural_changes: LossStructuralChanges,
}

impl LossReport {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"loss_report_id":"{}","timestamp":"{}","source_format":"{}","target_format":"{}","structural_changes":{{"discarded_o2o_links_count":{},"pruned_e2o_links_count":{},"duplicate_events_created":{},"pruned_events_count":{}}}}}"#,
            self.loss_report_id,
            self.timestamp,
            self.source_format,
            self.target_format,
            self.structural_changes.discarded_o2o_links_count,
            self.structural_changes.pruned_e2o_links_count,
            self.structural_changes.duplicate_events_created,
            self.structural_changes.pruned_events_count
        )
    }
}

/// Flattens the OCEL V2 log onto a single target object type.
/// Computes convergence (duplicated events), divergence (dropped events), and discards O2O.
pub fn flatten(ocel: &ZeroCopyOcelV2, target_object_type: &str) -> Result<(FlatLog, LossReport), OcelError> {
    let mut cases = Vec::new();
    let mut target_objects = Vec::new();
    
    // 1. Identify all objects matching the target type
    let mut target_type_exists = false;
    for obj_idx in 0..ocel.objects_count() {
        let ty = ocel.get_object_type(obj_idx)?;
        if ty == target_object_type {
            target_type_exists = true;
            target_objects.push(obj_idx);
        }
    }

    if !target_type_exists {
        return Err(OcelError::UnknownObjectType);
    }

    // 2. Count discarded O2O links
    let mut discarded_o2o = 0;
    for obj_idx in 0..ocel.objects_count() {
        let related = ocel.get_object_related_objects(obj_idx)?;
        discarded_o2o += related.len();
    }

    // 3. Build cases
    let mut pruned_events = 0;
    let mut duplicate_events = 0;
    let mut pruned_e2o_links = 0;

    for &target_obj_idx in &target_objects {
        let case_id = ocel.get_object_id(target_obj_idx)?.to_string();
        let mut case_events = Vec::new();

        for event_idx in 0..ocel.events_count() {
            let related_objs = ocel.get_event_objects(event_idx)?;
            
            // Check if target object is linked to this event
            if related_objs.contains(&target_obj_idx) {
                let event_id = ocel.get_event_id(event_idx)?.to_string();
                let activity = ocel.get_event_activity(event_idx)?.to_string();
                let timestamp = ocel.get_event_timestamp(event_idx)?;
                let event_type = ocel.get_event_type(event_idx)?.to_string();
                
                case_events.push(FlatEvent {
                    event_id,
                    activity,
                    timestamp,
                    event_type,
                });
            }
        }

        // Sort events in case by (timestamp, event_id)
        case_events.sort_by(|a, b| {
            a.timestamp.cmp(&b.timestamp).then_with(|| a.event_id.cmp(&b.event_id))
        });

        cases.push(FlatCase {
            case_id,
            events: case_events,
        });
    }

    // 4. Compute structural changes metrics
    for event_idx in 0..ocel.events_count() {
        let related_objs = ocel.get_event_objects(event_idx)?;
        
        // Count target objects linked to this event
        let mut target_links = 0;
        let mut other_links = 0;
        
        for &obj_idx in related_objs {
            let ty = ocel.get_object_type(obj_idx)?;
            if ty == target_object_type {
                target_links += 1;
            } else {
                other_links += 1;
            }
        }

        pruned_e2o_links += other_links;

        if target_links == 0 {
            pruned_events += 1;
        } else if target_links > 1 {
            duplicate_events += target_links - 1;
        }
    }

    let log = FlatLog { cases };
    let loss_report = LossReport {
        loss_report_id: "lr-ocel-flattening-v2".to_string(),
        timestamp: "2026-06-01T16:19:51-07:00".to_string(),
        source_format: "OCEL2.0-Binary".to_string(),
        target_format: "Flat-XES-Compatible".to_string(),
        structural_changes: LossStructuralChanges {
            discarded_o2o_links_count: discarded_o2o,
            pruned_e2o_links_count: pruned_e2o_links,
            duplicate_events_created: duplicate_events,
            pruned_events_count: pruned_events,
        },
    };

    Ok((log, loss_report))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_ocel_buffer() -> Vec<u8> {
        let mut buf = vec![0u8; 256];

        // Magic "OCEL"
        buf[0..4].copy_from_slice(&0x4F43454Cu32.to_be_bytes());
        // Version 2
        buf[4..8].copy_from_slice(&2u32.to_le_bytes());

        // Section Offsets
        let events_offset = 140u32;
        let events_count = 2u32;
        let objects_offset = 188u32;
        let objects_count = 2u32; // 2 objects now (Object 0: Order, Object 1: Item)
        let e2o_offset = 212u32;
        let o2o_offset = 236u32;
        let string_table_offset = 40u32;
        let string_table_size = 100u32;

        buf[8..12].copy_from_slice(&events_offset.to_le_bytes());
        buf[12..16].copy_from_slice(&events_count.to_le_bytes());
        buf[16..20].copy_from_slice(&objects_offset.to_le_bytes());
        buf[20..24].copy_from_slice(&objects_count.to_le_bytes());
        buf[24..28].copy_from_slice(&e2o_offset.to_le_bytes());
        buf[28..32].copy_from_slice(&o2o_offset.to_le_bytes());
        buf[32..36].copy_from_slice(&string_table_offset.to_le_bytes());
        buf[36..40].copy_from_slice(&string_table_size.to_le_bytes());

        // String Table at 40
        // Offset 0: "e1"
        buf[40..44].copy_from_slice(&2u32.to_le_bytes());
        buf[44..46].copy_from_slice(b"e1");
        
        // Offset 8: "e2"
        buf[48..52].copy_from_slice(&2u32.to_le_bytes());
        buf[52..54].copy_from_slice(b"e2");

        // Offset 16: "create_order"
        buf[56..60].copy_from_slice(&12u32.to_le_bytes());
        buf[60..72].copy_from_slice(b"create_order");

        // Offset 32: "approve_order"
        buf[72..76].copy_from_slice(&13u32.to_le_bytes());
        buf[76..89].copy_from_slice(b"approve_order");

        // Offset 52: "order_1"
        buf[92..96].copy_from_slice(&7u32.to_le_bytes());
        buf[96..103].copy_from_slice(b"order_1");

        // Offset 64: "Order"
        buf[104..108].copy_from_slice(&5u32.to_le_bytes());
        buf[108..113].copy_from_slice(b"Order");

        // Write Events at 140
        // Event 0: id_offset=0 ("e1"), act_offset=16 ("create_order"), ts=1000, type_offset=64 ("Order"), attr_count=0
        buf[140..144].copy_from_slice(&0u32.to_le_bytes());
        buf[144..148].copy_from_slice(&16u32.to_le_bytes());
        buf[148..156].copy_from_slice(&1000i64.to_le_bytes());
        buf[156..160].copy_from_slice(&64u32.to_le_bytes());
        buf[160..162].copy_from_slice(&0u16.to_le_bytes());

        // Event 1: id_offset=8 ("e2"), act_offset=32 ("approve_order"), ts=5000, type_offset=64 ("Order"), attr_count=0
        buf[164..168].copy_from_slice(&8u32.to_le_bytes());
        buf[168..172].copy_from_slice(&32u32.to_le_bytes());
        buf[172..180].copy_from_slice(&5000i64.to_le_bytes());
        buf[180..184].copy_from_slice(&64u32.to_le_bytes());
        buf[184..186].copy_from_slice(&0u16.to_le_bytes());

        // Write Objects at 188 (12 bytes per object)
        // Object 0: id_offset=52 ("order_1"), type_offset=64 ("Order"), attr_count=0
        buf[188..192].copy_from_slice(&52u32.to_le_bytes());
        buf[192..196].copy_from_slice(&64u32.to_le_bytes());
        buf[196..198].copy_from_slice(&0u16.to_le_bytes());

        // Object 1: id_offset=52 ("order_1"), type_offset=64 ("Order"), attr_count=0
        buf[200..204].copy_from_slice(&52u32.to_le_bytes());
        buf[204..208].copy_from_slice(&64u32.to_le_bytes());
        buf[208..210].copy_from_slice(&0u16.to_le_bytes());

        // Write E2O at 212
        // Event 0: offset=16, count=1
        // Event 1: offset=20, count=1
        buf[212..216].copy_from_slice(&16u32.to_le_bytes());
        buf[216..220].copy_from_slice(&1u32.to_le_bytes());
        buf[220..224].copy_from_slice(&20u32.to_le_bytes());
        buf[224..228].copy_from_slice(&1u32.to_le_bytes());
        // Element array starts at E2O offset + 16 = 228
        buf[228..232].copy_from_slice(&0u32.to_le_bytes()); // Event 0 -> Object 0
        buf[232..236].copy_from_slice(&0u32.to_le_bytes()); // Event 1 -> Object 0

        // Write O2O at 236
        // Object 0: offset=16, count=1 (Object 0 -> Object 1)
        // Object 1: offset=20, count=0
        buf[236..240].copy_from_slice(&16u32.to_le_bytes());
        buf[240..244].copy_from_slice(&1u32.to_le_bytes());
        buf[244..248].copy_from_slice(&20u32.to_le_bytes());
        buf[248..252].copy_from_slice(&0u32.to_le_bytes());
        // Elements array starts at O2O offset + 16 = 252
        buf[252..256].copy_from_slice(&1u32.to_le_bytes()); // Object 0 -> Object 1

        buf
    }

    #[test]
    fn test_ocel_v2_parsing_and_validation() {
        let buf = build_test_ocel_buffer();
        let ocel = ZeroCopyOcelV2::parse(&buf).unwrap();
        assert_eq!(ocel.events_count(), 2);
        assert_eq!(ocel.objects_count(), 2);

        // Test referential integrity
        let e0_objs = ocel.get_event_objects(0).unwrap();
        assert_eq!(e0_objs, &[0]);
        let o0_objs = ocel.get_object_related_objects(0).unwrap();
        assert_eq!(o0_objs, &[1]);
    }

    #[test]
    fn test_ocel_v2_dangling_reference_rejection() {
        let mut buf = build_test_ocel_buffer();
        // Modify E2O element for Event 0 to point to Object 99 (dangling)
        buf[228..232].copy_from_slice(&99u32.to_le_bytes());

        let res = ZeroCopyOcelV2::parse(&buf);
        assert_eq!(res.unwrap_err(), OcelError::DanglingReference);
    }

    #[test]
    fn test_ocel_v2_empty_event_objects_rejection() {
        let mut buf = build_test_ocel_buffer();
        // Set Event 0 E2O link count = 0
        buf[216..220].copy_from_slice(&0u32.to_le_bytes());

        let res = ZeroCopyOcelV2::parse(&buf);
        assert_eq!(res.unwrap_err(), OcelError::EmptyEventObjects);
    }

    #[test]
    fn test_ocel_v2_cycle_rejection() {
        let mut buf = build_test_ocel_buffer();
        // Make Object 1 point to Object 0, creating a cycle: Object 0 -> Object 1 -> Object 0
        buf[244..248].copy_from_slice(&16u32.to_le_bytes());
        buf[248..252].copy_from_slice(&1u32.to_le_bytes());

        let res = ZeroCopyOcelV2::parse(&buf);
        assert_eq!(res.unwrap_err(), OcelError::CycleDetected);
    }

    #[test]
    fn test_ocel_v2_temporal_anomaly_rejection() {
        let mut buf = build_test_ocel_buffer();
        // Change Event 1 timestamp to 500 (which is before Event 0 timestamp 1000)
        buf[172..180].copy_from_slice(&500i64.to_le_bytes());

        let res = ZeroCopyOcelV2::parse(&buf);
        assert_eq!(res.unwrap_err(), OcelError::TemporalAnomaly);
    }

    #[test]
    fn test_ocel_v2_invalid_metadata_string_rejection() {
        // Test 1: Out of bounds string reference
        let mut buf = build_test_ocel_buffer();
        // Event 0 activity offset is at buf[144..148]. Make it point to 999 (out of bounds of string table size 100)
        buf[144..148].copy_from_slice(&999u32.to_le_bytes());
        let res = ZeroCopyOcelV2::parse(&buf);
        assert_eq!(res.unwrap_err(), OcelError::OutOfBounds);

        // Test 2: Invalid UTF-8 in string table
        let mut buf2 = build_test_ocel_buffer();
        // The string for "e1" (event 0 id) is at offset 44 (2 bytes). Replace with invalid UTF-8 (0xFF, 0xFF)
        buf2[44] = 0xFF;
        buf2[45] = 0xFF;
        let res2 = ZeroCopyOcelV2::parse(&buf2);
        assert_eq!(res2.unwrap_err(), OcelError::Utf8Error);
    }

    #[test]
    fn test_ocel_v2_flattener() {
        let buf = build_test_ocel_buffer();
        let ocel = ZeroCopyOcelV2::parse(&buf).unwrap();

        let (flat_log, loss_report) = flatten(&ocel, "Order").unwrap();
        assert_eq!(flat_log.cases.len(), 2);
        assert_eq!(flat_log.cases[0].events.len(), 2);
        assert_eq!(flat_log.cases[0].events[0].activity, "create_order");

        // Verify loss report
        assert_eq!(loss_report.structural_changes.discarded_o2o_links_count, 1);
        assert_eq!(loss_report.structural_changes.duplicate_events_created, 0);
        assert_eq!(loss_report.structural_changes.pruned_events_count, 0);
        
        let json = loss_report.to_json();
        assert!(json.contains("\"discarded_o2o_links_count\":1"));
    }
}

