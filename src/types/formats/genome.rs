use std::io::Read;

use crate::{io::build_reader, types::Translater};
use anyhow::Result;
use csv::ByteRecord;
use hashbrown::HashMap;
use rand::{seq::IteratorRandom, RngCore};

pub struct Genome {
    /// Stores `name_id` to `chr_size`
    map: HashMap<usize, usize>,

    /// Stores `chr_name` to `name_id`
    translater: Option<Translater>,
}
impl Genome {
    pub fn from_params(n_chr: usize, max_chr_len: usize) -> Self {
        let map = (0..n_chr).map(|c| (c + 1, max_chr_len)).collect();
        Self {
            map,
            translater: None,
        }
    }

    pub fn from_reader_unnamed<R: Read>(reader: R) -> Result<Self> {
        let mut reader = build_reader(reader);
        let map = reader
            .deserialize()
            .map(|record| {
                let record: (usize, usize) = record?;
                Ok(record)
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            map,
            translater: None,
        })
    }

    pub fn from_reader_named<'a, R: Read>(reader: R) -> Result<Self> {
        let mut reader = build_reader(reader);
        let mut raw_record = ByteRecord::new();
        let mut translater = Translater::new();
        let mut map = HashMap::new();

        while reader.read_byte_record(&mut raw_record)? {
            let record: (&str, usize) = raw_record.deserialize(None)?;
            translater.add_name(record.0);
            let chr_int = translater.get_idx(record.0).unwrap();
            map.insert(chr_int, record.1);
        }

        Ok(Self {
            map,
            translater: Some(translater),
        })
    }

    pub fn translater(&self) -> Option<&Translater> {
        self.translater.as_ref()
    }

    pub fn sample_chr(&self, rng: &mut impl RngCore) -> usize {
        *self.map.keys().choose(rng).unwrap()
    }

    pub fn chr_size(&self, chr: usize) -> usize {
        *self.map.get(&chr).unwrap()
    }
}
