use std::fs::File;
use std::io::BufReader;
use lexpr::Value;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct Track {
  pub file: BufReader<File>,
  pub volume: Vec<f64>,
}

impl SexpDecoder for Track {
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(Track {
      file: Self::decode_at(v, "file")?,
      volume: Self::decode_at(v, "volume")?,
    })
  }
}
