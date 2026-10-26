# rek

rek is a multi-track audio recorder that operates as follows:

1. Reads s-expression representing project state from stdin.
2. Records/plays audio files until user hits ctrl+c.
3. Prints s-expression representing updated project state to stdout.

The user is expected to use a text editor like Vim or Emacs as the primary interface, highlighting state and piping it to `rek`.

## Requirements 

- [sndio](https://sndio.org/)

## Usage

```
rek <init | play | rec> <piped stdin>
```

- `init` prints a starting project state s-expression.
- `play` plays back already recorded tracks.
- `rec` records new tracks, n monophonic files for n channels, and adds them to project state.

The entire s-expression should be copied into stdin for each invocation.

## Project structure

```scheme
((options
   ((io (((device "default")
          (recording ((channels 2) (volume (1.0 1.0))))
          (playback
            ((fast-forward 6.0)
             (monitor 0.0)
             (channels 2)
             (volume (0.5 0.5))))))))
   (export ()))
 (tracks
   (((tag your-tag-here)
     (volume (1.0 0.0))
     (fast-forward -0.0)
     (file "tracks/00000000.raw"))
    ((tag your-tag-here)
     (volume (0.0 1.0))
     (fast-forward -0.0)
     (file "tracks/00000001.raw"))
    ((tag your-tag-here)
     (volume (1.0 0.0))
     (fast-forward -1.0)
     (file "tracks/00000002.raw"))
    ((tag your-tag-here)
     (volume (0.0 1.0))
     (fast-forward -1.0)
     (file "tracks/00000003.raw"))
    ((tag your-tag-here)
     (volume (1.0 0.0))
     (fast-forward -2.0)
     (file "tracks/00000004.raw"))
    ((tag your-tag-here)
     (volume (0.0 1.0))
     (fast-forward -2.0)
     (file "tracks/00000005.raw")))))
```

- options
  - io
    - device: sndio device name—usually "default"
    - recording
      - channels: number of input channels
      - volume: individual input volumes
    - playback
      - channels: number of output channels
      - volume: individual master output volumes
      - monitor: volume of existing track playback while recording new ones (0 to mute)
      - fast-forward: master playback position in seconds, where -n is a n second delay before playback, and +n seeks n seconds ahead in playback
  - export: will eventually contain WAV export options
- tracks
  - tag: arbitrary user name for a track
  - volume: individual channel playback volumes for a track; must match master playback channels
  - fast-forward: individual track position during playback, positive and negative; composes with master fast-forward
  - file: path to raw monophonic PCM data

## Intentions

- `rek` is only for OpenBSD. You can try installing sndio on your OS of choice and see if it works, but I don't care if it doesn't.
- It is hardcoded to 16bit/48Khz. I will likely change this to 24bit/96Khz once I acquire the right hardware. In anticipation of this switch, I have not bothered with dithering.
- This is meant for linear playback and recording and is utterly non-interactive. Once you run it, you cannot change any parameters until exiting with ^c.
- There is no DSP beyond applying volume. Users are expected to handle equalization on their hardware mixer at recording/playback time.
- This runs on s-expressions, not Scheme. There is no quasiquoting, inline code, or pretty printing. The user is expected to use his or her own editor to format the output. Of course, being an s-expression, it is easy to write external Scheme scripts to generate/manipulate `rek` files.
