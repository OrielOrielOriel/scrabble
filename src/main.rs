mod args;
use args::Opts;

fn main() {
   let opts = Opts::read();
   
   println!("{:?}", opts)
}
