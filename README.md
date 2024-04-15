# elf2mem

Converts `elf` files into `.mem` files, following the `$readmemh()` format, with address in hex, data in hex values and optional comments as the example below.

```text
@DEADBEEF // deadbeat address followed by data
000FF1CE // office
DEAD10CC // deadlock
BAAAAAAD // baaad

@C00010FF // cooloff address followed by data
0D15EA5E  // zero disease  
```

## Install

Clone repository and run:

```shell
cargo install path .
```

or directly from git:

```shell
cargo install --git https://github.com/perlindgren/elf2mem.git
```

## Usage

For help:

```shell
Extract .text and .data elf sections to Verilog .mem file

Usage: elf2mem [OPTIONS]

Options:
  -o, --out-dir <OUT_DIR>  Path for output
  -f, --file <FILE>        Input file in elf format [default: app.elf]
  -w, --width <WIDTH>      Width in bytes per package [default: 4]
  -p, --packed             Packed [default: non-packed (spaces)]
  -n, --native             Native byte order [default: flipped byte order]
  -h, --help               Print help
```

To generate mem files for the `.rodata` and `.text` sections:

```shell
elf2mem -f app.elf
```

Creates `text.mem`, and `data_[0..4].mem` in current folder by default, or according to `--out-dir` if provided.

```shell
elf2mem
```

Defaults to `app.elf` if `--file` is omitted:
  
## License

MIT
