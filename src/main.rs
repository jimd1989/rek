use lexpr::{sexp, Value};

mod models {
  pub mod recording_options;
}

mod traits {
  pub mod sexp_decoder;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = r#"((a 1)(b 2))"#;
  let v: Value = lexpr::from_str(input)?;
  let updated = sexp!((,v (c 3 (d 4))));
  let output = updated.to_string();
  let d: &Value = &updated["c"]["e"][0];
  println!("{}", d);
  Ok(())
}
