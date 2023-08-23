# `[ gia get-fasta ]`

## Background

This subcommand is used to extract sequences from intervals provided in an input BED file
from an indexed fasta.

The fasta is assumed indexed using `samtools faidx` and assumes the index is `</path/to.fasta>.fai`.

## Usage

See full arguments and options using:

```bash
gia get-fasta --help
```

### Extract Sequences

If the chromosome names are integers we can extract the sequences using the following command:

```bash
gia get-fasta -b <input.bed> -f <input.fa>
```

#### Input Integer BED

```bed
1	20	30
2	30	40
```

#### Input Integer Fasta

```fasta
>1
ACCCCTATCTATCACACTTCAGCGACTA
CGACTACGACCATCGACGATCAGCATCA
GCATCGACTACGACGATCAGCGACTACG
AGCTACGACGAGCG
>2
GGTAGTTAGTAGAGTTAGACTACGATCG
ATCGATCGATCGAGCGGCGCGCATCGAT
CGTAGCCGCGGCGTACGTAGCGCAGCAG
TCGTAGCTACGTAG
```

#### Output Integer Fasta

```fasta
>1:20-30
AGCGACTACG
>2:30-40
CGATCGATCG
```

### Extract Sequences with non-integer named bed and chromosome names

If the chromosome names are non-integers, `gia` can handle the conversion
and no extra flags are required.

```bash
gia get-fasta -b <input.bed> -f <input.fa>
```

#### Input Non-Integer BED

```bed
chr1	20	30
chr2	30	40
```

#### Input Non-Integer Fasta

```fasta
>chr1
ACCCCTATCTATCACACTTCAGCGACTA
CGACTACGACCATCGACGATCAGCATCA
GCATCGACTACGACGATCAGCGACTACG
AGCTACGACGAGCG
>chr2
GGTAGTTAGTAGAGTTAGACTACGATCG
ATCGATCGATCGAGCGGCGCGCATCGAT
CGTAGCCGCGGCGTACGTAGCGCAGCAG
TCGTAGCTACGTAG
```

#### Output Non-Integer Fasta

```fasta
>chr1:20-30
AGCGACTACG
>chr2:30-40
CGATCGATCG
```

