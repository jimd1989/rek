use std::fs::File;
use std::io::BufReader;
use sndio::Par;
use crate::models::parse::track::Track;

pub struct PlayTrack<'a> {
  pub file: &'a BufReader<File>,
  pub volume: &'a Vec<f64>,
  pchan: u32,
}

impl<'a> PlayTrack<'a> {
  pub fn new(t: &'a Track, p: Par) -> PlayTrack<'a> {
    Self {
      file: &t.file,
      volume: &t.volume,
      pchan: p.pchan,
    }
  }
  /* need mix() method that takes u8 buffer as input */
}
