pub struct TripleIter<'a, T> {
    pub inner: &'a T,
    pub len: usize,
    pub count: usize,
}

impl<'a, T> Iterator for TripleIter<'a, T> 
    where T: std::ops::Index<usize>
{
    type Item = (Option<&'a T::Output>, Option<&'a T::Output>, Option<&'a T::Output>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.len { return None }

        let prev: Option<&'a T::Output>;
        let next: Option<&'a T::Output>;

        if self.count == 0 {
            prev = None;
        } else {
            prev = Some(&self.inner[self.count -1]);
        }

        let current = Some(&self.inner[self.count]);

        if self.count +1 < self.len {
            next = Some(&self.inner[self.count +1]);
        } else {
            next = None
        }
        self.count += 1;
        Some((prev, current, next))
    }
}