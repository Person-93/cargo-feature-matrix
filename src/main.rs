use anyhow::Result;
use cargo_feature_matrix::features::{Feature, FeatureSet};
use cargo_feature_matrix::{Config, TaskKind};
use clap::{Parser, ValueEnum};
use figment::Figment;
use itertools::Itertools;
use std::{env, io::IsTerminal, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about, bin_name = "cargo feature-matrix")]
struct Opts {
  /// The cargo command to run.
  #[arg(required = true)]
  command: Option<String>,

  /// Arguments to pass to the cargo command
  #[arg(last = true)]
  args: Vec<String>,

  /// Colorize output
  #[arg(long, value_enum, default_value_t = ColorChoice::Auto)]
  color: ColorChoice,

  /// Add these features to the deny list
  #[arg(short, long, value_delimiter = ',')]
  deny: Vec<String>,

  /// Print a list of all the cargo commands one per line.
  ///
  /// This is intended to be consumed by external job runners.
  #[arg(short, long)]
  print_jobs: bool,

  /// Print all the feature-sets one per line with no command.
  ///
  /// NOTE: This does not include the empty set as the empty set will require
  ///       special handling by consumers anyways.
  #[arg(long, conflicts_with("print_jobs"), conflicts_with("command"))]
  print_matrix: bool,

  /// Perform a dry run and print output as if all the jobs succeeded.
  #[arg(long, conflicts_with("print_jobs"), conflicts_with("print_matrix"))]
  dry_run: bool,

  /// The path to the cargo manifest file to use.
  #[arg(short, long)]
  manifest_path: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
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
    print_matrix,
    dry_run,
    manifest_path,
  } = Opts::parse_from(args);

  match color {
    ColorChoice::Auto => {
      let enable = env::var("TERM").map_or(true, |term| term != "dumb")
        && std::io::stdout().is_terminal();
      if !enable {
        yansi::disable();
      }
    }
    ColorChoice::Always => yansi::enable(),
    ColorChoice::Never => yansi::disable(),
  }

  let task = if dry_run {
    TaskKind::DryRun
  } else if print_jobs {
    TaskKind::PrintJobs
  } else if print_matrix {
    TaskKind::PrintMatrix
  } else {
    TaskKind::Execute
  };

  let mut config = Config::default();
  config.deny = FeatureSet::from_iter(deny.into_iter().map(Feature::from));

  cargo_feature_matrix::run(
    command.unwrap_or_default(),
    args,
    task,
    manifest_path,
    Figment::from(config),
  )?;

  Ok(())
}

#[cfg(test)]
#[test]
fn test_cli() {
  use clap::CommandFactory;
  Opts::command().debug_assert();
}
