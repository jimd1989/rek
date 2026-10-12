use lexpr::Value;
use crate::models::parse::device_options::DeviceOptions;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RekOptions {
  pub io: Vec<DeviceOptions>,
}

impl SexpDecoder for RekOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(RekOptions {
      io: Self::decode_at(v, "io")?,
    })
  }
}
