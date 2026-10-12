use std::collections::HashSet;
use lexpr::Value;
use crate::models::parse::device_options::DeviceOptions;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RekOptions {
  pub io: Vec<DeviceOptions>,
}

impl SexpDecoder for RekOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    let ios = Self::decode_at::<Vec<DeviceOptions>>(v, "io")?;
    let names: HashSet<Option<String>> = 
      ios.clone().into_iter().map(|ω| ω.device).collect();
    if (names.len() != ios.len()) {
      return Err("duplicate sndio device names detected".to_string());
    }
    Ok(RekOptions { io: ios })
  }
}
