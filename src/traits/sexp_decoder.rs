use lexpr::Value;

pub trait SexpDecoder: Sized {
  fn decode(v: &Value) -> Result<Self, String>;
    fn decode_string(v: &Value, key: &str) -> Result<String, String> {
        v.get(key)
            .and_then(|val| val.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Missing or invalid string field '{}'", key))
    }
    fn decode_i64(v: &Value, key: &str) -> Result<i64, String> {
        v.get(key)
            .and_then(|val| val.as_i64())
            .ok_or_else(|| format!("Missing or invalid i64 field '{}'", key))
    }
}
