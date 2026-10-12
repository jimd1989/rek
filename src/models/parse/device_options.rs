use lexpr::Value;
use crate::models::parse::recording_options::RecordingOptions;
use crate::traits::sexp_decoder::SexpDecoder;

#[derive(Clone)]
pub struct DeviceOptions {
  pub device: Option<String>,
  pub recording: RecordingOptions,
  pub bits: u32,
  pub bps: u32,
  pub rate: u32,
  pub appbufsz: u32,
}


impl Default for DeviceOptions {
  fn default() -> Self {
    Self {
      device: None,
      recording: RecordingOptions::default(),
      bits: 16,
      bps: 2,
      rate: 48000,
      appbufsz: 1,
    }
  }
}

impl SexpDecoder for DeviceOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    let ω = DeviceOptions::default();
    Ok(DeviceOptions {
      device: Self::decode_option(v, "device")?,
      recording: Self::decode_at(v, "recording")?,
      bits: ω.bits,
      bps: ω.bps,
      rate: ω.rate,
      appbufsz: ω.appbufsz
    })
  }
}
