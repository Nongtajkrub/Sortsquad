use rand::seq::SliceRandom;
use rand::seq::IndexedRandom;

pub struct RandomBag<T> 
where 
    T: Copy + PartialEq + Eq,
{
    items: Vec<T>,
}

impl<T> RandomBag<T>
where 
    T: Copy + PartialEq + Eq,
{
    pub fn new(mut items: Vec<T>) -> Self {
        items.shuffle(&mut rand::rng());

        Self { 
            items,
        }
    }

    /// Returns the next random item, or None if the bag is empty
    pub fn next(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Returns the next random item, while trying to ignore a certain
    /// item if possible. Return None if bag is empty.
    pub fn try_next_without(&mut self, exclude: T) -> Option<T> {
        for i in (0..self.items.len()).rev() {
            if self.items[i] != exclude {
                return Some(self.items.swap_remove(i));
            }
        }

        // The excluded item can't be ignored.
        self.items.pop()
    }

    /// Check if we have more items
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

// Expect a list with elements.
pub fn random_from_list<T>(list: &[T]) -> T
where 
    T: Clone
{
    assert!(list.len() > 0);
    list.choose(&mut rand::rng()).unwrap().clone()
}
