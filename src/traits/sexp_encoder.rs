use lexpr::Value;

pub trait SexpEncoder<A> {
  fn encode(a: &A) -> Result<Value, String>;
}
