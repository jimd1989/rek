use lexpr::Value;
use crate::models::parse::recording_options::RecordingOptions;
use crate::traits::sexp_decoder::SexpDecoder;

#[derive(Debug)]
pub struct RekOptions {
  pub recording: Vec<RecordingOptions>,
}

impl SexpDecoder for RekOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(RekOptions {
      recording: Self::decode_at(v, "recording")?,
    })
  }
}
