/// This is a macro to match the LHS and RHS BED format and dispatch to some function with some
/// parameters and a writer.
#[macro_export]
macro_rules! dispatch_pair {
    ($reader_a:expr, $reader_b:expr, $writer:expr, $params:expr, $func:expr) => {{
        let mut translater = $reader_a
            .is_named()
            .then_some($crate::types::SplitTranslater::new());
        $crate::dispatch_to_lhs!($reader_a, $reader_b, translater, $writer, $params, $func)
    }};
}

/// This is a macro to match the LHS and RHS BED format and dispatch to some function with some
/// parameters and a writer.
#[macro_export]
macro_rules! dispatch_pair_multi {
    ($reader_a:expr, $reader_b:expr, $writer:expr, $params:expr, $func:expr) => {{
        let mut translater = $reader_a
            .is_named()
            .then_some($crate::types::SplitTranslater::new());
        $crate::dispatch_to_lhs_multi_rhs!(
            $reader_a, $reader_b, translater, $writer, $params, $func
        )
    }};
}

/// This is a macro to match the LHS BED format and dispatch to the RHS
#[macro_export]
macro_rules! dispatch_to_lhs {
    ($reader_a: expr, $reader_b:expr, $translater: expr, $writer:expr, $params:expr, $func:expr) => {
        match $reader_a.input_format() {
            InputFormat::Bed3 => {
                let set_a = $reader_a.bed3_set_with($translater.as_mut())?;
                $crate::dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::Bed4 => {
                let set_a = $reader_a.bed4_set_with($translater.as_mut())?;
                $crate::dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::Bed6 => {
                let set_a = $reader_a.bed6_set_with($translater.as_mut())?;
                $crate::dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::Bed12 => {
                let set_a = $reader_a.bed12_set_with($translater.as_mut())?;
                $crate::dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::Gtf => {
                let set_a = $reader_a.gtf_set_with($translater.as_mut())?;
                $crate::dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::Ambiguous => {
                let set_a = $reader_a.meta_interval_set_with($translater.as_mut())?;
                $crate::dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::BedGraph => {
                let set_a = $reader_a.bedgraph_set_with($translater.as_mut())?;
                $crate::dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
        }
    };
}

/// This is a macro to match the LHS BED format and dispatch to the RHS
#[macro_export]
macro_rules! dispatch_to_lhs_multi_rhs {
    ($reader_a: expr, $reader_b:expr, $translater: expr, $writer:expr, $params:expr, $func:expr) => {
        match $reader_a.input_format() {
            InputFormat::Bed3 => {
                let set_a = $reader_a.bed3_set_with($translater.as_mut())?;
                $crate::dispatch_to_multi_rhs!(
                    set_a,
                    $reader_b,
                    $translater,
                    $writer,
                    $params,
                    $func
                )
            }
            InputFormat::Bed4 => {
                let set_a = $reader_a.bed4_set_with($translater.as_mut())?;
                $crate::dispatch_to_multi_rhs!(
                    set_a,
                    $reader_b,
                    $translater,
                    $writer,
                    $params,
                    $func
                )
            }
            InputFormat::Bed6 => {
                let set_a = $reader_a.bed6_set_with($translater.as_mut())?;
                $crate::dispatch_to_multi_rhs!(
                    set_a,
                    $reader_b,
                    $translater,
                    $writer,
                    $params,
                    $func
                )
            }
            InputFormat::Bed12 => {
                let set_a = $reader_a.bed12_set_with($translater.as_mut())?;
                $crate::dispatch_to_multi_rhs!(
                    set_a,
                    $reader_b,
                    $translater,
                    $writer,
                    $params,
                    $func
                )
            }
            InputFormat::Gtf => {
                let set_a = $reader_a.gtf_set_with($translater.as_mut())?;
                $crate::dispatch_to_multi_rhs!(
                    set_a,
                    $reader_b,
                    $translater,
                    $writer,
                    $params,
                    $func
                )
            }
            InputFormat::Ambiguous => {
                let set_a = $reader_a.meta_interval_set_with($translater.as_mut())?;
                $crate::dispatch_to_multi_rhs!(
                    set_a,
                    $reader_b,
                    $translater,
                    $writer,
                    $params,
                    $func
                )
            }
            InputFormat::BedGraph => {
                let set_a = $reader_a.bedgraph_set_with($translater.as_mut())?;
                $crate::dispatch_to_multi_rhs!(
                    set_a,
                    $reader_b,
                    $translater,
                    $writer,
                    $params,
                    $func
                )
            }
        }
    };
}

/// This is a macro to match the RHS BED format and dispatch to some function with some
/// parameters and a writer.
#[macro_export]
macro_rules! dispatch_to_rhs {
    ($set_a: expr, $reader_b:expr, $translater: expr, $writer:expr, $params:expr, $func:expr) => {
        match $reader_b.input_format() {
            InputFormat::Bed3 => {
                let set_b = $reader_b.bed3_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Bed4 => {
                let set_b = $reader_b.bed4_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Bed6 => {
                let set_b = $reader_b.bed6_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Bed12 => {
                let set_b = $reader_b.bed12_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Gtf => {
                let set_b = $reader_b.gtf_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Ambiguous => {
                let set_b = $reader_b.meta_interval_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::BedGraph => {
                let set_b = $reader_b.bedgraph_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
        }
    };
}

/// This is a macro to match the RHS BED format and dispatch to some function with some
/// parameters and a writer.
#[macro_export]
macro_rules! dispatch_to_multi_rhs {
    ($set_a: expr, $multi_reader:expr, $translater: expr, $writer:expr, $params:expr, $func:expr) => {{
        match $multi_reader[0].input_format() {
            InputFormat::Bed3 => {
                let mut set_b = IntervalContainer::empty();
                for reader in $multi_reader {
                    reader.into_bed3_set_with(&mut set_b, $translater.as_mut())?;
                }
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Bed4 => {
                let mut set_b = IntervalContainer::empty();
                for reader in $multi_reader {
                    reader.into_bed4_set_with(&mut set_b, $translater.as_mut())?;
                }
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Bed6 => {
                let mut set_b = IntervalContainer::empty();
                for reader in $multi_reader {
                    reader.into_bed6_set_with(&mut set_b, $translater.as_mut())?;
                }
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Bed12 => {
                let mut set_b = IntervalContainer::empty();
                for reader in $multi_reader {
                    reader.into_bed12_set_with(&mut set_b, $translater.as_mut())?;
                }
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Gtf => {
                let mut set_b = IntervalContainer::empty();
                for reader in $multi_reader {
                    reader.into_gtf_set_with(&mut set_b, $translater.as_mut())?;
                }
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::BedGraph => {
                let mut set_b = IntervalContainer::empty();
                for reader in $multi_reader {
                    reader.into_bedgraph_set_with(&mut set_b, $translater.as_mut())?;
                }
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Ambiguous => {
                let mut set_b = IntervalContainer::empty();
                for reader in $multi_reader {
                    reader.into_meta_interval_set_with(&mut set_b, $translater.as_mut())?;
                }
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
        }
    }};
}
