# `[ gia sample ]`

## Background

This subcommand will randomly sample intervals from an input file.

> **Note:** The output of this command is **not guaranteed** to be sorted.

## Usage

See full arguments and options using:

```bash
gia sample --help
```

### Sample a fixed number of intervals

You can sample a fixed number of intervals from the dataset using the `-n` flag

```bash
# samples 100 intervals from the dataset
gia sample -i <input.bed> -n 100
```

### Sample a fractional number of intervals

You can sample a fractional number of intervals from the total size of the dataset
using the `-f` flag.

```bash
# samples 0.1 (10%) of the intervals
gia sample -i <input.bed> -f 0.1
```
