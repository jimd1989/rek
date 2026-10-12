use lexpr::Value;
use crate::models::parse::recording_options::RecordingOptions;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct DeviceOptions {
  pub device: Option<String>,
  pub recording: RecordingOptions,
}

impl SexpDecoder for DeviceOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(DeviceOptions {
      device: Self::decode_option(v, "device")?,
      recording: Self::decode_at(v, "recording")?,
    })
  }
}
