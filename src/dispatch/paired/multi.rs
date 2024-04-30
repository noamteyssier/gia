#[macro_export]
macro_rules! dispatch_pair_multi {
    ($reader_a:expr, $reader_b:expr, $writer:expr, $params:expr, $func:expr) => {{
        use $crate::types::InputFormat;

        let mut translater = $reader_a
            .is_named()
            .then_some($crate::types::SplitTranslater::new());
        $crate::dispatch_to_lhs_multi_rhs!(
            $reader_a, $reader_b, translater, $writer, $params, $func
        )
    }};
}

#[macro_export]
macro_rules! dispatch_to_lhs_multi_rhs {
    ($reader_a:expr, $multi_reader:expr, $translater:expr, $writer:expr, $params:expr, $func:expr) => {
        $crate::format_to_lhs_multi_rhs!(
            $reader_a,
            $multi_reader,
            $translater,
            $writer,
            $params,
            $func,
            (Bed3, bed3_set_with),
            (Bed4, bed4_set_with),
            (Bed6, bed6_set_with),
            (Bed12, bed12_set_with),
            (Gtf, gtf_set_with),
            (Ambiguous, meta_interval_set_with),
            (BedGraph, bedgraph_set_with)
        )
    };
}

#[macro_export]
macro_rules! dispatch_to_multi_rhs {
    ($set_a:expr, $multi_reader:expr, $translater:expr, $writer:expr, $params:expr, $func:expr) => {
        $crate::format_to_multi_rhs!(
            $set_a,
            $multi_reader,
            $translater,
            $writer,
            $params,
            $func,
            (Bed3, into_bed3_set_with),
            (Bed4, into_bed4_set_with),
            (Bed6, into_bed6_set_with),
            (Bed12, into_bed12_set_with),
            (Gtf, into_gtf_set_with),
            (BedGraph, into_bedgraph_set_with),
            (Ambiguous, into_meta_interval_set_with)
        )
    };
}

#[macro_export]
macro_rules! format_to_lhs_multi_rhs {
    ($reader_a:expr, $multi_reader:expr, $translater:expr, $writer:expr, $params:expr, $func:expr, $( ($fmt:ident, $method:ident) ),*) => {
        match $reader_a.input_format() {
            $(
                InputFormat::$fmt => {
                    let set_a = $reader_a.$method($translater.as_mut())?;
                    $crate::dispatch_to_multi_rhs!(set_a, $multi_reader, $translater, $writer, $params, $func)
                },
            )*
        }
    };
}

#[macro_export]
macro_rules! format_to_multi_rhs {
    ($set_a:expr, $multi_reader:expr, $translater:expr, $writer:expr, $params:expr, $func:expr, $( ($fmt:ident, $method:ident) ),*) => {
        match $multi_reader[0].input_format() {
            $(
                InputFormat::$fmt => {
                    let mut set_b = IntervalContainer::empty();
                    for reader in $multi_reader {
                        reader.$method(&mut set_b, $translater.as_mut())?;
                    }
                    $func($set_a, set_b, $translater.as_ref(), $params, $writer)
                },
            )*
        }
    };
}
