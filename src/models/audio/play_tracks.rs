use sndio::Par;
use crate::models::audio::play_track::PlayTrack;
use crate::models::parse::rek_session::RekSession;

pub struct PlayTracks<'a> {
  mix_buf: Vec<f64>,
  pub out_buf: Vec<u8>,
  tracks: Vec<PlayTrack<'a>>,
}

impl<'a> PlayTracks<'a> {
  pub fn new(p: Par, r: &'a RekSession) -> PlayTracks {
    let tracks = &r.tracks;
    let mix_len = (p.appbufsz * p.pchan) as usize;
    let out_len = mix_len * (p.bps as usize);
    let mix_buf = vec![0.0; mix_len];
    let out_buf = vec![0; out_len];
    let plays = tracks.iter().map(move |t| PlayTrack::new(t, p)).collect();
    Self {
      mix_buf: mix_buf,
      out_buf: out_buf,
      tracks: plays,
    }
  }
}

/* need impl */
/* mix to tanh or "soft slip curve" */
