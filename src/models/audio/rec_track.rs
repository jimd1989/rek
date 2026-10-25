use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use lexpr::Value;
use sndio::Par;
use crate::traits::sexp_encoder::SexpEncoder;

pub struct RecTrack {
  name: String,
  file: BufWriter<File>,
  volume: f64,
  ch: usize,
  rchan: usize,
}

impl RecTrack {
  pub fn new(p: Par, extant_tracks: usize, ch: usize, volume: f64) 
    -> Result<RecTrack, Box<dyn Error>> {
      let name = format!("tracks/{0:8}.raw", extant_tracks + ch);
      let file = BufWriter::new(File::create(&name)?);
      let rchan = p.rchan as usize;
      Ok(RecTrack { name, file, volume, ch, rchan})
  }
}

impl SexpEncoder<RecTrack> for RecTrack {
  fn encode(&self) -> Value {
    let mut vols = vec![0.0; self.rchan];
    vols[self.ch] = 1.0;
    Value::list(vec![
      Self::kv("tag", Value::symbol("your-tag-here")),
      Self::kv("volume", Value::list(vols)),
      Self::kv("file", Value::string(self.name.clone())),
    ])
  }
}
