extern crate structopt;

use structopt::*;

#[derive(StructOpt, Debug)]
pub enum Modes {
    #[structopt(name = "curate", help = "Do curate things.")]
    Curate(CurateOpts),

    #[structopt(name = "ingest", help = "Do ingest things.")]
    Ingest(IngestOpts),

    #[structopt(name = "permute", help = "Do permute things.")]
    Permute(PermuteOpts),

    #[structopt(name = "skewer", help = "Do skewer things.")]
    Skewer(SkewerOpts),
}
#[derive(StructOpt, Debug)]
pub struct Opts {
    #[structopt(subcommand)]
    mode: Modes,
}

impl Opts {
    pub fn read() -> Self {
        let opts = Opts::from_args();
        
        opts
    }
}

#[derive(StructOpt, Debug)]
pub struct CurateOpts {

}

#[derive(StructOpt, Debug)]
pub struct IngestOpts {
    
}

#[derive(StructOpt, Debug)]
pub struct PermuteOpts {
    /// Expects: -k key,value
    #[structopt(short, long, use_delimiter = true)]
    key: Vec<String>,

    /// Expects a pattern or a file
    #[structopt(short, long)]
    pattern: String,
}

#[derive(StructOpt, Debug)]
pub struct SkewerOpts {
    
}

