use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::BufWriter;
use sndio::Par;
use crate::models::parse::rek_session::RekSession;

pub struct RecTracks<'a> {
  pub in_buf: Vec<u8>,
  files: Vec<BufWriter<File>>,
  volume: &'a Vec<f64>,
}

impl<'a> RecTracks<'a> {
  pub fn new(p: Par, r: &'a RekSession, extant_tracks: usize)
  -> Result<RecTracks, Box<dyn Error>> {
    let ch = p.rchan as usize;
    let mut files = Vec::with_capacity(ch);
    create_dir_all("tracks")?;
    for n in 0..ch {
      let name = format!("tracks/{}.raw", extant_tracks + n);
      let file = File::create(name)?;
      files.push(BufWriter::new(file));
    }
    let buf_size = ch * (p.bps as usize) * (p.appbufsz as usize);
    let in_buf = vec![0u8; buf_size]; 
    let volume = &r.options.io.recording.volume;
    Ok(RecTracks { in_buf, files, volume })
  }
}
