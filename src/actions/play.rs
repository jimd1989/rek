use std::ffi::CString;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use sndio::{Mode, Sndio};
use crate::models::parse::rek_session::RekSession;
use crate::transformers::device_options_to_sio_par::device_options_to_sio_par;

pub fn play(r: &RekSession) -> Result<(), Box<dyn std::error::Error>> {
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
  /* Check pchan eventually */
  let mut buf = vec![0u8; p.appbufsz as usize];
  if !(sio.start()) {
    return Err("Error starting sndio".into());
  }
  while (running.load(Ordering::SeqCst)) {
    /* Actually use audio here */
    sio.write(&buf);
  }
  if !(sio.stop()) {
    return Err("Error stopping sndio".into());
  }
  Ok(())
}
