
// direct copy of Chunks, was trying for windows, but then found windows. 
#[derive(Debug, Clone)]
pub struct Neighbors<'a, T: 'a>{
    v: &'a [T],
    size: usize,
}

impl <'a, T: 'a> Neighbors<'a, T> {
    #[inline]
    pub(super) fn new(slice: &'a [T], size: usize) -> Self {
	Self { v: slice, size: size }
    }
}

impl <'a, T> Iterator for Neighbors<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
	if self.v.is_empty() {
	    None
	} else {
	    let chunksz = cmp::min(self.v.len(), self.size);
	    let (fst, snd) = self.v.split_at(chunksz);
	    self.v = snd;
	    Some(fst)
	}
    }
}
