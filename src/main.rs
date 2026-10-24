#![allow(warnings)]
use std::error::Error;
use std::io;
use lexpr::print::Options;
use crate::actions::play::play;
use crate::models::parse::rek_session::RekSession;
use crate::traits::sexp_decoder::SexpDecoder;

mod actions {
  pub mod play;
}

mod models {
  pub mod audio {
    pub mod offset;
    pub mod play_track;
    pub mod play_tracks;
    pub mod rec_tracks;
  }
  pub mod parse {
    pub mod device_options;
    pub mod playback_options;
    pub mod recording_options;
    pub mod rek_options;
    pub mod rek_session;
    pub mod track;
  }
}

mod traits {
  pub mod sexp_decoder;
}

mod transformers {
  pub mod device_options_to_sio_par;
}

fn main() -> Result<(), Box<dyn Error>> {
  let i = io::stdin().lock();
  let v = lexpr::from_reader(i)?;
  let mut r = RekSession::decode(&v)?;
  play(&mut r);
  lexpr::print::to_writer_custom(io::stdout().lock(), &v, Options::default())?;
  Ok(())
}
