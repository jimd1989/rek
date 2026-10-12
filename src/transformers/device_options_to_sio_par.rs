use sndio::Par;
use crate::models::parse::device_options::DeviceOptions;

pub fn device_options_to_sio_par(o: &DeviceOptions, p: &mut Par) {
  p.bits = o.bits;
  p.bps = o.bps;
  p.rate = o.rate;
  p.appbufsz = o.appbufsz;
  p.rchan = o.recording.channels;
}
