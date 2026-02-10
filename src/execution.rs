use crate::{Error, features::FeatureMatrix};
use lazy_static::lazy_static;
use std::{
  env, ffi::OsString, io::Write, path::Path, process::Command, process::Stdio,
};
use yansi::Paint;

pub(crate) struct Task<'t> {
  matrix: FeatureMatrix<'t>,
  manifest_path: Option<&'t Path>,
  package_name: &'t str,
  args: &'t [String],
  command: &'t str,
  kind: TaskKind,
}

#[derive(Copy, Clone)]
pub enum TaskKind {
  DryRun,
  PrintJobs,
  PrintMatrix,
  Execute,
}

impl<'t> Task<'t> {
  pub(crate) fn new(
    kind: TaskKind,
    command: &'t str,
    manifest_path: Option<&'t Path>,
    package_name: &'t str,
    args: &'t [String],
    matrix: FeatureMatrix<'t>,
  ) -> Self {
    Self {
      matrix,
      manifest_path,
      package_name,
      args,
      command,
      kind,
    }
  }

  pub(crate) fn run(self) -> Result<(), Error> {
    match self.kind {
      TaskKind::DryRun => self.execute(true),
      TaskKind::PrintJobs => {
        self.print_jobs();
        Ok(())
      }
      TaskKind::PrintMatrix => {
        self.print_matrix();
        Ok(())
      }
      TaskKind::Execute => self.execute(false),
    }
  }

  fn execute(self, dry_run: bool) -> Result<(), Error> {
    let mut exit_status = None;
    for feature_set in self.matrix {
      let feature_set = feature_set.to_string();

      let cmd = if dry_run {
        None
      } else {
        let mut cmd = Command::new(CARGO.as_os_str());
        cmd
          .arg(self.command)
          .args(self.args.iter())
          .stderr(Stdio::piped())
          .stdout(Stdio::piped())
          .arg("--no-default-features")
          .arg("--package")
          .arg(self.package_name);

        if let Some(manifest_path) = self.manifest_path {
          cmd.arg("--manifest-path").arg(manifest_path);
        }

        if !feature_set.is_empty() {
          cmd.arg("--features").arg(&feature_set);
        }

        Some(cmd)
      };

      print!(
        "{}{} {}{} {}{}{}......",
        "running: cmd=".cyan(),
        self.command,
        "package=".cyan(),
        self.package_name,
        "features=[".cyan(),
        feature_set,
        "]".cyan(),
      );
      std::io::stdout().flush().expect("failed to flush stdout");

      // let on_success = || println!("{}", Black.style().bg(Green).paint("OK"));
      let on_success = || println!("{}", "OK".black().on_green());

      match cmd {
        None => on_success(),
        Some(mut cmd) => {
          let output = cmd.output().map_err(|err| Error::Io {
            message: "failed to get output of cargo command",
            source: err,
          })?;
          if output.status.success() {
            on_success();
          } else {
            // println!("{}", Black.style().bg(Red).paint("Fail"));
            println!("{}", "Fail".black().on_red());
            exit_status = Some(output.status);
            std::io::stderr()
              .write_all(&output.stderr)
              .expect("failed to write to stderr");
            std::io::stdout()
              .write_all(&output.stdout)
              .expect("failed to write to stdout");
          }
        }
      };
    }

    match exit_status {
      Some(exit_status) => Err(Error::Fail(exit_status)),
      None => Ok(()),
    }
  }

  fn print_jobs(self) {
    let prefix = format!(
      "{} {} --package {} {}",
      CARGO.to_string_lossy(),
      self.command,
      self.package_name,
      self.args.join(" ")
    );
    for feature_set in self.matrix {
      if feature_set.is_empty() {
        println!("{}", prefix);
      } else {
        println!("{} --features {}", prefix, feature_set);
      }
    }
  }

  fn print_matrix(self) {
    for feature_set in self.matrix {
      if feature_set.is_empty() {
        continue;
      }
      println!("{feature_set}");
    }
  }
}

lazy_static! {
  static ref CARGO: OsString =
    env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
}
