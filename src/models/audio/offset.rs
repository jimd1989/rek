use std::ops::Add;
use lexpr::Value;
use sndio::Par;
use crate::traits::sexp_encoder::SexpEncoder;

#[derive(Clone)]
pub struct Offset {
  pub delay_cycles: u32,
  pub fast_forward_bytes: u32,
  delay: u32,
  appbufsz: u32,
  bps: u32,
  rate: u32,
}

impl Offset {
  pub fn from_seconds(p: Par, s: f64) -> Self {
    let samples = (s * (p.rate as f64)) as i32;
    if (samples < 0) {
      let delay_cycles = ((samples * -1) as u32) / p.appbufsz;
      return Self { 
        delay_cycles: delay_cycles, 
        fast_forward_bytes: 0, 
        delay: 0,
        appbufsz: p.appbufsz,
        bps: p.bps,
        rate: p.rate,
      };
    }
    let ff = (samples as u32) * p.bps;
    Self { 
      delay_cycles: 0, 
      fast_forward_bytes: ff, 
      delay: 0,
      appbufsz: p.appbufsz,
      bps: p.bps,
      rate: p.rate,
    }
  }
  pub fn is_delaying(&self) -> bool {
    self.delay < self.delay_cycles
  }
  pub fn decrement_delay(&mut self) {
    self.delay += 1;
  }
}

impl Add for Offset {
  type Output = Self;
  fn add(self, ω: Self) -> Self::Output {
    Self {
      delay_cycles: self.delay_cycles + ω.delay_cycles,
      fast_forward_bytes: self.fast_forward_bytes + ω.fast_forward_bytes,
      delay: 0,
      appbufsz: self.appbufsz,
      bps: self.bps,
      rate: self.rate,
    }
  }
}

impl SexpEncoder for Offset {
  fn encode(&self) -> Value {
    if self.delay_cycles > 0 {
      let bytes = (self.appbufsz * self.delay_cycles) as f64;
      let seconds = bytes / (self.rate as f64);
      Self::kv("fast-forward", seconds)
    } else {
      let bytes = (self.fast_forward_bytes as f64) / self.bps as f64;
      let seconds = bytes / (self.rate as f64);
      Self::kv("fast-forward", -1.0 * seconds)
    }
  }
}
