pub struct SequenceIterator {
    sequence: Box<dyn Sequence>,
    index: usize,
}

pub struct AnyValue {

}

impl Iterator for SequenceIterator {
    type Item = AnyValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.index.checked_add(1)?;
        self.sequence.get(self.index)
    }
}

pub trait Sequence where Self: FromIterator  {
    fn get(&self, index: usize) -> Option<AnyValue>;
    fn iterator(&self) -> SequenceIterator {
        vec![].iter().collect()
        SequenceIterator { sequence: Box::new(*self), index: 0 }
    }
}
