mod gzip;
mod text;

use self::gzip::get_gzip_fasta;
use self::text::get_text_fasta;

use crate::cli::GetFastaArgs;
use anyhow::Result;

pub fn get_fasta(args: GetFastaArgs) -> Result<()> {
    if args.fasta.ends_with(".gz") {
        get_gzip_fasta(args)
    } else {
        get_text_fasta(args)
    }
}
