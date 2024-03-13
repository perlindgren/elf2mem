# elf2mem

Converts elf files into mem files, following the `$readmemh()` format, with address in hex, white space separated hex values in bytes and optional comments as the example below.

```text
@ DEADBEEF // deadbeat address followed by data
00 0F F1 CE // office
DE AD 10 CC // deadlock
BA AA AA AD // baaad

@ 0xC00010FF // cooloff address followed by data
0D 15 EA 5E  // zero disease  
```

## Install

```shell
cargo install .
```

## Use

```shell
elf2mem <elf file>
```