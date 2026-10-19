use std::fs::File;
use std::io::BufReader;
use lexpr::Value;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct Track {
  pub file: BufReader<File>,
  pub volume: Vec<f64>,
  pub offset: f64,
}

impl SexpDecoder for Track {
  fn decode(v: &Value) -> Result<Self, String> {
    let off = Self::decode_option::<f64>(v, "fast-forward")?;
    let offset = off.unwrap_or(0.0);
    Ok(Track {
      file: Self::decode_at(v, "file")?,
      volume: Self::decode_at(v, "volume")?,
      offset: offset,
    })
  }
}
