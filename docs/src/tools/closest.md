# `[ gia closest ]`

## Background

Similar to [intersect](./intersect.md), `closest` searches for the closest
feature in `B` for each feature in `A`.

The `closest` feature is not necessarily non-overlapping and under certain
search constraints may not exist.

> **Note:** Chromosomal Distance
>
> There is no notion of interchromosomal distance, so if an interval is
> alone on an interval in `A`, then there will be no closest interval
> in `B`.

## Usage

### Default Behavior

```text
(a)           x----------y
(b)   i-----j                 i-------j
=========================================
      i-----j
```

By default `closest` will return the single closest, potentially overlapping, feature in `B`
for each interval in `A`.

Ties will be given to the left-most interval with the closest start coordinate to the query.

```bash
gia closest -a <input.bed> -b <input.bed>
```

### Closest Upstream

```text
(a)               x----------y
(b)   i-----j                 i-------j
=========================================
      i-----j
```

You can explicitly exclude all downstream intervals from the search, regardless
of their distance, with the upstream flag.

```bash
gia closest -a <input.bed> -b <input.bed> -u
```

### Closest Downstream

```text
(a)          x----------y
(b)   i-----j                 i-------j
=========================================
                              i-------j
```

You can explicitly exclude all upstream intervals from the search, regardless
of their distance, with the downstream flag.

```bash
gia closest -a <input.bed> -b <input.bed> -d
```
