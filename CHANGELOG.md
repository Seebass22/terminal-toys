# v0.2.0

## all screensavers
- `ESC` and `CTRL-C` now quit the app (in addition to `q`)

## pipes3d
- Slowed down camera follow speed
- Added `--camera-speed` / `-x` option
- Lowered default segment limit to 2000 (was 20000)
- Added `--rotate` / `-r`, which removes segments from the start of the line
  instead of resetting when the segment limit is reached

## sand
- Fixed a bug where flipping would cause the sand to be emptied
