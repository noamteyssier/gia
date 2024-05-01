use bedrs::Coordinates;

use super::{NamedBed12, NamedBed3, NamedBed4, NamedBed6};
use std::io::{Error, Write};

pub trait Header {
    fn write_header<W: Write>(&self, output: &mut W) -> Result<(), Error>;
}

impl<'a> Header for NamedBed3<'a> {
    fn write_header<W: Write>(&self, output: &mut W) -> Result<(), Error> {
        writeln!(output, ">{}:{}-{}", self.chr(), self.start(), self.end())
    }
}

impl<'a> Header for NamedBed4<'a> {
    fn write_header<W: Write>(&self, output: &mut W) -> Result<(), Error> {
        writeln!(
            output,
            ">{}:{}-{}::{}",
            self.chr(),
            self.start(),
            self.end(),
            self.name()
        )
    }
}

impl<'a> Header for NamedBed6<'a> {
    fn write_header<W: Write>(&self, output: &mut W) -> Result<(), Error> {
        writeln!(
            output,
            ">{}:{}-{}::{}::{}::{}",
            self.chr(),
            self.start(),
            self.end(),
            self.name(),
            self.score(),
            self.strand().unwrap_or_default(),
        )
    }
}

impl<'a> Header for NamedBed12<'a> {
    fn write_header<W: Write>(&self, output: &mut W) -> Result<(), Error> {
        writeln!(
            output,
            ">{}:{}-{}::{}::{}::{}::{}::{}::{}::{}::{}::{}",
            self.chr(),
            self.start(),
            self.end(),
            self.name(),
            self.score(),
            self.strand().unwrap_or_default(),
            self.thick_start(),
            self.thick_end(),
            self.item_rgb(),
            self.block_count(),
            self.block_sizes(),
            self.block_starts(),
        )
    }
}
