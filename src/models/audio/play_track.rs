use std::fs::File;
use std::io::BufReader;
use sndio::Par;
use crate::models::parse::track::Track;

pub struct PlayTrack<'a> {
  pub file: BufReader<File>,
  pub volume: Vec<f64>,
  pub buf: &'a Vec<f64>,
  pchan: u32,
  div: f64,
}

impl<'a> PlayTrack<'a> {
  pub fn new(t: Track, p: Par, buf: &'a Vec<f64>, div: f64) -> PlayTrack {
    let ρ = (p.appbufsz * p.pchan) as usize;
    Self {
      file: t.file,
      volume: t.volume,
      buf: buf,
      pchan: p.pchan,
      div: div
    }
  }
  /* need mix() method that takes u8 buffer as input */
}
