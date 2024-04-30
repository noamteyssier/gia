#[macro_export]
macro_rules! dispatch_single {
    ($reader:expr, $writer:expr, $params:expr, $func:expr) => {{
        use $crate::types::InputFormat;

        $crate::format_cases!(
            $reader,
            $func,
            $params,
            $writer,
            (Bed3, bed3_set),
            (Bed4, bed4_set),
            (Bed6, bed6_set),
            (Bed12, bed12_set),
            (Gtf, gtf_set),
            (Ambiguous, meta_interval_set),
            (BedGraph, bedgraph_set)
        )
    }};
}
#[macro_export]
macro_rules! format_cases {
    ($reader:expr, $func:expr, $params:expr, $writer:expr, $( ($fmt:ident, $method:ident) ),* ) => {
        match $reader.input_format() {
            $(
                InputFormat::$fmt => $crate::gen_single!($reader, $func, $params, $writer, $method),
            )*
        }
    };
}

#[macro_export]
macro_rules! gen_single {
    ($reader:expr, $func:expr, $params:expr, $writer:expr, $set_method:ident) => {{
        let (set, translator) = $reader.$set_method()?;
        $func(set, translator.as_ref(), $params, $writer)
    }};
}
