use anyhow::Result;
use cargo_feature_matrix::{Config, TaskKind};
use clap::{
    crate_authors, crate_description, crate_license, crate_name, crate_version,
    AppSettings, Parser,
};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(
    name = crate_name!(),
    author = crate_authors!(),
    version = crate_version!(),
    about = crate_description!(),
    license = crate_license!(),
    bin_name = "cargo feature-matrix",
    setting = AppSettings::TrailingVarArg,
)]
struct Opts {
    /// The cargo commands to run.
    command: String,

    /// Arguments to pass to the cargo command
    #[clap(last = true)]
    args: Vec<String>,

    /// Print a list of all the cargo commands one per line.
    ///
    /// This is intended to be consumed by external job runners.
    #[clap(short, long)]
    print_jobs: bool,

    /// Perform a dry run and print output as if all the jobs succeeded.
    #[clap(short, long, conflicts_with("print-jobs"))]
    dry_run: bool,

    /// The path to the cargo manifest file to use.
    #[clap(short, long)]
    manifest_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let Opts {
        command,
        args,
        print_jobs,
        dry_run,
        manifest_path,
    } = Opts::parse();

    let task = if dry_run {
        TaskKind::DryRun
    } else if print_jobs {
        TaskKind::PrintJobs
    } else {
        TaskKind::Execute
    };

    cargo_feature_matrix::run(
        command,
        args,
        task,
        manifest_path,
        Config::default(),
    )?;

    Ok(())
}
