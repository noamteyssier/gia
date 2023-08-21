# `[ gia extend ]`

## Background

This subcommand is used to grow intervals from either of their terminals.
It can be used to either grow both sides, just the left, or just the right.
An optional `-g` flag can be given if the chromosome length is known, which will
be used to truncate growths to not overextend the chromosome length.

## Usage

See full arguments and options using:

```bash
gia extend --help
```

### Extend Left

```text
(input)          x---------y    x------y
==============================================
(output)   x---------------y
                         x-------------y
```

This grows the intervals to the left but truncates to zero to avoid negatives.

```bash
gia extend -i <input.bed> -l 20
```

### Extend Right

```text
(input)          x---------y    x------y
==============================================
(output)         x---------------y
                                x-------------y
```

This grows the intervals to the right but truncates to chromosome max
if a `.genome` file is provided.

```bash
gia extend -i <input.bed> -r 20
```

### Extend Both (Equal)

```text
(input)          x---------y    x------y
==============================================
(output)    x-------------------y
                            x---------------y
```

This grows the intervals to the left and right by an equal amount.
This will truncate to zero if a potential negative is encountered
and will truncate to the chromozome max if a `.genome` file is provided.

```bash
gia extend -i <input.bed> -b 20
```

### Extend Both (Unequal)

```text
(input)          x---------y    x------y
==============================================
(output)    x---------------y
                            x-----------y
```

This grows the intervals to the left and right by an unequal amount.
The growth to the left is controlled by the `-l` flag and the growth
to the right is controlled by the `-r` flag separately.

This will truncate to zero if a potential negative is encountered
and will truncate to the chromozome max if a `.genome` file is provided.

```bash
gia extend -i <input.bed> -l 20 -r 5
```
