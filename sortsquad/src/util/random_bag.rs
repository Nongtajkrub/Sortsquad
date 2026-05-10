use rand::seq::SliceRandom;

pub struct RandomBag<T> {
    items: Vec<T>,
    size: usize,
}

impl<T> RandomBag<T> {
    pub fn new(mut items: Vec<T>) -> Self {
        items.shuffle(&mut rand::rng());

        Self { 
            size: items.len(),
            items,
        }
    }

    /// Returns the next random item, or None if the bag is empty
    pub fn next(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Check if we have more items
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
