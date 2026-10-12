use lexpr::Value;
use crate::models::parse::device_options::DeviceOptions;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RekOptions {
  pub io: DeviceOptions,
}

impl SexpDecoder for RekOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    let ios = Self::decode_at::<Vec<DeviceOptions>>(v, "io")?;
    let io = ios.first().ok_or_else(|| format!("No i/o specified"))?;
    Ok(RekOptions { io: io.clone() })
  }
}
