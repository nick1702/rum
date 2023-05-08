use std::sync::atomic::{AtomicU32, Ordering};

/// Manages memory segments, allocating and deallocating segment IDs.
/// Uses a Vec to store segments and a Vec to store unmapped segment IDs.
#[derive(Debug)]
pub struct SegmentManager {
    pub segments: Vec<Vec<u32>>,
    unmapped_ids: Vec<u32>,
    next_id: AtomicU32,
}

impl SegmentManager {
    /// Creates a new SegmentManager instance.
    ///
    /// # Returns
    ///
    /// A new instance of SegmentManager.
    pub fn new() -> Self {
        SegmentManager {
            segments: Vec::new(),
            unmapped_ids: Vec::new(),
            next_id: AtomicU32::new(0),
        }
    }

    /// Allocates a new memory segment with the specified size and returns its ID.
    ///
    /// # Arguments
    ///
    /// * `size` - A usize representing the number of memory cells in the segment.
    ///
    /// # Returns
    ///
    /// A u32 integer representing the segment ID.
    pub fn allocate_segment(&mut self, size: usize) -> u32 {
        let id = if let Some(reused_id) = self.unmapped_ids.pop() {
            reused_id
        } else {
            let next_id = self.next_id.fetch_add(1, Ordering::SeqCst);
            if self.segments.capacity() == self.segments.len() {
                self.segments.reserve(self.segments.len()); // Double the capacity of segments
            }
            self.segments.push(Vec::new());
            next_id
        };

        let segment = &mut self.segments[id as usize];
        if segment.len() != size {
            segment.resize(size, 0);
        }

        id
    }

    /// Deallocates the memory segment with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A u32 integer representing the segment ID to deallocate.
    pub fn deallocate_segment(&mut self, id: u32) {
        let segment = &mut self.segments[id as usize];
        segment.clear();
        segment.shrink_to_fit();
        if self.unmapped_ids.capacity() == self.unmapped_ids.len() {
            self.unmapped_ids.reserve(self.unmapped_ids.len()); // Double the capacity of unmapped_ids
        }
        self.unmapped_ids.push(id);
    }

    /// Retrieves a mutable reference to the memory segment with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A u32 integer representing the segment ID.
    ///
    /// # Returns
    ///
    /// An Option containing a mutable reference to the `Vec<u32>` if the ID is valid,
    /// or None if the ID is not found.
    pub fn get_segment_mut(&mut self, id: u32) -> Option<&mut Vec<u32>> {
        self.segments.get_mut(id as usize)
    }
}
