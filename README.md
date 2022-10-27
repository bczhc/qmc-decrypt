QMC-decrypt
---
## Supported formats
- `qmcflac` to `flac`
- `qmc0` to `mp3`
- `mgg1` and `mflac0` with manually `ekey` passed to be used, to `ogg` and `flac`
  
## See also/references
- https://github.com/unlock-music/cli/issues/37
- https://github.com/jixunmoe/qmc2-rust
- https://github.com/unlock-music/unlock-music/discussions/278
- https://github.com/bczhc/qmc-decode

Thanks to the `qmc2-crypto` module from [jixunmoe](https://github.com/jixunmoe).

The qmcflac decryption algorithm is picked randomly by me,
namely from https://github.com/juhemusic/LRC/blob/master/worker/qmc-worker.ts
