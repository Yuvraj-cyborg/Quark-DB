pub struct CircularBuffer {
    memory: Vec<u8>, // The huge chunk of RAM (e.g., 2GB)
    head: usize,     // Eviction pointer
    tail: usize,     // Allocation pointer
}

impl CircularBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size],
            head: 0,
            tail: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        // Check if we have space between tail and head
        // If full, trigger eviction of Head [cite: 324]

        let ptr = self.tail;
        self.tail += size;
        Some(ptr)
    }
}
