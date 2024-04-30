/// This is a macro to match the input format and dispatch to some function with some parameters and
/// a writer.
#[macro_export]
macro_rules! dispatch_single {
    ($reader:expr, $writer:expr, $params:expr, $func:expr) => {{
        use $crate::types::InputFormat;

        match $reader.input_format() {
            InputFormat::Bed3 => $crate::gen_single!($reader, $func, $params, $writer, bed3_set),
            InputFormat::Bed4 => $crate::gen_single!($reader, $func, $params, $writer, bed4_set),
            InputFormat::Bed6 => $crate::gen_single!($reader, $func, $params, $writer, bed6_set),
            InputFormat::Bed12 => $crate::gen_single!($reader, $func, $params, $writer, bed12_set),
            InputFormat::Gtf => $crate::gen_single!($reader, $func, $params, $writer, gtf_set),
            InputFormat::Ambiguous => {
                $crate::gen_single!($reader, $func, $params, $writer, meta_interval_set)
            }
            InputFormat::BedGraph => {
                $crate::gen_single!($reader, $func, $params, $writer, bedgraph_set)
            }
        }
    }};
}

/// This is a macro to match the input format and dispatch to some function with some parameters and
/// a writer with an owned translater.
#[macro_export]
macro_rules! dispatch_single_owned_tl {
    ($reader:expr, $writer:expr, $params:expr, $func:expr) => {{
        use $crate::types::InputFormat;

        match $reader.input_format() {
            InputFormat::Bed3 => {
                $crate::gen_single_owned_tl!($reader, $func, $params, $writer, bed3_set)
            }
            InputFormat::Bed4 => {
                $crate::gen_single_owned_tl!($reader, $func, $params, $writer, bed4_set)
            }
            InputFormat::Bed6 => {
                $crate::gen_single_owned_tl!($reader, $func, $params, $writer, bed6_set)
            }
            InputFormat::Bed12 => {
                $crate::gen_single_owned_tl!($reader, $func, $params, $writer, bed12_set)
            }
            InputFormat::Gtf => {
                $crate::gen_single_owned_tl!($reader, $func, $params, $writer, gtf_set)
            }
            InputFormat::Ambiguous => {
                $crate::gen_single_owned_tl!($reader, $func, $params, $writer, meta_interval_set)
            }
            InputFormat::BedGraph => {
                $crate::gen_single_owned_tl!($reader, $func, $params, $writer, bedgraph_set)
            }
        }
    }};
}

/// This is a macro to match the input format and dispatch to some function with some parameters and
/// a writer alongside an HTS Reader.
#[macro_export]
macro_rules! dispatch_single_with_htslib {
    ($hts_reader:expr, $bed_reader:expr, $writer:expr, $params:expr, $func:expr) => {{
        use $crate::types::InputFormat;

        match $bed_reader.input_format() {
            InputFormat::Bed3 => {
                $crate::gen_single_hts!($hts_reader, $bed_reader, $func, $params, $writer, bed3_set)
            }
            InputFormat::Bed4 => {
                $crate::gen_single_hts!($hts_reader, $bed_reader, $func, $params, $writer, bed4_set)
            }
            InputFormat::Bed6 => {
                $crate::gen_single_hts!($hts_reader, $bed_reader, $func, $params, $writer, bed6_set)
            }
            InputFormat::Bed12 => {
                $crate::gen_single_hts!(
                    $hts_reader,
                    $bed_reader,
                    $func,
                    $params,
                    $writer,
                    bed12_set
                )
            }
            InputFormat::Gtf => {
                $crate::gen_single_hts!($hts_reader, $bed_reader, $func, $params, $writer, gtf_set)
            }
            InputFormat::Ambiguous => {
                $crate::gen_single_hts!(
                    $hts_reader,
                    $bed_reader,
                    $func,
                    $params,
                    $writer,
                    meta_interval_set
                )
            }
            InputFormat::BedGraph => {
                $crate::gen_single_hts!(
                    $hts_reader,
                    $bed_reader,
                    $func,
                    $params,
                    $writer,
                    bedgraph_set
                )
            }
        }
    }};
}

/// Shared behavior for each match arm - just changes which IO function is called
#[macro_export]
macro_rules! gen_single {
    ($reader:expr, $func:expr, $params:expr, $writer:expr, $set_method:ident) => {{
        let (set, translater) = $reader.$set_method()?;
        $func(set, translater.as_ref(), $params, $writer)
    }};
}

/// Shared behavior for each match arm - just changes which IO function is called
#[macro_export]
macro_rules! gen_single_owned_tl {
    ($reader:expr, $func:expr, $params:expr, $writer:expr, $set_method:ident) => {{
        let (set, translater) = $reader.$set_method()?;
        $func(set, translater, $params, $writer)
    }};
}

/// Shared behavior for each match arm - just changes which IO function is called
/// but using an HTS Reader.
#[macro_export]
macro_rules! gen_single_hts {
    ($hts_reader:expr, $reader:expr, $func:expr, $params:expr, $writer:expr, $set_method:ident) => {{
        let (set, translater) = $reader.$set_method()?;
        $func($hts_reader, set, translater.as_ref(), $params, $writer)
    }};
}
