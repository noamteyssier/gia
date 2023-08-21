# `[ gia random ]`

## Background

This subcommand will generate a random set of intervals.

## Usage

See full arguments and options using: 

```bash
gia random --help
```

### Parameterization

This generates intervals given:
- Number of Intervals
- Length of Intervals
- Number of Chromosomes
- Max Chromosome Length

The intervals are drawn randomly from a random chromosome and are each
equally sized to the length provided.

An optional `*.genome` file can be given to provide multiple and potentially
differently sized chromosome lengths.

```bash
# will randomly generate intervals given default params
gia random

# sets the number of chromosomes to 10
gia random -c 10

# generates 100 random intervals
gia random -n 100

# generates 100 random intervals each with a length of 500
gia random -n 100 -l 500

# generates 100 random intervals with a length of 500 using a prior genome
gia random -n 100 -l 500 -g <my.genome>
```
