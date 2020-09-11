mod commands;
pub mod shell;

use crate::commands::export::OptExport;
use crate::shell::Shell;
use std::io::Write as _;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(bin_name("cargo xtask"))]
pub enum Opt {
    /// Export the library
    Export(OptExport),
}

pub fn run(opt: Opt, shell: &mut Shell) -> anyhow::Result<()> {
    match opt {
        Opt::Export(opt) => commands::export::run(opt, shell),
    }
}

pub fn exit_with_error(err: anyhow::Error, shell: &mut Shell) -> ! {
    let _ = shell.error(&err);

    for cause in err.chain().skip(1) {
        let _ = writeln!(shell.err(), "\nCaused by:");

        for line in cause.to_string().lines() {
            let _ = match line {
                "" => writeln!(shell.err()),
                line => writeln!(shell.err(), "  {}", line),
            };
        }
    }

    std::process::exit(1);
}
