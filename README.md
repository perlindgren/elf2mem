# elf2mem

Converts elf files into mem files, following the `$readmemh()` format, with address in hex, data in hex values and optional comments as the example below.

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
elf2mem --help
Extract .text and .data elf sections to Verilog .mem file

Usage: elf2mem [OPTIONS]

Options:
  -d, --out-data <OUT_DATA>  Optional name for generated .data file [default: <input file>_data.mem)
  -t, --out-text <OUT_TEXT>  
  -f, --file <FILE>          Input file in elf format [default: app.elf]
  -w, --width <WIDTH>        Width in bytes per package [default: 4]
  -s, --spaced               Inject spaces between bytes [default: packed (no spaces)]
  -n, --native               Native byte order [default: flipped byte order]
  -h, --help                 Print help
```

To generate mem files for the .data and .text sections:

```shell
elf2mem -f app.elf -d app_data.mem -t app_text.mem
```

Defaults to `_text.mem`, and `_data.mem` extensions of the elf file if out file omitted:

```shell
elf2mem -f app.elf 
```

Defaults to `app.elf` if elf file is omitted:

```shell
elf2mem
```
  
## License

MIT
