use std::io::Write;

use anyhow::{bail, Result};
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;

use crate::{
    io::{
        match_output, write_pairs_iter_with, write_records_iter_with, BedReader, WriteNamedIter,
        WriteNamedIterImpl,
    },
    types::{InputFormat, IntervalPair, Rename, Renamer, Translater},
    utils::sort_pairs,
};

fn windowed_set_overlaps<'a, Ia, Ib, Na, Nb, W>(
    set_a: &'a mut IntervalContainer<Ia, usize, usize>,
    set_b: &'a mut IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a Translater>,
    left: usize,
    right: usize,
    inverse: bool,
    output: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Serialize + Copy,
    Ib: IntervalBounds<usize, usize> + Serialize + Copy,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    Nb: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<Ia> + WriteNamedIter<Ib>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    sort_pairs(set_a, set_b, false);
    if inverse {
        let iv_iter = set_a
            .iter()
            .map(|iv| {
                let mut w_iv = *iv;
                w_iv.extend_left(&left);
                w_iv.extend_right(&right, None);
                (iv, w_iv)
            })
            .filter(|(_iv, w_iv)| {
                let overlaps = set_b.find_iter_sorted_unchecked(w_iv).count();
                overlaps == 0
            })
            .map(|(iv, _w_iv)| *iv);
        write_records_iter_with(iv_iter, output, translater)?;
    } else {
        let windows_iter = set_a.iter().map(|iv| {
            let mut w_iv = *iv;
            w_iv.extend_left(&left);
            w_iv.extend_right(&right, None);
            (iv, w_iv)
        });
        let pairs_iter = windows_iter.flat_map(|(iv, w_iv)| {
            let overlaps = set_b.find_iter_sorted_owned_unchecked(w_iv);
            overlaps.map(|ov| IntervalPair::new(*iv, *ov, translater))
        });
        write_pairs_iter_with(pairs_iter, output, translater)?;
    }
    Ok(())
}

fn dispatch_window<W: Write>(
    reader_a: BedReader,
    reader_b: BedReader,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    inverse: bool,
    output: W,
) -> Result<()> {
    if reader_a.is_named() != reader_b.is_named() {
        bail!("Input files must both be named or both be unnamed");
    }
    let mut translater = if reader_a.is_named() {
        Some(Translater::new())
    } else {
        None
    };
    let (left, right) = if let Some(b) = both {
        (b, b)
    } else {
        (left.unwrap_or(0), right.unwrap_or(0))
    };
    match reader_a.input_format() {
        InputFormat::Bed3 => {
            let mut set_a = reader_a.bed3_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let mut set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
                InputFormat::Bed6 => {
                    let mut set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
                InputFormat::Bed12 => {
                    let mut set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
            }
        }
        InputFormat::Bed6 => {
            let mut set_a = reader_a.bed6_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let mut set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
                InputFormat::Bed6 => {
                    let mut set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
                InputFormat::Bed12 => {
                    let mut set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
            }
        }
        InputFormat::Bed12 => {
            let mut set_a = reader_a.bed12_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let mut set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
                InputFormat::Bed6 => {
                    let mut set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
                InputFormat::Bed12 => {
                    let mut set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    windowed_set_overlaps(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        left,
                        right,
                        inverse,
                        output,
                    )
                }
            }
        }
    }
}

pub fn window(
    path_a: Option<String>,
    path_b: String,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    inverse: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed_a = BedReader::from_path(path_a, None, None)?;
    let bed_b = BedReader::from_path(Some(path_b), None, None)?;
    let output_handle = match_output(output, compression_threads, compression_level)?;
    dispatch_window(bed_a, bed_b, both, left, right, inverse, output_handle)
}
