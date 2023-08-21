# Background

## Set Theory on the Genome

### Genomic Intervals

The core piece of genome interval arithmetic is the `interval` object:

```rust
GenomicInterval {
    Chromosome,
    Start,
    End,
}
```

This is an abstract object with *at minimum* 3 attributes defining its **chromosome**,
**start**, and **end** positions on the genome.

### Genomic Interval Sets

A collection of genomic intervals can be considered a [set](https://en.wikipedia.org/wiki/Set_(mathematics))
which in brief is a collection of objects that match particular properties.

There is a branch of mathematics known as [set theory](https://en.wikipedia.org/wiki/Set_theory)
which describe a range of operations, such as the
[union](https://en.wikipedia.org/wiki/Union_(set_theory)),
[intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory)),
[difference](https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement),
[complement](https://en.wikipedia.org/wiki/Complement_(set_theory)),
etc of those sets.

Some examples of these are shown below:

#### Intersection

This generates all the intervals that are at the [intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory))
of two interval sets.

```text
(a)   x---------y    x-----------y
(b)     i--j  i--------j    i--------j
========================================
        i--j  i-y    x-j    i----y
```

#### Difference

This generates all the intervals in `a` which are **not covered** by `b`.
This calculates the [difference / relative complement](https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement)
of a set.

```text
(a)  x----------y   x------------y
(b)     i--j  i--------j    i--------j
========================================
     x--i  j--i        j----i    
```

#### Complement

This generates all the intervals in `a` which are not covered by its span (`s`).
This is the [absolute complement](https://en.wikipedia.org/wiki/Complement_(set_theory))
of the set.

```text
(s)  s----------------------------s
========================================
(a)  x-----y   x------y    x------y
========================================
           y---x      y----x    
```

### Genomic Interval Arithmetic

Genomic interval arithmetic revolves around performing set theoretical operations
in the context of genomic regions, and is useful for a wide range of purposes
in bioinformatics analyses.
