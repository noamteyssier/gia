use core::fmt;
use noodles::{
    bam::record::Cigar,
    sam::alignment::record::cigar::{op::Kind, Op},
};
use std::fmt::{Display, Formatter};

/// A wrapper around a CIGAR string for display purposes.
pub struct WrapCigar<'a>(Cigar<'a>);
impl<'a> Display for WrapCigar<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for op in self.0.iter() {
            if let Ok(op) = op {
                write!(f, "{}", WrapOp(op))?;
            } else {
                Err(fmt::Error)?
            }
        }
        Ok(())
    }
}
impl<'a> From<Cigar<'a>> for WrapCigar<'a> {
    fn from(cigar: Cigar<'a>) -> Self {
        Self(cigar)
    }
}

/// A wrapper around a CIGAR operation for display purposes.
struct WrapOp(Op);
impl WrapOp {
    pub fn kind(&self) -> WrapKind {
        self.0.kind().into()
    }
}
impl Display for WrapOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0.len(), self.kind())
    }
}

/// A wrapper around a CIGAR operation kind for display purposes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WrapKind {
    /// An alignment match (`M`).
    Match,
    /// An insertion into the reference (`I`).
    Insertion,
    /// A deletion from the reference (`D`).
    Deletion,
    /// A skipped region from the reference (`N`).
    Skip,
    /// A soft clip (`S`).
    SoftClip,
    /// A hard clip (`H`).
    HardClip,
    /// Padding (`P`).
    Pad,
    /// A sequence match (`=`).
    SequenceMatch,
    /// A sequence mismatch (`X`).
    SequenceMismatch,
}
impl From<Kind> for WrapKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Match => Self::Match,
            Kind::Insertion => Self::Insertion,
            Kind::Deletion => Self::Deletion,
            Kind::Skip => Self::Skip,
            Kind::SoftClip => Self::SoftClip,
            Kind::HardClip => Self::HardClip,
            Kind::Pad => Self::Pad,
            Kind::SequenceMatch => Self::SequenceMatch,
            Kind::SequenceMismatch => Self::SequenceMismatch,
        }
    }
}
impl From<WrapKind> for char {
    fn from(kind: WrapKind) -> Self {
        match kind {
            WrapKind::Match => 'M',
            WrapKind::Insertion => 'I',
            WrapKind::Deletion => 'D',
            WrapKind::Skip => 'N',
            WrapKind::SoftClip => 'S',
            WrapKind::HardClip => 'H',
            WrapKind::Pad => 'P',
            WrapKind::SequenceMatch => '=',
            WrapKind::SequenceMismatch => 'X',
        }
    }
}
impl Display for WrapKind {
    // use serde to convert to string
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
