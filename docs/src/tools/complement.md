# `[ gia complement ]`

## Background

Similar to [subtract](./subtract.md), `complement` generates all the intervals
that are **not covered** by the input set. 

This is equivalent to subtracting the interval set from its *span*, which is
an interval defined by the min and max interval of the set by chromosome.

## Usage

See full arguments and options using:

```bash
gia complement --help
```

### Default Behavior

```text
(span)        s---------------------------------s
==========================================================
(input)       x------y     x-----y    x---------y
==========================================================
(complement)         y-----x     y----x
```

By default `complement` will return all intervals that are uncovered by the
span of the incoming interval set.

The span of the interval set is calculated by chromosome.

> **Note:** Internal vs Complete Complement
>
> The internal complement of a set necessarily excludes the the chromosomal
> start to the span start and the span end to the chromosomal end. 
> 
> All other potential intervals are included.
