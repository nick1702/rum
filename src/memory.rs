use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashMap;

/// Represents a memory segment with a vector of u32 values.
#[derive(Debug)]
pub struct Segment {
    pub memory: Vec<u32>,
}

/// Manages memory segments, allocating and deallocating segment IDs.
/// Uses a HashMap to store segments and a Vec to store unmapped segment IDs.
#[derive(Debug)]
pub struct SegmentManager {
    pub segments: HashMap<u32, Segment>,
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
            segments: HashMap::new(),
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
            next_id
        };
        self.segments.insert(id, Segment { memory: vec![0; size] });
        id
    }

    /// Deallocates the memory segment with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A u32 integer representing the segment ID to deallocate.
    pub fn deallocate_segment(&mut self, id: u32) {
        if self.segments.remove(&id).is_some() {
            self.unmapped_ids.push(id);
        }
    }

    /// Retrieves a mutable reference to the memory segment with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A u32 integer representing the segment ID.
    ///
    /// # Returns
    ///
    /// An Option containing a mutable reference to the `Segment` if the ID is valid,
    /// or None if the ID is not found.
    pub fn get_segment_mut(&mut self, id: u32) -> Option<&mut Segment> {
        self.segments.get_mut(&id)
    }
}