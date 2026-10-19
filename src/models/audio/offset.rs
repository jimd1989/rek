use std::ops::Add;
use sndio::Par;

pub struct Offset {
  pub delay_cycles: u32,
  pub fast_forward_bytes: u32,
}

impl Offset {
  pub fn from_seconds(p: Par, s: f64) -> Self {
    let samples = (s * (p.rate as f64)) as i32;
    if (samples < 0) {
      let delay_cycles = ((samples * -1) as u32) / p.appbufsz;
      return Self { delay_cycles: delay_cycles, fast_forward_bytes: 0 };
    }
    return Self { delay_cycles: 0, fast_forward_bytes: samples as u32 };
  }
  pub fn is_delaying(self) -> bool {
    return (self.delay_cycles > 0);
  }
  pub fn decrement_delay(&mut self) {
    self.delay_cycles = self.delay_cycles - 1;
  }
}

impl Add for Offset {
  type Output = Self;
  fn add(self, ω: Self) -> Self::Output {
    Self {
      delay_cycles: self.delay_cycles + ω.delay_cycles,
      fast_forward_bytes: self.fast_forward_bytes + ω.fast_forward_bytes,
    }
  }
}
