#[allow(dead_code)]
pub trait Vec {
    fn normalize(&self);
    fn norm(&self); // same as |magnitude|
}

#[allow(unused)]
pub fn dot(v1: impl Vec, v2: impl Vec) {}

#[allow(unused)]
pub fn cross(v1: impl Vec, v2: impl Vec) {}
