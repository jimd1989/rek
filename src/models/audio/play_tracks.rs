use sndio::Par;
use crate::models::audio::play_track::PlayTrack;
use crate::models::parse::rek_session::RekSession;

/*
pub struct PlayTrack<'a> {
  pub file: BufReader<File>,
  pub volume: Vec<f64>,
  pub buf: &'a Vec<f64>,
  pchan: u32,
  div: f64,
}
*/

pub struct PlayTracks<'a> {
  tracks: Vec<PlayTrack<'a>>,
}

/* need impl */
