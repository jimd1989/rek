use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use lexpr::Value;
use crate::traits::sexp_encoder::SexpEncoder;

pub struct RecTrack {
  pub name: String,
  file: BufWriter<File>,
  volume: f64,
  ch: usize,
}

impl RecTrack {
  pub fn new(extant_tracks: usize, ch: usize, volume: f64) 
    -> Result<RecTrack, Box<dyn Error>> {
      let name = format!("tracks/{0:8}.raw", extant_tracks + ch);
      let file = BufWriter::new(File::create(&name)?);
      Ok(RecTrack { name, file, volume, ch})
  }
}

impl SexpEncoder<RecTrack> for RecTrack {
  fn encode(r: &RecTrack) -> Result<Value, String> {
    /* do actual values here */
    let x = "";
    lexpr::from_str(x).map_err(|e| format!("{}", e))
  }
}
