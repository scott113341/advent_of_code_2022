pub struct ConstantIterator<T>
where
    T: Clone,
{
    constant: T,
}

impl<T> ConstantIterator<T>
where
    T: Clone,
{
    pub fn new(constant: T) -> ConstantIterator<T> {
        ConstantIterator { constant }
    }
}

impl<T> Iterator for ConstantIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.constant.clone())
    }
}
