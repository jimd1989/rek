#![allow(warnings)]
use std::env::args;
use std::error::Error;
use std::io;
use std::str::FromStr;
use lexpr::print::Options;
use crate::actions::play::play;
use crate::models::parse::cmd::Cmd;
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
    pub mod rec_track;
    pub mod rec_tracks;
  }
  pub mod parse {
    pub mod cmd;
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
  pub mod sexp_encoder;
}

mod transformers {
  pub mod device_options_to_sio_par;
}

fn main() -> Result<(), Box<dyn Error>> {
  let cmd = args().nth(1)
    .ok_or_else(|| "needs REC | PLAY | INIT".to_string())
    .and_then(|ω| Cmd::from_str(&ω))?;
  let i = io::stdin().lock();
  let v = lexpr::from_reader(i)?;
  let mut r = RekSession::decode(&v)?;
  match cmd {
    Cmd::Play => { play(&mut r); }
    Cmd::Rec  => { play(&mut r); }
    Cmd::Init => { play(&mut r); }
  }
  lexpr::print::to_writer_custom(io::stdout().lock(), &v, Options::default())?;
  Ok(())
}
