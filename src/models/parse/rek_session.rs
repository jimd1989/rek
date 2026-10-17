use lexpr::Value;
use crate::models::parse::rek_options::RekOptions;
use crate::models::parse::track::Track;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RekSession {
  pub options: RekOptions,
  pub tracks: Vec<Track>,
}

impl SexpDecoder for RekSession {
  fn decode(v: &Value) -> Result<Self, String> {
    let options = Self::decode_at::<RekOptions>(v, "options")?;
    let tracks = Self::decode_at::<Vec<Track>>(v, "tracks")?;
    let ρ = options.io.playback.volume.len();
    for t in &tracks {
      let vol = t.volume.len();
      if (vol != ρ) {
        return Err(format!("Expected {} track volumes, got {}", ρ, vol));
      }
    }
    Ok(RekSession { options, tracks })
  }
}
