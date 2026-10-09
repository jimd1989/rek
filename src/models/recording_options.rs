use lexpr::Value;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RecordingOptions {
  pub channels: i64,
}

impl SexpDecoder for RecordingOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    let channels = Self::decode_i64(v, "channels")?;
    Ok(RecordingOptions { channels } )
  }
}
