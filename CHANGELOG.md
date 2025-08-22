# v0.6.0

## new screensaver: cube
A 3d rotating cube made out of sine waves.

Options change the x, y and z rotation speed, amplitude, frequency and speed of the sine waves.

Use the `--color-speed <SPEED>` option to make the colors shift over time.

# v0.5.0

## new screensaver: bubble
Bubble universe is a screensaver that originally comes from [this 2020 tweet by A-na5](https://twitter.com/yuruyurau/status/1226846058728177665).
The code is adapted from [this TIC-80 code by jtruk](https://tcc.lovebyte.party/day8extra/#tic-80).

Try setting different values for the `-a` and `-b` parameters.

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
