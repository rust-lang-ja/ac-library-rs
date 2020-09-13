use crate::shell::Shell;
use anyhow::{anyhow, bail, Context as _};
use cargo_metadata::{self as cm, MetadataCommand};
use duct::cmd;
use quote::ToTokens as _;
use std::{
    env,
    io::{self, Write as _},
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use syn::{Item, ItemMod};

#[derive(StructOpt, Debug)]
pub struct OptExport {
    /// Save the output to the file
    #[structopt(short, long, value_name("PATH"))]
    output: Option<PathBuf>,
}

pub(crate) fn run(opt: OptExport, shell: &mut Shell) -> anyhow::Result<()> {
    let OptExport { output } = opt;

    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .map_err(|err| match err {
            cm::Error::CargoMetadata { stderr } => {
                anyhow!("{}", stderr.trim_start_matches("error: "))
            }
            err => anyhow!("{}", err),
        })?;

    let cm::Target { src_path, .. } = metadata
        .packages
        .iter()
        .filter(|p| p.manifest_path == metadata.workspace_root.join("Cargo.toml"))
        .flat_map(|p| &p.targets)
        .find(|cm::Target { kind, .. }| *kind == ["lib".to_owned()])
        .with_context(|| "could find the library")?;

    let code = std::fs::read_to_string(src_path)?;
    let syn::File { items, .. } =
        syn::parse_file(&code).with_context(|| format!("`{}` is broken", src_path.display()))?;

    let mut acc = vec!["".to_owned(), "".to_owned()];

    for item in items {
        match item {
            Item::Mod(ItemMod {
                attrs,
                vis,
                ident,
                content: None,
                semi: Some(_),
                ..
            }) => {
                let acc = &mut acc[1];
                let path = src_path
                    .with_file_name(ident.to_string())
                    .with_extension("rs");
                if !path.exists() {
                    unimplemented!("is this `mod.rs`?: {}", ident);
                }
                let content = std::fs::read_to_string(&path)?;
                let is_safe_to_indent = !syn::parse_file(&content)
                    .map_err(|e| anyhow!("{:?}", e))
                    .with_context(|| format!("could not parse `{}`", path.display()))?
                    .into_token_stream()
                    .into_iter()
                    .any(|tt| {
                        matches!(
                            tt, proc_macro2::TokenTree::Literal(lit)
                            if lit.span().start().line != lit.span().end().line
                        )
                    });

                for attr in attrs {
                    *acc += &attr.to_token_stream().to_string();
                    *acc += "\n";
                }
                *acc += &vis.to_token_stream().to_string();
                *acc += " mod ";
                *acc += &ident.to_string();
                *acc += " {\n";
                if is_safe_to_indent {
                    for line in content.lines() {
                        *acc += "    ";
                        *acc += line;
                        *acc += "\n";
                    }
                } else {
                    *acc += &content;
                }
                *acc += "}\n";
            }
            item => {
                let acc = &mut acc[0];
                *acc += &item.to_token_stream().to_string();
                *acc += "\n";
            }
        }
    }

    let acc = rustfmt(&acc.join("\n"))?;

    shell.status(
        "Expanded",
        format!("{} ({} B)", src_path.display(), acc.len()),
    )?;

    if let Some(output) = output {
        std::fs::write(&output, acc)
            .with_context(|| format!("could not write `{}`", output.display()))?;
        shell.status("Wrote", output.display())?;
    } else {
        io::stdout().write_all(acc.as_ref())?;
        io::stdout().flush()?;
    }
    Ok(())
}

fn rustfmt(code: &str) -> anyhow::Result<String> {
    let tempdir = tempfile::Builder::new()
        .prefix("ac-library-rs-xtask")
        .tempdir()?;

    let path = tempdir.path().join("expanded.rs");

    std::fs::write(&path, code)?;

    let rustfmt_exe = Path::new(&env::var_os("CARGO").with_context(|| "missing `$CARGO`")?)
        .with_file_name("rustfmt")
        .with_extension(env::consts::EXE_EXTENSION);

    if !rustfmt_exe.exists() {
        bail!(
            "`{}` does not exist. Run `rustup component add rustfmt` first",
            rustfmt_exe.display(),
        );
    }

    cmd!(rustfmt_exe, "--edition", "2018", &path)
        .run()
        .with_context(|| "could not format the output")?;

    let output = std::fs::read_to_string(path)?;
    tempdir.close()?;
    Ok(output)
}
