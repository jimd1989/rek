use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use sndio::Par;
use crate::models::parse::track::Track;

pub struct PlayTrack<'a> {
  pub file: &'a mut BufReader<File>,
  pub volume: &'a Vec<f64>,
  appbufsz: usize,
  bps: usize,
  pchan: usize,
}

impl<'a> PlayTrack<'a> {
  pub fn new(t: &'a mut Track, p: Par) -> PlayTrack<'a> {
    Self {
      file: &mut t.file,
      volume: &t.volume,
      appbufsz: p.appbufsz as usize,
      bps: p.bps as usize,
      pchan: p.pchan as usize,
    }
  }
  pub fn mix(&mut self, buf: &mut Vec<f64>)
    -> Result<(), Box<dyn std::error::Error>> {
    let ρ = self.appbufsz * self.bps;
    let mut file_buf = vec![0u8; ρ];
    let bytes_read = self.file.read(&mut file_buf)?;
    for (frame_n, bytes) in file_buf.chunks_exact(2).enumerate() {
      let s16 = i16::from_le_bytes([bytes[0], bytes[1]]);
      let sf = (s16 as f64) / 32768.0;
      let n = frame_n * self.pchan;
      for ch in 0 .. self.pchan {
        buf[n + ch] += sf * self.volume[ch];
      }
    }
    return Ok(());
  }
}
