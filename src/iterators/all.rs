use Counter;
use Histogram;
use iterators::{HistogramIterator, PickyIterator, PickMetadata};

/// An iterator that will yield every bin.
pub struct Iter { visited: Option<usize> }

impl Iter {
    /// Construct a new full iterator. See `Histogram::iter_all` for details.
    pub fn new<'a, T: Counter>(hist: &'a Histogram<T>) -> HistogramIterator<'a, T, Iter> {
        HistogramIterator::new(hist, Iter{ visited: None })
    }
}

impl<T: Counter> PickyIterator<T> for Iter {
    fn pick(&mut self, index: usize, _: u64) -> Option<PickMetadata> {
        if self.visited.map(|i| i != index).unwrap_or(true) {
            // haven't visited this index yet
            self.visited = Some(index);
            Some(PickMetadata::new(None, None))
        } else {
            None
        }
    }

    fn more(&mut self, _: usize) -> bool {
        true
    }
}
