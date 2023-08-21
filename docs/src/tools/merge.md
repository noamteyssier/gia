# `[ gia merge ]`

## Background

This subcommand will merge all **overlapping** and **bordering** intervals
within the input set.

It can accept either a presorted input (which can be streamed) or an unsorted
input which will first be sorted in-place before merging.

## Usage

See full arguments and options using:

```bash
gia merge --help
```

### Default Behavior (Inplace)

```text
(e)                    q----r
(b)      k----l
(a)    i----j
(c)           l----m
(d)                  o----p
===============================
(1)    i-----------m
(2)                  o------r
```

This will merge all overlapping and bordering intervals into their sub-spans.

It **does not assume presorted input** and will sort the input inplace on load.

```bash
gia merge -i <input.bed>
```

### Streamable Input

```text
(a)    i----j
(b)      k----l
(c)           l----m
(d)                  o----p
(e)                    q----r
===============================
(1)    i-----------m
(2)                  o------r
```

This will merge all overlapping and bordering intervals into their sub-spans.

This **assumes presorted input** and will have undefined behavior is input is not-sorted.

```bash
gia merge -i <input.bed> -S
```
