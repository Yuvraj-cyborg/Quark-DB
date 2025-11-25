use std::mem::size_of;

// Paper specifies 4KB pages for disk alignment [cite: 7]
pub const PAGE_SIZE: usize = 4096;

// Paper[cite: 171]: KV Meta is 8 bytes
// Packed as: KeySize(14b) | ValSize(14b) | Offset(16b) | Type(2b) | Fence(1b) | Ref(1b) | LookAhead(16b)
#[derive(Debug, Clone, Copy)]
pub struct KVMeta {
    pub raw: u64,
}

impl KVMeta {
    pub fn new(key_len: u16, val_len: u16, offset: u16) -> Self {
        // In a real impl, we would use bit shifting here to pack
        // 14 bits for lengths, 16 bits for offset, etc.
        // For clarity, we are simplifying slightly.
        let mut raw = 0u64;
        raw |= (key_len as u64) << 50;
        raw |= (val_len as u64) << 36;
        raw |= offset as u64;
        Self { raw }
    }

    pub fn get_offset(&self) -> usize {
        (self.raw & 0xFFFF) as usize
    }
}

// Paper: Node Meta is 12 bytes
// Node Size(16) | Type(1) | Split(1) | RecordCnt(16) | Leaf ID(48)
#[derive(Debug, Clone, Copy)]
pub struct NodeMeta {
    pub size: u16,
    pub is_mini_page: bool,
    pub record_count: u16,
    pub leaf_page_id: u64, // 48 bits in paper
}

// The generic container for both Mini and Leaf pages
pub struct SlottedPage {
    data: Vec<u8>, // Can be 64 bytes (Mini) or 4096 bytes (Leaf)
}

impl SlottedPage {
    pub fn new(capacity: usize, is_mini: bool) -> Self {
        Self {
            data: vec![0; capacity],
        }
    }

    // See[cite: 236]: Insert by shifting metadata, not entire node
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> bool {
        let meta_size = size_of::<NodeMeta>() + size_of::<KVMeta>() * (self.get_record_count() + 1);
        let data_size = self.get_used_data_size() + key.len() + value.len();

        // Check if we have space
        if meta_size + data_size > self.data.len() {
            return false; // Page is full!
        }

        // Write Data to the end (growing from right to left)
        let write_pos = self.data.len() - data_size;
        // ... (Write key and value bytes at write_pos) ...

        // Update Metadata (growing from left to right)
        // ... (Add KVMeta at the beginning) ...

        true
    }

    fn get_record_count(&self) -> usize {
        // parse bytes 2-4 of NodeMeta
        0 // Placeholder
    }

    fn get_used_data_size(&self) -> usize {
        0 // Placeholder
    }
}
