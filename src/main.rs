use std::fs::File;
use std::io::BufReader;
use crate::models::parse::rek_session::RekSession;
use crate::traits::sexp_decoder::SexpDecoder;

mod models {
  pub mod parse {
    pub mod recording_options;
    pub mod rek_options;
    pub mod rek_session;
  }
}

mod traits {
  pub mod sexp_decoder;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let f = File::open("./examples/example.scm")?;
  let r = BufReader::new(f);
  let v = lexpr::from_reader(r)?;
  let ω = RekSession::decode(&v)?;
  println!("{:?}", ω);
  Ok(())
}
