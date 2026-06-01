use crate::ocel::ZeroCopyOcel;
use crate::sandbox::{GasMeter, RecursionGuard};

#[derive(Debug, Clone)]
pub struct OcpqQuery {
    pub activity_1: String,
    pub activity_2: String,
    pub delta_t_max_us: i64,
}

impl OcpqQuery {
    pub fn parse(query_str: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = query_str.split(',').collect();
        if parts.len() < 3 {
            return Err("Invalid query format. Expected: activity_1,activity_2,delta_t_max_us");
        }
        let activity_1 = parts[0].trim().to_string();
        let activity_2 = parts[1].trim().to_string();
        let delta_t_max_us = parts[2]
            .trim()
            .parse::<i64>()
            .map_err(|_| "Invalid delta_t value")?;
        Ok(Self {
            activity_1,
            activity_2,
            delta_t_max_us,
        })
    }
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub match_count: u32,
    pub matches: Vec<OcpqMatch>,
}

#[derive(Debug, Clone)]
pub struct OcpqMatch {
    pub event_1_id: String,
    pub event_2_id: String,
    pub object_id: String,
    pub duration_us: i64,
}

// Inverted Object-to-Event (O2E) index structure built in transient memory
struct O2eIndex {
    // Array of event list offsets for each object index
    offsets: Vec<u32>,
    // Contiguous event indices
    event_indices: Vec<u32>,
}

impl O2eIndex {
    fn build(ocel: &ZeroCopyOcel, gas_meter: &mut GasMeter) -> Result<Self, u32> {
        let obj_count = ocel.objects_count() as usize;
        let mut object_event_counts = vec![0u32; obj_count];
        
        // Count events per object
        for event_idx in 0..ocel.events_count() {
            gas_meter.consume(10)?; // 10 cycles per event scan
            let related_objs = ocel.get_event_objects(event_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
            for &obj_idx in related_objs {
                if (obj_idx as usize) < obj_count {
                    let val = object_event_counts[obj_idx as usize];
                    let next_val = val.checked_add(1).ok_or(crate::sandbox::ERR_LIFECYCLE_VIOLATION)?;
                    object_event_counts[obj_idx as usize] = next_val;
                }
            }
        }

        // Compute offsets
        let mut offsets = vec![0u32; obj_count];
        let mut current_offset = 0u32;
        for i in 0..obj_count {
            offsets[i] = current_offset;
            current_offset = current_offset.checked_add(object_event_counts[i]).ok_or(crate::sandbox::ERR_LIFECYCLE_VIOLATION)?;
        }

        // Fill event indices array
        let mut event_indices = vec![0u32; current_offset as usize];
        let mut write_offsets = offsets.clone();

        for event_idx in 0..ocel.events_count() {
            gas_meter.consume(15)?;
            let related_objs = ocel.get_event_objects(event_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
            for &obj_idx in related_objs {
                if (obj_idx as usize) < obj_count {
                    let write_pos = write_offsets[obj_idx as usize] as usize;
                    if write_pos >= event_indices.len() {
                        return Err(crate::sandbox::ERR_LIFECYCLE_VIOLATION);
                    }
                    event_indices[write_pos] = event_idx;
                    let next_offset = write_offsets[obj_idx as usize].checked_add(1).ok_or(crate::sandbox::ERR_LIFECYCLE_VIOLATION)?;
                    write_offsets[obj_idx as usize] = next_offset;
                }
            }
        }

        Ok(Self {
            offsets,
            event_indices,
        })
    }

    fn get_events_for_object(&self, obj_idx: u32, _ocel: &ZeroCopyOcel) -> &[u32] {
        let idx = obj_idx as usize;
        if idx >= self.offsets.len() {
            return &[];
        }
        let start = self.offsets[idx] as usize;
        let end = if idx + 1 < self.offsets.len() {
            self.offsets[idx + 1] as usize
        } else {
            self.event_indices.len()
        };
        if start > end || end > self.event_indices.len() {
            return &[];
        }
        &self.event_indices[start..end]
    }
}

pub fn execute_ocpq_query(
    ocel: &ZeroCopyOcel,
    query: &OcpqQuery,
    gas_meter: &mut GasMeter,
    recursion_guard: &mut RecursionGuard,
) -> Result<QueryResult, u32> {
    // 1. Build inverted index in transient memory to avoid quadratic scanning
    let o2e = O2eIndex::build(ocel, gas_meter)?;
    
    let mut matches = Vec::new();
    let mut match_count = 0;

    // 2. Traversal
    for e1_idx in 0..ocel.events_count() {
        gas_meter.consume(50)?; // 50 cycles per event query evaluation
        
        let act1 = ocel.get_event_activity(e1_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
        if act1 != query.activity_1 {
            continue;
        }

        let e1_ts = ocel.get_event_timestamp(e1_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
        let related_objs = ocel.get_event_objects(e1_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;

        for &obj_idx in related_objs {
            gas_meter.consume(30)?;
            recursion_guard.enter()?;

            let obj_id = ocel.get_object_id(obj_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
            let candidate_events = o2e.get_events_for_object(obj_idx, ocel);

            for &e2_idx in candidate_events {
                if e1_idx == e2_idx {
                    continue;
                }
                gas_meter.consume(20)?;

                let act2 = ocel.get_event_activity(e2_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
                if act2 != query.activity_2 {
                    continue;
                }

                let e2_ts = ocel.get_event_timestamp(e2_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
                if e1_ts < e2_ts {
                    let diff = e2_ts - e1_ts;
                    if diff <= query.delta_t_max_us {
                        let e1_id = ocel.get_event_id(e1_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
                        let e2_id = ocel.get_event_id(e2_idx).map_err(|_| crate::sandbox::ERR_QUERY_TIMEOUT)?;
                        
                        matches.push(OcpqMatch {
                            event_1_id: e1_id.to_string(),
                            event_2_id: e2_id.to_string(),
                            object_id: obj_id.to_string(),
                            duration_us: diff,
                        });
                        match_count += 1;

                        // Enforce match limits to prevent transient heap overflow
                        if match_count >= 1000 {
                            recursion_guard.exit();
                            return Ok(QueryResult { match_count, matches });
                        }
                    }
                }
            }

            recursion_guard.exit();
        }
    }

    Ok(QueryResult { match_count, matches })
}
