use super::build_writer;
use crate::types::{NumericBed6, Translate, Translater};
use anyhow::Result;
use bedrs::{Coordinates, GenomicInterval};
use serde::Serialize;
use std::{io::Write, marker::PhantomData};

pub fn write_records_iter_with<W, I, Co>(
    records: I,
    writer: W,
    translater: Option<&Translater>,
) -> Result<()>
where
    W: Write,
    I: Iterator<Item = Co>,
    Co: Coordinates<usize, usize> + Serialize,
    WriteNamedIterImpl: WriteNamedIter<Co>,
{
    if let Some(translater) = translater {
        WriteNamedIterImpl::write_named_iter(writer, records, translater)?;
    } else {
        WriteIterImpl::<Co>::write_iter(writer, records)?;
    }
    Ok(())
}

pub trait WriteIter<C>
where
    C: Coordinates<usize, usize>,
{
    fn write_iter<W: Write, It: Iterator<Item = C>>(writer: W, iterator: It) -> Result<()>;
}

pub struct WriteIterImpl<C>
where
    C: Coordinates<usize, usize>,
{
    phantom: PhantomData<C>,
}
impl<C> WriteIter<C> for WriteIterImpl<C>
where
    C: Coordinates<usize, usize> + Serialize,
{
    fn write_iter<W: Write, It: Iterator<Item = C>>(writer: W, iterator: It) -> Result<()> {
        let mut wtr = build_writer(writer);
        for interval in iterator {
            wtr.serialize(interval)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

pub trait WriteNamedIter<C>
where
    C: Coordinates<usize, usize>,
{
    #[allow(unused_variables)]
    fn write_named_iter<W: Write, It: Iterator<Item = C>>(
        writer: W,
        iterator: It,
        translater: &Translater,
    ) -> Result<()> {
        unimplemented!()
    }
}
pub struct WriteNamedIterImpl;
impl WriteNamedIter<GenomicInterval<usize>> for WriteNamedIterImpl {
    fn write_named_iter<W: Write, It: Iterator<Item = GenomicInterval<usize>>>(
        writer: W,
        iterator: It,
        translater: &Translater,
    ) -> Result<()> {
        let mut wtr = build_writer(writer);
        for interval in iterator {
            let chr = translater.get_name(*interval.chr()).unwrap();
            let named_interval = (chr, interval.start(), interval.end());
            wtr.serialize(named_interval)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
impl WriteNamedIter<NumericBed6> for WriteNamedIterImpl {
    fn write_named_iter<W: Write, It: Iterator<Item = NumericBed6>>(
        writer: W,
        iterator: It,
        translater: &Translater,
    ) -> Result<()> {
        let mut wtr = build_writer(writer);
        for interval in iterator {
            let chr = translater.get_name(*interval.chr()).unwrap();
            let name = translater.get_name(interval.name()).unwrap();
            let named_interval = (
                chr,
                interval.start(),
                interval.end(),
                name,
                interval.score,
                interval.strand(),
            );
            wtr.serialize(named_interval)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
