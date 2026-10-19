use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Read;
use sndio::Par;
use crate::models::parse::track::Track;

pub struct PlayTrack<'a> {
  pub file: &'a mut BufReader<File>,
  pub volume: &'a Vec<f64>,
}

impl<'a> PlayTrack<'a> {
  pub fn new(t: &'a mut Track, p: Par) -> PlayTrack<'a> {
    Self {
      file: &mut t.file,
      volume: &t.volume,
    }
  }
  pub fn mix(&mut self, mix_buf: &mut Vec<f64>, file_buf: &mut Vec<u8>)
    -> Result<(), Box<dyn std::error::Error>> {
    let pchan = self.volume.len();
    file_buf.fill(0);
    match self.file.read_exact(file_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {} /* no problem */
        Err(e) => return Err(Box::new(e)), /* actual IO error */
    }
    for (frame_n, bytes) in file_buf.chunks_exact(2).enumerate() {
      let s16 = i16::from_le_bytes([bytes[0], bytes[1]]);
      let sf = (s16 as f64) / 32768.0;
      let n = frame_n * pchan;
      for ch in 0 .. pchan {
        mix_buf[n + ch] += sf * self.volume[ch];
      }
    }
    return Ok(());
  }
}
