use lexpr::Value;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RecordingOptions {
  pub channels: Option<i64>,
  pub volume: Option<Vec<f64>>,
}

impl SexpDecoder for RecordingOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(RecordingOptions {
      channels: Self::decode_option(v, "channels")?,
      volume: Self::decode_option(v, "volume")?
    })
  }
}
