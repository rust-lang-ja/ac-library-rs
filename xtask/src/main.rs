use structopt::StructOpt as _;
use xtask::{shell::Shell, Opt};

fn main() {
    let opt = Opt::from_args();
    let mut shell = Shell::new();
    if let Err(err) = xtask::run(opt, &mut shell) {
        xtask::exit_with_error(err, &mut shell);
    }
}
