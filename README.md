# terminal-toys
A collection of terminal screensavers

![screenshot](https://github.com/user-attachments/assets/61bcb66a-4963-4dc5-8b90-c21736f5cce2)
## Screensavers
### 3d pipes
https://github.com/user-attachments/assets/e2425ca8-b1fd-46b0-895c-74e3edbf159c
### bouncy balls
https://github.com/user-attachments/assets/17bb430a-8b08-44ee-afce-faf2bcc9205d
### splits
https://github.com/user-attachments/assets/79d8a8c2-4549-4d5a-aa0e-466159fb300c
### falling sand
https://github.com/user-attachments/assets/db9570ac-ed66-4afe-8e28-af47050fa13d
### game of life
https://github.com/user-attachments/assets/c1acfead-9885-4aa2-a81d-e784d26ea62d

## Installation
Install Rust if you don't have it yet: https://rustup.rs/
```
cargo install --git https://github.com/Seebass22/terminal-toys
```

## Usage
List all screensavers by running `terminal-toys` without arguments (or with `--help`, `-h` or `help`)
```
$ terminal-toys -h
Usage: terminal-toys <COMMAND>

Commands:
  balls    Bouncy balls!
  pipes3d  3d pipe screensaver
  splits   Lines that split after a while
  life     Game of life
  sand     Falling sand
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
Each screensaver has its' own `--help`/`-h`
```
$ terminal-toys balls -h
Bouncy balls!

Usage: terminal-toys balls [OPTIONS]

Options:
  -m, --marker <TYPE>      Marker type (Braille, Dot, Bar, Block, HalfBlock) [default: Braille]
  -n, --max-balls <BALLS>  Number of balls to spawn [default: 50]
  -h, --help               Print help
```
If your terminal font does not support braille characters, try using `-m HalfBlock`
