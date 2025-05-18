QMC-decrypt
---
## Supported formats
- `qmcflac` to `flac`
- `qmc0` to `mp3`
- `mgg1` and `mflac0` with manually `ekey` passed to be used, to `ogg` and `flac`
  
## Usage
```
Usage: qmc-decrypt <input> <output> [ekey]

Arguments:
  <input>   
  <output>  
  [ekey]    

Options:
  -h, --help  Print help information
```

## See also/references
- https://github.com/unlock-music/cli/issues/37 (archive: https://web.archive.org/web/20221227073117/https://git.unlock-music.dev/um/cli/issues/37)
- https://github.com/jixunmoe/qmc2-rust
- https://github.com/unlock-music/unlock-music/discussions/278
- https://github.com/bczhc/qmc-decode
- https://github.com/zeroclear/unlock-mflac-20220931/issues/1 (archive: https://web.archive.org/web/20221227073855/https://github.com/zeroclear/unlock-mflac-20220931/issues/1)

Thanks to the `qmc2-crypto` module from [jixunmoe](https://github.com/jixunmoe).

The qmcflac decryption algorithm is picked randomly by me,
namely from https://github.com/juhemusic/LRC/blob/master/worker/qmc-worker.ts

### 关于ekey获取
<img width="1149" alt="image" src="https://github.com/user-attachments/assets/89be9970-5e6d-4236-b605-0172c135a2ce" />
<img width="985" alt="image" src="https://github.com/user-attachments/assets/897a4d36-d3a1-469a-8dd0-59f4ddf5fb61" /> (https://gist.github.com/bczhc/ad58a291895f701839098e4b403a521a)
