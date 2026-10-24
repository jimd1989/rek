use lexpr::Value;
use crate::traits::sexp_decoder::SexpDecoder;

#[derive(Clone)]
pub struct PlaybackOptions {
  pub channels: u32,
  pub volume: Vec<f64>,
  pub monitor: f64,
  pub offset: f64,
}

impl Default for PlaybackOptions {
  fn default() -> Self {
    Self {
      channels: 1,
      volume: vec![],
      monitor: 0.0,
      offset: 0.0,
    }
  }
}

impl SexpDecoder for PlaybackOptions {
  fn decode(v: &Value) -> Result<Self, String> {
    let ω = PlaybackOptions::default();
    let ch = Self::decode_option::<u32>(v, "channels")?;
    let vo = Self::decode_option::<Vec<f64>>(v, "volume")?;
    let off = Self::decode_option::<f64>(v, "fast-forward")?;
    let mon = Self::decode_option::<f64>(v, "monitor")?;
    let channels = ch.unwrap_or(ω.channels);
    let volume = match (channels, vo) {
      (n, Some(vo)) if vo.len() == n as usize => Ok(vo),
      (n, None) => Ok(vec![1.0; n as usize]),
      (n, (Some(vo))) => Err(format!("{} channels; {} volume", n, vo.len())),
    }?;
    let offset = off.unwrap_or(0.0);
    let monitor = off.unwrap_or(0.0);
    Ok(PlaybackOptions { channels, volume, monitor, offset })
  }
}
