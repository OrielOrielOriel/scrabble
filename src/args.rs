extern crate structopt;

pub use structopt::*;
use std::error::Error;


fn parse_key_val<K, V>(s: &str) -> Result<(K, V), Box<dyn Error>>
where
    K: std::str::FromStr,
    K::Err: Error + 'static,
    V: std::str::FromStr,
    V::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found around `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

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
#[structopt(name = "scrabble", setting = structopt::clap::AppSettings::SubcommandRequiredElseHelp)]
pub struct Opts {
    #[structopt(subcommand)]
    pub mode: Option<Modes>,

    #[structopt(short, parse(from_occurrences))]
    pub verbosity: u8,

    #[structopt(long)]
    pub accessible: bool,
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
    #[structopt(required = true,
                short, 
                long, 
                use_delimiter = true, 
                parse(try_from_str = parse_key_val),
                number_of_values = 1)]
    pub key: Vec<(String, String)>,

    /// Expects a pattern or a file
    #[structopt(required = true, short, long)]
    pub pattern: String,

    /// Output file
    #[structopt(short, long)]
    pub output: Option<String>,
    
    /// Logging
    #[structopt(long)]
    #[allow(clippy::option_option)]
    pub log: Option<Option<String>>
}

#[derive(StructOpt, Debug)]
pub struct SkewerOpts {
    
}


