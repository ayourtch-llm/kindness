pub struct MappedFilter<I, F, P> {
    iter: I,
    map_fn: F,
    pred: P,
}

impl<I, T, U, F, P> Iterator for MappedFilter<I, F, P>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
    P: Fn(&U) -> bool,
{
    type Item = U;

    fn next(&mut self) -> Option<U> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(item) => {
                    let mapped = (self.map_fn)(item);
                    if (self.pred)(&mapped) {
                        return Some(mapped);
                    }
                }
            }
        }
    }
}

pub fn map_filter<I, T, U, F, P>(iter: I, map_fn: F, pred: P) -> MappedFilter<I, F, P>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
    P: Fn(&U) -> bool,
{
    MappedFilter { iter, map_fn, pred }
}
