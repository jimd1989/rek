use lexpr::Value;

pub trait SexpDecoder: Sized {
  fn decode(v: &Value) -> Result<Self, String>;

  fn decode_at<A: SexpDecoder>(v: &Value, key: &str) -> Result<A, String> {
    let ω = v.get(key).ok_or_else(|| format!("Missing field {}", key))?;
    let α = ω.as_cons().ok_or_else(|| format!("Not list {}", ω))?;
    let h = α.list_iter().nth(0).ok_or_else(|| format!("Empty {}", key))?;
    A::decode(h).map_err(|e| format!("Error at {}: {}", key, e))
  }

  fn decode_option<A: SexpDecoder>(v: &Value, key: &str)
    -> Result<Option<A>, String> {
      match v.get(key) {
        None    => Ok(None),
        Some(_) => A::decode_at(v, key).map(Some)
      }
  }
}

impl SexpDecoder for i64 {
  fn decode(v: &Value) -> Result<Self, String> {
    v.as_i64().ok_or_else(|| format!("Expected i64, got {:?}", v))
  }
}

impl SexpDecoder for f64 {
  fn decode(v: &Value) -> Result<Self, String> {
    v.as_f64().ok_or_else(|| format!("Expected f64, got {:?}", v))
  }
}

impl SexpDecoder for String {
  fn decode(v: &Value) -> Result<Self, String> {
    v.as_str()
      .map(|ω| ω.to_string())
      .ok_or_else(|| format!("Expected string, got {:?}", v))
  }
}

impl SexpDecoder for bool {
  fn decode(v: &Value) -> Result<Self, String> {
    v.as_bool().ok_or_else(|| format!("Expected bool, got {:?}", v))
  }
}

impl<A: SexpDecoder> SexpDecoder for Vec<A> {
  fn decode(v: &Value) -> Result<Self, String> {
    let ω = v.as_cons().ok_or_else(|| format!("Expected list, got {:?}", v))?;
    ω.list_iter()
      .enumerate()
      .map(|(n, α)| A::decode(α).map_err(|e| format!("Error at {}: {}", n, e)))
      .collect()
  }
}
