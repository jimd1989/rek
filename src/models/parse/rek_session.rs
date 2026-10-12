use lexpr::Value;
use crate::models::parse::rek_options::RekOptions;
use crate::traits::sexp_decoder::SexpDecoder;

pub struct RekSession {
  options: RekOptions,
}

impl SexpDecoder for RekSession {
  fn decode(v: &Value) -> Result<Self, String> {
    Ok(RekSession {
      options: Self::decode_at(v, "options")?,
    })
  }
}
