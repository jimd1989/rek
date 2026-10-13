use lexpr::Value;
use crate::models::parse::rek_options::RekOptions;
use crate::models::parse::track::Track;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RekSession {
  pub options: RekOptions,
  pub tracks: Vec<Track>,
}

impl SexpDecoder for RekSession {
  /* Enforce channel consistency for tracks */
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(RekSession {
      options: Self::decode_at(v, "options")?,
      tracks: Self::decode_at(v, "tracks")?,
    })
  }
}
