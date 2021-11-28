use clap::{
    crate_authors, crate_description, crate_license, crate_name, crate_version,
    Parser,
};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(
    name = crate_name!(),
    author = crate_authors!(),
    version = crate_version!(),
    about = crate_description!(),
    license = crate_license!(),
)]
struct Opts {
    /// The cargo commands to run.
    #[clap(min_values(1))]
    commands: Vec<String>,

    /// Print a list of all the cargo commands one per line.
    ///
    /// This is intended to be consumed by external job runners.
    #[clap(short, long)]
    print_jobs: bool,

    /// Perform a dry run and print output as if all the jobs succeeded.
    #[clap(short, long, conflicts_with("print-jobs"))]
    dry_run: bool,

    /// The number of jobs to run in parallel.
    /// Zero will use the number of CPUs.
    #[clap(short, long, default_value = "0", conflicts_with("print-jobs"))]
    jobs: u8,

    /// The path to the cargo manifest file to use.
    #[clap(short, long)]
    manifest_path: Option<PathBuf>,
}

fn main() {
    println!("{:#?}", Opts::parse());
}
