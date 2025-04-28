# v0.4.0

## new screensaver: ant
Langton's ant (https://en.wikipedia.org/wiki/Langton%27s_ant).

The `--pattern <N>` option sets the starting pattern.
These patterns can be modified with the `--pattern-len <N>` option.

The `--filled`, `--dist-by-color` and `--n-colors` options provide more variation.

## tunnel
Added `--twist` option: makes the tunnel twist.

# v0.3.0

## new screensaver: tunnel
A rotating and moving tunnel.

The `--depth <DEPTH>` option controls the 3d effect.

`--depth 0` = no 3d

`--depth 1` = slight 3d effect

`--depth 2` = proper 3d effect

The default is 1 as it works better on smaller terminals than 2.

The number of colors and segments can be set with the `-n <N>` option and speed with `-x <SPEED>`.

## sand
* Changed scaling for `Block`, `Bar` or `Dot` characters so each character represents 2 pixels

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
