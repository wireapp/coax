use std::hash::{Hash, Hasher};
use fnv::FnvHasher;

macro_rules! with {
    ($($name:ident),+ => $f:expr) => {{
        $(let $name = $name.clone();)+
        $f
    }}
}

#[inline]
pub fn hash<T: Hash>(x: T) -> u64 {
    let mut h = FnvHasher::default();
    x.hash(&mut h);
    h.finish()
}
