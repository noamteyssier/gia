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
