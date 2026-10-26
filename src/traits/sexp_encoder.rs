use lexpr::Value;

pub trait SexpEncoder{
  fn kv<A: Into<Value>>(k: &str, v: A) -> Value {
    Value::list(vec![Value::symbol(k), v.into()])
  }
  fn encode(&self) -> Value;
}
