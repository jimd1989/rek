use std::str::FromStr;

pub enum Cmd {
  Rec,
  Play,
  Init,
}

impl FromStr for Cmd {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_uppercase().as_str() {
      "REC" => Ok(Cmd::Rec),
      "PLAY" => Ok(Cmd::Play),
      "INIT" => Ok(Cmd::Init),
      _ => Err(format!("bad cmd: {}", s)),
    }
  }
}
