use lexpr::{sexp, Value};
use crate::models::parse::recording_options::RecordingOptions;
use crate::traits::sexp_decoder::SexpDecoder;

mod models {
  pub mod parse {
    pub mod recording_options;
  }
}

mod traits {
  pub mod sexp_decoder;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = r#"(recording (channels 2) (volume 1.0 1.0))"#;
  let v: Value = lexpr::from_str(input)?;
  let ω = RecordingOptions::decode_at::<RecordingOptions>(&v, "recording");
  Ok(())
}
