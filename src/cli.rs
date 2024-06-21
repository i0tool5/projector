
use clap::{Arg, Command};

pub fn command() -> Command {
    Command::new("projector")
        .about("A project generator utility")
        .arg(
            Arg::new("out_dir")
            .long("out_dir")
            .short('o')
            .required(false)
            .default_value("/tmp/project_template/")
        )
        .arg(
            Arg::new("template_file")
                .long("t_file")
                .short('f')
                .required(true)
                .help("Provides a path to template file")
        )
}