use super::build_writer;
use crate::types::{NumericBed12, NumericBed6, Translate};
use anyhow::Result;
use bedrs::{Coordinates, GenomicInterval};
use serde::Serialize;
use std::{io::Write, marker::PhantomData};

pub fn write_records_iter_with<W, I, Co, Tr>(
    records: I,
    writer: W,
    translater: Option<&Tr>,
) -> Result<()>
where
    W: Write,
    I: Iterator<Item = Co>,
    Co: Coordinates<usize, usize> + Serialize,
    Tr: Translate,
    WriteNamedIterImpl: WriteNamedIter<Co>,
{
    if let Some(translater) = translater {
        WriteNamedIterImpl::write_named_iter(writer, records, translater)?;
    } else {
        WriteIterImpl::<Co>::write_iter(writer, records)?;
    }
    Ok(())
}

pub fn write_3col_iter_with<W, I, Co, Tr>(
    records: I,
    writer: W,
    translater: Option<&Tr>,
) -> Result<()>
where
    W: Write,
    I: Iterator<Item = Co>,
    Co: Coordinates<usize, usize> + Serialize,
    Tr: Translate,
    WriteNamedIterImpl: WriteNamedIter<Co>,
{
    if let Some(translater) = translater {
        WriteNamedIterImpl::write_named_3col_iter(writer, records, translater)?;
    } else {
        WriteIterImpl::<Co>::write_3col_iter(writer, records)?;
    }
    Ok(())
}

pub trait WriteIter<C>
where
    C: Coordinates<usize, usize>,
{
    fn write_iter<W: Write, It: Iterator<Item = C>>(writer: W, iterator: It) -> Result<()>;
    fn write_3col_iter<W: Write, It: Iterator<Item = C>>(writer: W, iterator: It) -> Result<()>;
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

    fn write_3col_iter<W: Write, It: Iterator<Item = C>>(writer: W, iterator: It) -> Result<()> {
        let mut wtr = build_writer(writer);
        for interval in iterator {
            let named_interval = (interval.chr(), interval.start(), interval.end());
            wtr.serialize(named_interval)?;
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
    fn write_named_iter<W: Write, It: Iterator<Item = C>, Tr: Translate>(
        writer: W,
        iterator: It,
        translater: &Tr,
    ) -> Result<()> {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn write_named_3col_iter<W: Write, It: Iterator<Item = C>, Tr: Translate>(
        writer: W,
        iterator: It,
        translater: &Tr,
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
pub struct WriteNamedIterImpl;
impl WriteNamedIter<GenomicInterval<usize>> for WriteNamedIterImpl {
    fn write_named_iter<W: Write, It: Iterator<Item = GenomicInterval<usize>>, Tr: Translate>(
        writer: W,
        iterator: It,
        translater: &Tr,
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
impl<'a> WriteNamedIter<&'a GenomicInterval<usize>> for WriteNamedIterImpl {
    fn write_named_iter<
        W: Write,
        It: Iterator<Item = &'a GenomicInterval<usize>>,
        Tr: Translate,
    >(
        writer: W,
        iterator: It,
        translater: &Tr,
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
    fn write_named_iter<W: Write, It: Iterator<Item = NumericBed6>, Tr: Translate>(
        writer: W,
        iterator: It,
        translater: &Tr,
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
impl<'a> WriteNamedIter<&'a NumericBed6> for WriteNamedIterImpl {
    fn write_named_iter<W: Write, It: Iterator<Item = &'a NumericBed6>, Tr: Translate>(
        writer: W,
        iterator: It,
        translater: &Tr,
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
impl WriteNamedIter<NumericBed12> for WriteNamedIterImpl {
    fn write_named_iter<W: Write, It: Iterator<Item = NumericBed12>, Tr: Translate>(
        writer: W,
        iterator: It,
        translater: &Tr,
    ) -> Result<()> {
        let mut wtr = build_writer(writer);
        for interval in iterator {
            let chr = translater.get_name(*interval.chr()).unwrap();
            let name = translater.get_name(interval.name()).unwrap();
            let item_rgb = translater.get_name(interval.item_rgb).unwrap();
            let block_count = translater.get_name(interval.block_count).unwrap();
            let block_sizes = translater.get_name(interval.block_sizes).unwrap();
            let block_starts = translater.get_name(interval.block_starts).unwrap();
            let named_interval = (
                chr,
                interval.start(),
                interval.end(),
                name,
                interval.score,
                interval.strand,
                interval.thick_start,
                interval.thick_end,
                item_rgb,
                block_count,
                block_sizes,
                block_starts,
            );
            wtr.serialize(named_interval)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
impl<'a> WriteNamedIter<&'a NumericBed12> for WriteNamedIterImpl {
    fn write_named_iter<W: Write, It: Iterator<Item = &'a NumericBed12>, Tr: Translate>(
        writer: W,
        iterator: It,
        translater: &Tr,
    ) -> Result<()> {
        let mut wtr = build_writer(writer);
        for interval in iterator {
            let chr = translater.get_name(*interval.chr()).unwrap();
            let name = translater.get_name(interval.name()).unwrap();
            let item_rgb = translater.get_name(interval.item_rgb).unwrap();
            let block_count = translater.get_name(interval.block_count).unwrap();
            let block_sizes = translater.get_name(interval.block_sizes).unwrap();
            let block_starts = translater.get_name(interval.block_starts).unwrap();
            let named_interval = (
                chr,
                interval.start(),
                interval.end(),
                name,
                interval.score,
                interval.strand,
                interval.thick_start,
                interval.thick_end,
                item_rgb,
                block_count,
                block_sizes,
                block_starts,
            );
            wtr.serialize(named_interval)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
