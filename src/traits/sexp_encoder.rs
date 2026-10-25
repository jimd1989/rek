use lexpr::Value;

pub trait SexpEncoder<A> {
  fn kv<B: Into<Value>>(k: &str, v: B) -> Value {
    Value::list(vec![Value::symbol(k), v.into()])
  }
  fn encode(&self) -> Value;
}
