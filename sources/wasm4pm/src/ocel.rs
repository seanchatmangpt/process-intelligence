// Zero-Copy OCEL 2.0 Binary Parser and Index Traversal Spec

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcelError {
    InvalidMagic,
    InvalidVersion,
    OutOfBounds,
    Utf8Error,
    NullPointer,
}

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
            let end = offset as usize + size;
            if end > data.len() {
                Err(OcelError::OutOfBounds)
            } else {
                Ok(())
            }
        };

        // Events section size: 24 bytes per event
        check_bound(events_offset, events_count as usize * 24)?;
        // Objects section size: 12 bytes per object
        check_bound(objects_offset, objects_count as usize * 12)?;
        // String table size
        check_bound(string_table_offset, string_table_size as usize)?;

        // Index tables contain entry arrays first (8 bytes per entry)
        check_bound(e2o_offset, events_count as usize * 8)?;
        check_bound(o2o_offset, objects_count as usize * 8)?;

        Ok(Self {
            data,
            events_count,
            events_offset,
            objects_count,
            objects_offset,
            e2o_offset,
            o2o_offset,
            string_table_offset,
            string_table_size,
        })
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
        let abs_offset = self.string_table_offset as usize + offset as usize;
        
        // Ensure we can read length (4 bytes)
        if abs_offset + 4 > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }
        let len = u32::from_le_bytes(self.data[abs_offset..abs_offset + 4].try_into().unwrap()) as usize;
        
        if abs_offset + 4 + len > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }

        let slice = &self.data[abs_offset + 4..abs_offset + 4 + len];
        std::str::from_utf8(slice).map_err(|_| OcelError::Utf8Error)
    }

    // Fetch Event metadata
    pub fn get_event_id(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = self.events_offset as usize + index as usize * 24;
        let id_offset = u32::from_le_bytes(self.data[offset..offset + 4].try_into().unwrap());
        self.get_string(id_offset)
    }

    pub fn get_event_activity(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = self.events_offset as usize + index as usize * 24;
        let act_offset = u32::from_le_bytes(self.data[offset + 4..offset + 8].try_into().unwrap());
        self.get_string(act_offset)
    }

    pub fn get_event_timestamp(&self, index: u32) -> Result<i64, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = self.events_offset as usize + index as usize * 24;
        let timestamp = i64::from_le_bytes(self.data[offset + 8..offset + 16].try_into().unwrap());
        Ok(timestamp)
    }

    pub fn get_event_type(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = self.events_offset as usize + index as usize * 24;
        let type_offset = u32::from_le_bytes(self.data[offset + 16..offset + 20].try_into().unwrap());
        self.get_string(type_offset)
    }

    // Fetch Object metadata
    pub fn get_object_id(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.objects_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = self.objects_offset as usize + index as usize * 12;
        let id_offset = u32::from_le_bytes(self.data[offset..offset + 4].try_into().unwrap());
        self.get_string(id_offset)
    }

    pub fn get_object_type(&self, index: u32) -> Result<&'a str, OcelError> {
        if index >= self.objects_count {
            return Err(OcelError::OutOfBounds);
        }
        let offset = self.objects_offset as usize + index as usize * 12;
        let type_offset = u32::from_le_bytes(self.data[offset + 4..offset + 8].try_into().unwrap());
        self.get_string(type_offset)
    }

    // Traversal: Event-to-Object (E2O) index
    pub fn get_event_objects(&self, event_idx: u32) -> Result<&'a [u32], OcelError> {
        if event_idx >= self.events_count {
            return Err(OcelError::OutOfBounds);
        }
        
        let entry_offset = self.e2o_offset as usize + event_idx as usize * 8;
        let array_offset = u32::from_le_bytes(self.data[entry_offset..entry_offset + 4].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(self.data[entry_offset + 4..entry_offset + 8].try_into().unwrap()) as usize;

        let abs_start = self.e2o_offset as usize + array_offset;
        let abs_end = abs_start + count * 4;

        if abs_end > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }

        // Transmute/parse the u32 slice safely
        let slice = &self.data[abs_start..abs_end];
        let ptr = slice.as_ptr() as *const u32;
        
        // Ensure proper alignment for safe dereferencing
        if (ptr as usize) % 4 != 0 {
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

        let entry_offset = self.o2o_offset as usize + object_idx as usize * 8;
        let array_offset = u32::from_le_bytes(self.data[entry_offset..entry_offset + 4].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(self.data[entry_offset + 4..entry_offset + 8].try_into().unwrap()) as usize;

        let abs_start = self.o2o_offset as usize + array_offset;
        let abs_end = abs_start + count * 4;

        if abs_end > self.data.len() {
            return Err(OcelError::OutOfBounds);
        }

        let slice = &self.data[abs_start..abs_end];
        let ptr = slice.as_ptr() as *const u32;
        
        if (ptr as usize) % 4 != 0 {
            return Err(OcelError::InvalidMagic);
        }

        let u32_slice = unsafe { std::slice::from_raw_parts(ptr, count) };
        Ok(u32_slice)
    }
}
