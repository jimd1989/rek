use sndio::Par;
use crate::models::parse::device_options::DeviceOptions;

pub fn device_options_to_sio_par(o: &DeviceOptions) -> Par {
  let mut ω = Par::default();
  ω.bits = o.bits;
  ω.bps = o.bps;
  ω.rate = o.rate;
  ω.appbufsz = o.appbufsz;
  ω.rchan = o.recording.channels;
  return ω;
}
