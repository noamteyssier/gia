# `[ gia sort ]`

## Background

This subcommand will sort intervals from an input file.

> **Note:** Chromosome Ordering
>
> If named chromosomes are provided they will be sorted by
> the **order in which they appear in the input file**.
> In the future this will likely be lexicographically ordered.

## Usage

See full arguments and options using:

```bash
gia sort --help
```

### Sort an input BED

You can sort an input file with the following command:

```bash
gia sort -i <input.bed>
```

### Sort a named input BED

If the chromosome names are non-integers you can run the following command:

```bash
gia sort -i <input.bed> -N
```
