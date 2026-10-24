use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

pub struct RecTrack {
  pub name: String,
  file: BufWriter<File>,
  volume: f64,
}

impl RecTrack {
  pub fn new(n: usize, volume: f64) -> Result<RecTrack, Box<dyn Error>> {
      let name = format!("tracks/{0:8}.raw", n);
      let file = BufWriter::new(File::create(&name)?);
      Ok(RecTrack { name, file, volume })
  }
}
