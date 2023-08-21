# `[ gia intersect ]`

## Background

This subcommand calculates the genomic regions found at the intersection
of the first input (query) and the second input (target).

```text
(query)     x-------------y        x----------y   x------y
(target)        i------j        i------j        i-----------j
=================================================================
                i------j           x---j          x------y
```

## Usage

See full arguments and options using:

```bash
gia intersect --help
```

### Default Behavior

By default the intervals in the query and target are not assumed sorted or merged
and intersections are performed inplace.

```bash
gia intersect -a <query.bed> -b <target.bed>
```

### Streamed Intervals

If the intervals in the query and target are presorted then we can operate on them
as a stream - which keeps the memory usage constant and leads to speedups at very
high number of intervals.

```bash
gia intersect -a <query.bed> -b <target.bed> -S
```

### Fractional Overlaps

We can also define conditional operations on fractional overlaps
of the query, target, or both.

This means that the intersection will only be done on query-target pairs which
meet the fractional overlap predicate provided.

#### On Query

```text
              x-------------y   x---------------y
              i---------j       i--j
======================================================
(-f 0.5)
(ix)          x---------j
```

We can supply a minimum overlap requirement on the query with the `-f` flag.

This will only apply intersection operations on query-target pairs in which the
target overlaps the query by the amount required in the `-f` argument.

In the example case, only the first query-target pair was operated upon since
the second did not overlap the query by 50%.

```bash
gia intersect -a <query.bed> -b <target.bed> -f 0.5
```

#### On Target

```text
        x-------------y   x---------------y
              i---------j               i----------j
======================================================
(-F 0.5)
(ix)          i-------y
```

We can supply a minimum overlap requirement on the target with the `-F` flag.

This will only apply intersection operations on query-target pairs in which the
query overlaps the target by the amount required in the `-F` argument.

In the example case, only the first query-target pair was operated upon since
the second did not overlap the target by 50%.

```bash
gia intersect -a <query.bed> -b <target.bed> -F 0.5
```

#### Reciprocal

We can introduce a reciprocal argument (`-r`) which requires the `-f` flag
and requires that both query and target meet the same requirements of the flag.

```bash
gia intersect -a <query.bed> -b <target.bed> -f 0.5 -r
```

#### Either

We can introduce the either flag (`-e`) which is used with **both** the `-f` and `-F` flags.
This will require that **either** condition is met and include those subtraction operations.

```bash
gia intersect -a <query.bed> -b <target.bed> -f 0.5 -F 0.3 -e
```

### Reporting Query, Target, or Inverse

In the cases where we're interested in specifically what intervals are overlapping
and not necessarily their specific intersections we can report either the query or
target interval for each potential intersection event.

```bash
# Reports all query intervals that intersect the target intervals
gia intersect -a <query.bed> -b <target.bed> -q

# Reports all query intervals that intersect the target intervals (only once)
gia intersect -a <query.bed> -b <target.bed> -q -u

# Reports all target intervals that intersect the query intervals
gia intersect -a <query.bed> -b <target.bed> -t
```

We can also report all query intervals that **do not intersect** the target intervals.

This is an homage to `grep -v`

```bash
# Reports all query intervals that do not intersect the target intervals
gia intersect -a <query.bed> -b <target.bed> -v
```
