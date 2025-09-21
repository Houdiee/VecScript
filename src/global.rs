#[allow(dead_code)]
pub fn sin(x: f64, deg: bool) -> f64 {
    let angle = if deg { x.to_radians() } else { x };
    angle.sin()
}

#[allow(dead_code)]
pub fn cos(x: f64, deg: bool) -> f64 {
    let angle = if deg { x.to_radians() } else { x };
    angle.cos()
}

#[allow(dead_code)]
pub fn tan(x: f64, deg: bool) -> f64 {
    let angle = if deg { x.to_radians() } else { x };
    angle.tan()
}

pub trait Addable<T> {
    type Output;
    fn add(self, other: T) -> Self::Output;
}
