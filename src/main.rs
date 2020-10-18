mod args;
use args::Opts;

mod permute;
use permute::do_everything;

fn main() {
   let global_opts = Opts::read();
   
   let mode = match global_opts.mode {
    Some(it) => it,
    _ => return,
};
  match mode {
     args::Modes::Curate(opts) => {
        println!("Curate")
     },
     args::Modes::Ingest(opts) => {
        println!("Ingest")
     },
     args::Modes::Permute(opts) => {
        permute::do_everything();
     },
     args::Modes::Skewer(opts) => {
        println!("Skewer")
     },
  }
}
