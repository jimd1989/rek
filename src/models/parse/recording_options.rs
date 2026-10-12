use lexpr::Value;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RecordingOptions {
  pub channels: i64,
  pub volume: Vec<f64>,
}

impl Default for RecordingOptions {
  fn default() -> Self {
    Self {
      channels: 1,
      volume: vec![1.0],
    }
  }
}

impl SexpDecoder for RecordingOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    let ω = RecordingOptions::default();
    let ch = Self::decode_option::<i64>(v, "channels")?;
    let vo = Self::decode_option::<Vec<f64>>(v, "volume")?;
    let channels = ch.unwrap_or(ω.channels);
    let volume = match (channels, vo) {
      (n, Some(vo)) if vo.len() == n as usize => Ok(vo),
      (n, None) => Ok(vec![1.0; n as usize]),
      (n, (Some(vo))) => Err(format!("{} channels; {} volume", n, vo.len())),
    }?;
    Ok(RecordingOptions { channels, volume })
  }
}
