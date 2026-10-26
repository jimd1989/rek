use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::BufWriter;
use lexpr::Value;
use sndio::Par;
use crate::models::audio::rec_track::RecTrack;
use crate::models::parse::rek_session::RekSession;
use crate::traits::sexp_encoder::SexpEncoder;

pub struct RecTracks {
  pub in_buf: Vec<u8>,
  tracks: Vec<RecTrack>,
  mix: Vec<Vec<f64>>,
}

impl RecTracks {
  pub fn new(p: Par, r: &RekSession) -> Result<RecTracks, Box<dyn Error>> {
    let offset = r.options.io.playback.offset;
    let extant_tracks = r.tracks.len();
    let ch = p.rchan as usize;
    let buf_frames = p.appbufsz as usize;
    let bps = p.bps as usize;
    let in_buf_size = ch * buf_frames * bps;
    let in_buf = vec![0u8; in_buf_size]; 
    let volume = &r.options.io.recording.volume;
    let mix = vec![vec![0.0; buf_frames]; ch];
    create_dir_all("tracks")?;
    let tracks = volume.into_iter().enumerate()
      .map(|(n, v)| RecTrack::new(p, extant_tracks, n, *v, offset))
      .collect::<Result<Vec<_>, _>>()?;
    Ok(RecTracks { in_buf, tracks, mix })
  }
  pub fn mix(&mut self) -> std::io::Result<()> {
    let rchan = self.mix.len();
    self.in_buf.chunks_exact(2).enumerate().for_each(|(n, bs)| {
      let s = i16::from_le_bytes([bs[0], bs[1]]);
      let f = (s as f64) / 32768.0;
      let ch = n % rchan;
      let frame = n / rchan;
      self.mix[ch][frame] = f;
    });
    self.tracks.iter_mut().zip(&self.mix).try_for_each(|(t, b)| t.write(b))?;
    Ok(())
  }
  pub fn flush(&mut self) -> std::io::Result<()> {
    self.tracks.iter_mut().try_for_each(|t| t.flush())
  }
}

impl SexpEncoder for RecTracks {
  fn encode(&self) -> Value {
    Value::list(self.tracks.iter().map(|ω| ω.encode()))
  }
}
