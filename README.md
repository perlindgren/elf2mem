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

Options:

- `-w <WIDTH>` sets the width of each packet
- `-p` packed (no space in between bytes)
- `-e` flips the endianness of the ELF (defaults to true)
  
## License

MIT
