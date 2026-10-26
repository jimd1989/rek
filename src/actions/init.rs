pub const INIT_STATE: &str = r#"
; run `rek rec` over this text to begin recording tracks
((options 
   ((io 
      (((device "default")
        (recording ((channels 2) (volume (1.0 1.0))))
        (playback ((monitor 1.0) (channels 2) (volume (0.5 0.5))))))))
   (export ()))
 (tracks ()))
"#;

pub fn init() {
  println!("{}", INIT_STATE);
}
