use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Read;
use sndio::Par;
use crate::models::audio::offset::Offset;
use crate::models::parse::track::Track;

pub struct PlayTrack<'a> {
  file: &'a mut BufReader<File>,
  volume: &'a Vec<f64>,
  offset: Offset,
}

impl<'a> PlayTrack<'a> {
  pub fn new(t: &'a mut Track, p: Par, o: Offset, v: &Vec<f64>) 
    -> PlayTrack<'a> {
    let local_offset = Offset::from_seconds(p, t.offset);
    let offset = o + local_offset;
    t.file.seek_relative(offset.fast_forward_bytes as i64);
    t.volume.iter_mut().zip(v).for_each(|(α, ω)| *α *= ω);
    Self {
      file: &mut t.file,
      volume: &t.volume,
      offset: offset,
    }
  }
  pub fn mix(&mut self, mix_buf: &mut [f64], file_buf: &mut [u8])
    -> std::io::Result<()> {
    if (self.offset.is_delaying()) {
      self.offset.decrement_delay();
      return Ok(());
    }
    let pchan = self.volume.len();
    file_buf.fill(0);
    match self.file.read_exact(file_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {} /* no problem */
        Err(e) => return Err(e), /* actual IO error */
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
