use std::str::FromStr;

pub enum Cmd {
  Init,
  Play,
  Rec,
}

impl FromStr for Cmd {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_uppercase().as_str() {
      "INIT" => Ok(Cmd::Init),
      "PLAY" => Ok(Cmd::Play),
      "REC" => Ok(Cmd::Rec),
      _ => Err(format!("bad cmd: {}", s)),
    }
  }
}
