use lexpr::Value;
use crate::traits::sexp_decoder::SexpDecoder;

#[derive(Debug)]
pub struct RecordingOptions {
  pub device: Option<String>,
  pub channels: i64,
  pub volume: Option<Vec<f64>>,
}

impl SexpDecoder for RecordingOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(RecordingOptions {
      device: Self::decode_option(v, "device")?,
      channels: Self::decode_at(v, "channels")?,
      volume: Self::decode_option(v, "volume")?
    })
  }
}
