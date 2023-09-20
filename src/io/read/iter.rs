use std::io::Read;
use bedrs::traits::{IntervalBounds, ChromBounds, ValueBounds};
use serde::de::DeserializeOwned;

pub fn iter_unnamed<'a, R, I, C, T>(
    reader: &'a mut csv::Reader<R>,
) -> Box<dyn Iterator<Item = I> + 'a>
where
    R: Read,
    I: IntervalBounds<C, T> + DeserializeOwned + 'a,
    C: ChromBounds,
    T: ValueBounds,
{
    let record_iter = reader
        .deserialize()
        .map(|record| {
            let record: I = match record {
                Ok(record) => record,
                Err(e) => {
                    panic!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            record
        });
    Box::new(record_iter)
}


