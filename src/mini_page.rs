use crate::layout::PAGE_SIZE;
use crate::layout::SlottedPage;

pub struct MiniPage {
    pub page: SlottedPage,
}

impl MiniPage {
    // Start small (64 bytes) to align with cache lines
    pub fn new_minimal() -> Self {
        Self {
            page: SlottedPage::new(64, true),
        }
    }

    pub fn insert_record(&mut self, key: &[u8], value: &[u8]) -> Result<(), &'static str> {
        // Try to insert into current page
        if self.page.insert(key, value) {
            return Ok(());
        }

        // If full, we need to handle "Resize" or "Merge"
        let current_size = self.page.data.len();

        if current_size >= PAGE_SIZE {
            return Err("Must Merge to Disk"); // [cite: 434]
        }

        // Grow: Double the size
        let new_size = current_size * 2;
        let mut new_page = SlottedPage::new(new_size, true);

        // Copy old data to new page (Logic omitted for brevity)

        self.page = new_page;

        // Retry insert
        if self.page.insert(key, value) {
            Ok(())
        } else {
            Err("Resize failed")
        }
    }
}
