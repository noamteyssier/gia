use crate::{
    io::{build_reader, match_input},
    types::Translater,
};
use anyhow::Result;
use csv::ByteRecord;
use hashbrown::HashMap;
use rand::{seq::IteratorRandom, RngCore};
use std::io::Read;

pub struct Genome<'a> {
    /// Stores `name_id` to `chr_size`
    map: HashMap<usize, usize>,

    /// Stores `chr_name` to `name_id`
    translater: Option<&'a Translater>,
}
impl<'a> Genome<'a> {
    pub fn from_params(n_chr: usize, max_chr_len: usize) -> Self {
        let map = (0..n_chr).map(|c| (c + 1, max_chr_len)).collect();
        Self {
            map,
            translater: None,
        }
    }

    #[allow(dead_code)]
    pub fn from_reader<R: Read>(reader: R, translater: Option<&'a mut Translater>) -> Result<Self> {
        match translater {
            Some(translater) => Self::from_reader_named(reader, translater),
            None => Self::from_reader_unnamed(reader),
        }
    }

    pub fn from_reader_immutable<R: Read>(
        reader: R,
        translater: Option<&'a Translater>,
        break_on_missing: bool,
    ) -> Result<Self> {
        match translater {
            Some(translater) => {
                Self::from_reader_named_immutable(reader, translater, break_on_missing)
            }
            None => Self::from_reader_unnamed(reader),
        }
    }

    pub fn from_path_immutable_with(
        path: String,
        translater: Option<&'a Translater>,
        break_on_missing: bool,
    ) -> Result<Self> {
        let handle = match_input(Some(path))?;
        Self::from_reader_immutable(handle, translater, break_on_missing)
    }

    pub fn from_opt_path_immutable_with(
        path: Option<String>,
        translater: Option<&'a Translater>,
        break_on_missing: bool,
    ) -> Result<Option<Self>> {
        if let Some(path) = path {
            Self::from_path_immutable_with(path, translater, break_on_missing).map(Some)
        } else {
            Ok(None)
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

    pub fn from_reader_named<R: Read>(reader: R, translater: &'a mut Translater) -> Result<Self> {
        let mut reader = build_reader(reader);
        let mut raw_record = ByteRecord::new();
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

    pub fn from_reader_named_immutable<R: Read>(
        reader: R,
        translater: &'a Translater,
        break_on_missing: bool,
    ) -> Result<Self> {
        let mut reader = build_reader(reader);
        let mut raw_record = ByteRecord::new();
        let mut map = HashMap::new();

        while reader.read_byte_record(&mut raw_record)? {
            let record: (&str, usize) = raw_record.deserialize(None)?;
            if let Some(chr_int) = translater.get_idx(record.0) {
                map.insert(chr_int, record.1);
            } else if break_on_missing {
                anyhow::bail!(
                    "Genome file contains chromosome name not in BED file: {}",
                    record.0
                );
            }
        }

        Ok(Self {
            map,
            translater: Some(translater),
        })
    }

    pub fn translater(&self) -> Option<&Translater> {
        self.translater
    }

    pub fn sample_chr(&self, rng: &mut impl RngCore) -> usize {
        *self.map.keys().choose(rng).unwrap()
    }

    pub fn chr_size(&self, chr: usize) -> Option<&usize> {
        self.map.get(&chr)
    }

    pub fn chr_size_unchecked(&self, chr: usize) -> usize {
        *self.map.get(&chr).unwrap()
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::types::Translate;

    const GENOME_UNNAMED: &[u8] = b"1\t1000\n2\t2000\n3\t3000\n";
    const GENOME_NAMED: &[u8] = b"chr1\t1000\nchr2\t2000\nchr3\t3000\n";

    #[test]
    fn test_genome_unnamed() {
        let genome = Genome::from_reader_unnamed(GENOME_UNNAMED).unwrap();
        assert_eq!(genome.chr_size_unchecked(1), 1000);
        assert_eq!(genome.chr_size_unchecked(2), 2000);
        assert_eq!(genome.chr_size_unchecked(3), 3000);
        assert!(genome.translater().is_none());
    }

    #[test]
    fn test_genome_named() {
        let mut translater = Translater::new();
        let genome = Genome::from_reader_named(GENOME_NAMED, &mut translater).unwrap();
        assert_eq!(genome.chr_size_unchecked(1), 1000);
        assert_eq!(genome.chr_size_unchecked(2), 2000);
        assert_eq!(genome.chr_size_unchecked(3), 3000);
        assert!(genome.translater().is_some());
        let translater = genome.translater().unwrap();
        assert_eq!(translater.get_chr_name(0).unwrap(), ".");
        assert_eq!(translater.get_chr_name(1).unwrap(), "chr1");
        assert_eq!(translater.get_chr_name(2).unwrap(), "chr2");
        assert_eq!(translater.get_chr_name(3).unwrap(), "chr3");
    }

    #[test]
    fn test_sampling() {
        let mut rng = rand::thread_rng();
        let genome = Genome::from_reader_unnamed(GENOME_UNNAMED).unwrap();
        let chr = genome.sample_chr(&mut rng);
        (1..=3).contains(&chr);
    }
}
