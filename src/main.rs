use anyhow::Result;
use cargo_feature_matrix::features::{Feature, FeatureSet};
use cargo_feature_matrix::{Config, TaskKind};
use clap::{
  crate_authors, crate_description, crate_license, crate_name, crate_version,
  AppSettings, ArgEnum, ArgSettings, Parser,
};
use figment::Figment;
use itertools::Itertools;
use std::{env, io::IsTerminal, path::PathBuf};
use yansi::Paint;

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

  /// Colorize output
  #[clap(long, arg_enum, default_value = "auto")]
  color: ColorChoice,

  /// Add these features to the deny list
  #[clap(short, long, require_delimiter = true, setting = ArgSettings::UseValueDelimiter)]
  deny: Vec<String>,

  /// Print a list of all the cargo commands one per line.
  ///
  /// This is intended to be consumed by external job runners.
  #[clap(short, long)]
  print_jobs: bool,

  /// Perform a dry run and print output as if all the jobs succeeded.
  #[clap(long, conflicts_with("print-jobs"))]
  dry_run: bool,

  /// The path to the cargo manifest file to use.
  #[clap(short, long)]
  manifest_path: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug, ArgEnum)]
enum ColorChoice {
  Auto,
  Always,
  Never,
}

fn main() -> Result<()> {
  let mut args = env::args().collect_vec();
  if let Some(name) = args.get(1) {
    if name == "feature-matrix" {
      args.remove(1);
    }
  }

  let Opts {
    command,
    color,
    args,
    deny,
    print_jobs,
    dry_run,
    manifest_path,
  } = Opts::parse_from(args);

  match color {
    ColorChoice::Auto => {
      #[cfg(windows)]
      if !Paint::enable_windows_ascii() {
        Paint::disable();
      }

      if env::var("TERM").map_or(true, |term| term == "dumb")
        || !std::io::stdout().is_terminal()
      {
        Paint::disable();
      }
    }
    ColorChoice::Always => {}
    ColorChoice::Never => Paint::disable(),
  }

  let task = if dry_run {
    TaskKind::DryRun
  } else if print_jobs {
    TaskKind::PrintJobs
  } else {
    TaskKind::Execute
  };

  let mut config = Config::default();
  config.deny = FeatureSet::from_iter(deny.into_iter().map(Feature::from));

  cargo_feature_matrix::run(
    command,
    args,
    task,
    manifest_path,
    Figment::from(config),
  )?;

  Ok(())
}
