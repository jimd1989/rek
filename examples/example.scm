((options 
   ((io 
      (((device "default")
        (recording ((channels 2) (volume (1.0 1.0))))
        (playback ()))
       ; For drift reasons, only the first device is opened, but it's good to
       ; specify all inputs/outputs. Just move the target device to the top of
       ; this list.
       ((device "aux")
        (recording ((channels 4) (volume (1.0 1.0 1.0 1.0))))
        (playback ()))))
    )
   (export ()))
 (tracks ()))
