use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::BufWriter;
use sndio::Par;
use crate::models::audio::rec_track::RecTrack;
use crate::models::parse::rek_session::RekSession;

pub struct RecTracks {
  pub in_buf: Vec<u8>,
  pub tracks: Vec<RecTrack>,
  mix: Vec<Vec<f64>>,
}

impl RecTracks {
  pub fn new(p: Par, r: &RekSession, extant_tracks: usize)
  -> Result<RecTracks, Box<dyn Error>> {
    let ch = p.rchan as usize;
    let buf_frames = p.appbufsz as usize;
    let bps = p.bps as usize;
    let in_buf_size = ch * buf_frames * bps;
    let in_buf = vec![0u8; in_buf_size]; 
    let volume = &r.options.io.recording.volume;
    let mix = vec![vec![0.0; buf_frames]; ch];
    create_dir_all("tracks")?;
    let tracks = volume.into_iter().enumerate()
      .map(|(n, v)| RecTrack::new(extant_tracks, n, *v))
      .collect::<Result<Vec<_>, _>>()?;
    Ok(RecTracks { in_buf, tracks, mix })
  }
}
