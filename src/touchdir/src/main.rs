use clap::Parser;
use touchdir::*;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// List of file extensions
    extensions: Vec<String>,

    /// Directory used for creating files
    #[clap(short, long, default_value = ".")]
    directory: String,

    /// Print filenames to stdout instead of creating them
    #[clap(long)]
    dryrun: bool,
}

fn main() -> Result<(), TouchdirError> {
    let args = Args::parse();

    let mode = TouchdirMode::from_dryrun_bool(args.dryrun);

    Touchdir::new(mode)
        .add_extensions(args.directory, args.extensions)?
        .run()?;

    Ok(())
}
