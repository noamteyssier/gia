/// This is a macro to match the input format and dispatch to some function with some parameters and
/// a writer.
#[macro_export]
macro_rules! dispatch_single {
    ($reader:expr, $writer:expr, $params:expr, $func:expr) => {
        match $reader.input_format() {
            InputFormat::Bed3 => {
                let (set, translater) = $reader.bed3_set()?;
                $func(set, translater, $params, $writer)
            }
            InputFormat::Bed6 => {
                let (set, translater) = $reader.bed6_set()?;
                $func(set, translater, $params, $writer)
            }
            InputFormat::Bed12 => {
                let (set, translater) = $reader.bed12_set()?;
                $func(set, translater, $params, $writer)
            }
        }
    };
}

/// This is a macro to match the LHS and RHS BED format and dispatch to some function with some
/// parameters and a writer.
#[macro_export]
macro_rules! dispatch_pair {
    ($reader_a:expr, $reader_b:expr, $writer:expr, $params:expr, $func:expr) => {{
        let mut translater = $reader_a.is_named().then_some(Translater::new());
        dispatch_to_lhs!($reader_a, $reader_b, translater, $writer, $params, $func)
    }};
}

/// This is a macro to match the LHS BED format and dispatch to the RHS
#[macro_export]
macro_rules! dispatch_to_lhs {
    ($reader_a: expr, $reader_b:expr, $translater: expr, $writer:expr, $params:expr, $func:expr) => {
        match $reader_a.input_format() {
            InputFormat::Bed3 => {
                let set_a = $reader_a.bed3_set_with($translater.as_mut())?;
                dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::Bed6 => {
                let set_a = $reader_a.bed6_set_with($translater.as_mut())?;
                dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
            }
            InputFormat::Bed12 => {
                let set_a = $reader_a.bed12_set_with($translater.as_mut())?;
                dispatch_to_rhs!(set_a, $reader_b, $translater, $writer, $params, $func)
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
            InputFormat::Bed6 => {
                let set_b = $reader_b.bed6_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
            InputFormat::Bed12 => {
                let set_b = $reader_b.bed12_set_with($translater.as_mut())?;
                $func($set_a, set_b, $translater.as_ref(), $params, $writer)
            }
        }
    };
}
