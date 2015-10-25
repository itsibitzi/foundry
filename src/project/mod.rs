use std::fs::{DirBuilder, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

use cargo::core::{ColorConfig, MultiShell, Shell, ShellConfig, Verbosity};
use cargo::util::Config;

mod history;

static PROJECT_METADATA_DIR: &'static str = ".foundry";
static PROJECT_HISTORY_DIR: &'static str = ".foundry/history";

static CARGO_OUT: &'static str = ".foundry/cargo.out";
static CARGO_ERR: &'static str = ".foundry/cargo.err";

static SHELL_CONFIG: ShellConfig = ShellConfig {color_config: ColorConfig::Never, tty: false};

pub struct Project {
    pub config: Config,
}

impl Project {
    pub fn open(verbose: bool) -> Project {
        DirBuilder::new().recursive(true).create(PROJECT_HISTORY_DIR);

        fn create_project_shell<P: AsRef<Path>>(path: P) -> Shell {
            let file = OpenOptions::new().write(true).append(true).create(true).open(path).unwrap();
            let writer = BufWriter::new(file);

            Shell::create(Box::new(writer), SHELL_CONFIG)
        }

        let out = create_project_shell(CARGO_OUT);
        let err = create_project_shell(CARGO_ERR);

        let verbosity = if verbose {
            Verbosity::Verbose
        } else {
            Verbosity::Normal
        };

        let shell = MultiShell::new(out, err, verbosity);

        Project {
            config: Config::new(shell).unwrap(),
        }
    }
}
