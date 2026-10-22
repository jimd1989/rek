use sndio::Par;
use crate::models::audio::offset::Offset;
use crate::models::audio::play_track::PlayTrack;
use crate::models::parse::rek_session::RekSession;

pub struct PlayTracks<'a> {
  mix_buf: Vec<f64>,
  file_buf: Vec<u8>,
  pub out_buf: Vec<u8>,
  tracks: Vec<PlayTrack<'a>>,
}

impl<'a> PlayTracks<'a> {
  pub fn new(p: Par, r: &'a mut RekSession) -> PlayTracks {
    let tracks = &mut r.tracks;
    let mix_len = (p.appbufsz * p.pchan) as usize;
    let out_len = mix_len * (p.bps as usize);
    let mix_buf = vec![0.0; mix_len];
    let out_buf = vec![0; out_len];
    let chan_len = (p.appbufsz * p.bps) as usize;
    let file_buf = vec![0; chan_len];
    let offset = Offset::from_seconds(p, r.options.io.playback.offset);
    let volume = &r.options.io.playback.volume;
    let plays = tracks.iter_mut()
      .map(move |t| PlayTrack::new(t, p, offset.clone(), volume)).collect();
    Self {
      mix_buf: mix_buf,
      file_buf: file_buf,
      out_buf: out_buf,
      tracks: plays,
    }
  }
  pub fn mix(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.mix_buf.fill(0.0);
    for t in &mut self.tracks {
      t.mix(&mut self.mix_buf, &mut self.file_buf)?;
    }
    for (f, bs) in self.mix_buf.iter().zip(self.out_buf.chunks_exact_mut(2)) {
      let s16 = (f.clamp(-1.0, 1.0) * 32768.0) as i16;
      let bytes = s16.to_le_bytes();
      bs[0] = bytes[0];
      bs[1] = bytes[1];
    }
    Ok(())
  }
}
