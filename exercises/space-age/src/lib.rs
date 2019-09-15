// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    duration: u64,
}

impl From<u64> for Duration {
    fn from(duration: u64) -> Self {
        Duration { duration }
    }
}

pub trait Planet {
    const ORBITAL: f64;
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / Self::ORBITAL
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL: f64 = 31_557_600f64 * 0.240_846_7;
}
impl Planet for Venus {
    const ORBITAL: f64 = 31_557_600f64 * 0.615_197_26;
}
impl Planet for Earth {
    const ORBITAL: f64 = 31_557_600f64 * 1.0;
}
impl Planet for Mars {
    const ORBITAL: f64 = 31_557_600f64 * 1.880_815_8;
}
impl Planet for Jupiter {
    const ORBITAL: f64 = 31_557_600f64 * 11.862_615;
}
impl Planet for Saturn {
    const ORBITAL: f64 = 31_557_600f64 * 29.447_498;
}
impl Planet for Uranus {
    const ORBITAL: f64 = 31_557_600f64 * 84.016_846;
}
impl Planet for Neptune {
    const ORBITAL: f64 = 31_557_600f64 * 164.791_320;
}
