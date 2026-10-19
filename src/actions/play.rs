use std::ffi::CString;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use sndio::{Mode, Sndio};
use crate::models::audio::play_tracks::PlayTracks;
use crate::models::parse::rek_session::RekSession;
use crate::transformers::device_options_to_sio_par::device_options_to_sio_par;

pub fn play(r: &mut RekSession) -> Result<(), Box<dyn std::error::Error>> {
  let running = Arc::new(AtomicBool::new(true));
  let running_clone = Arc::clone(&running);
  ctrlc::set_handler(move || { running_clone.store(false, Ordering::SeqCst) });
  let mut p = Sndio::init_par();
  let o = &r.options.io;
  let dev = o.device.as_deref().map(CString::new).transpose()?;
  let mut sio = Sndio::open(dev.as_deref(), Mode::PLAY, false)
    .ok_or_else(|| "Error starting sndio")?;
  device_options_to_sio_par(&r.options.io, &mut p);
  if !sio.set_par(&mut p) {
    return Err("Error setting sndio parameters".into());
  }
  if !sio.get_par(&mut p) {
    return Err("Error getting sndio parameters".into());
  }
  if (o.bits != p.bits) {
    return Err(format!("Expected {} bits; got {}", o.bits, p.bits).into());
  }
  if (o.rate != p.rate) {
    return Err(format!("Expected {} Hz; got {}", o.rate, p.rate).into());
  }
  if (o.playback.channels != p.pchan) {
    return Err(format!("Expected {} chans; got {}",
        o.playback.channels, p.pchan).into());
  }
  if !(sio.start()) {
    return Err("Error starting sndio".into());
  }
  let mut pt = PlayTracks::new(p, r);
  while (running.load(Ordering::SeqCst)) {
    pt.mix()?;
    sio.write(&pt.out_buf);
  }
  if !(sio.stop()) {
    return Err("Error stopping sndio".into());
  }
  Ok(())
}
