use crate::{features::FeatureMatrix, Error};
use lazy_static::lazy_static;
use std::{env, ffi::OsString, io::Write, process::Command, process::Stdio};
use termion::color;

pub(crate) struct Task<'t> {
    matrix: FeatureMatrix<'t>,
    package_name: &'t str,
    args: &'t [String],
    command: &'t str,
    kind: TaskKind,
}

#[derive(Copy, Clone)]
pub enum TaskKind {
    DryRun,
    PrintJobs,
    Execute,
}

impl<'t> Task<'t> {
    pub(crate) fn new(
        kind: TaskKind,
        command: &'t str,
        package_name: &'t str,
        args: &'t [String],
        matrix: FeatureMatrix<'t>,
    ) -> Self {
        Self {
            matrix,
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
                cmd.arg(self.command)
                    .args(self.args.iter())
                    .stderr(Stdio::piped())
                    .stdout(Stdio::null());

                if !feature_set.is_empty() {
                    cmd.arg("--features").arg(feature_set.to_string());
                }

                Some(cmd)
            };

            print!(
                "{}running: cmd={}{}{} package={}{}{} features=[{}{}{}]{}......",
                color::Fg(color::LightCyan),

                color::Fg(color::Reset),
                self.command,
                color::Fg(color::LightCyan),

                color::Fg(color::Reset),
                self.package_name,
                color::Fg(color::LightCyan),

                color::Fg(color::Reset),
                feature_set,
                color::Fg(color::LightCyan),

                color::Fg(color::Reset),
            );

            let on_success = || {
                println!(
                    "{}{}OK{}{}",
                    color::Bg(color::LightGreen),
                    color::Fg(color::Black),
                    color::Bg(color::Reset),
                    color::Fg(color::Reset)
                )
            };

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
                        println!(
                            "{}{}Fail{}{}",
                            color::Bg(color::LightRed),
                            color::Fg(color::Black),
                            color::Bg(color::Reset),
                            color::Fg(color::Reset)
                        );
                        exit_status = Some(output.status);
                        std::io::stderr()
                            .write_all(&output.stderr)
                            .expect("failed to write to stderr");
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
            "{} {} {}",
            CARGO.to_string_lossy(),
            self.command,
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
}

lazy_static! {
    static ref CARGO: OsString =
        env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
}
