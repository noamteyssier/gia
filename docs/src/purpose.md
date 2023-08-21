# Purpose - i.e. why gia?

`gia` was developed to split the difference between [`bedtools`](https://bedtools.readthedocs.io/en/latest/)
and [`bedops`](https://bedops.readthedocs.io/en/latest/) and be a tool that can
match both philosophies without sacrificing either efficiency or convenience.

That being said, the author of `gia` was greatly inspired by both tools and has
used them extensively for years.

`gia` would not exist if not for the work of their authors and maintainers as well
as their meticulous documentation.

## Philosophies

### `bedtools` - utility over efficiency

[`bedtools`](https://bedtools.readthedocs.io/en/latest/) is the original
genome interval toolkit and is the go-to for many people.

It prioritizes convenience and utility over computational efficiency, and
does that very well.

One of the major design choices for most of the tools in the toolkit is that
the genome interval sets are loaded into memory completely before processing
occurs.

This incurs a huge memory and computational overhead once genome interval sets
get larger and larger - which is increasingly the case for large high throughput
genomic datasets.

### `bedops` - efficiency over utility

[`bedops`](https://bedops.readthedocs.io/en/latest/) came later from `bedtools`
and was built for computational efficiency.

Most of the methods within focus around pre-sorted data, and the computational
and memory efficiency comes from the fact that everything is built around [streams](https://en.wikipedia.org/wiki/Stream_(computing))
(i.e. intervals are assumed sorted and only kept in memory for the abosolute minimum
amount of time required for the operation.)

This leads to highly efficient streaming operations with a constant memory overhead,
but provides some inconveniences, as all inputs must be presorted, and some functional
limitations, as most of the set operations implicitly merge intervals on input.

### `gia` - both in a single tool

`gia` was built with the idea that both philosophies are useful for different
purposes and that the same operations and underlying implementations can be
shared.

By default, all tools are built with an **inplace** memory load, which allows
for the complete set of functionality available in `bedtools` with no expectation
that the dataset is *a priori* sorted or merged, but where relevant an argument
may be passed to allow for **streaming** operations, which perform highly performant
memory constant operations on pre-sorted inputs such as in `bedops`.
