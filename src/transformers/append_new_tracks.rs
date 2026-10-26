use lexpr::{Cons, Value};

fn merge(mut α: Value, mut ω: Value) -> Value {
  let mut merged = Vec::new();
  while let Value::Cons(p) = α {
    let (car, cdr) = p.into_pair();
    merged.push(car);
    α = cdr;
  }
  while let Value::Cons(p) = ω {
    let (car, cdr) = p.into_pair();
    merged.push(car);
    ω = cdr;
  }
  Value::list(merged)
}

pub fn append_new_tracks(v: Value, new: Value) -> Value {
  match v {
    Value::Cons(p) => {
      let (car, cdr) = p.into_pair();
      if let Value::Symbol(ref s) = car {
        if **s == *"tracks" {
          if let Value::Cons(pp) = cdr {
            let (tracks, nil) = pp.into_pair();
            let merged_tracks = merge(tracks, new);
            let rewrapped_cdr = Value::Cons(Cons::new(merged_tracks, nil));
            return Value::Cons(Cons::new(car, rewrapped_cdr));
          }
          let merged = merge(cdr, new);
          return Value::Cons(Cons::new(car, merged));
        }
      }
      let ncar = append_new_tracks(car, new.clone());
      let ncdr = append_new_tracks(cdr, new);
      Value::Cons(Cons::new(ncar, ncdr))
    }
    ω => ω,
  }
}
