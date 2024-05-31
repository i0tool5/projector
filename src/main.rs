use clap::{Arg, Command};

use template_parser;
use creator;

fn cli() -> Command {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    let out_dir: &String = matches
        .get_one("out_dir")
        .unwrap();

    let file_path: &String = matches
        .get_one("template_file")
        .expect("path to template file should be provided");

    let template_data = template_parser::read_file(file_path).unwrap();
    let template = template_parser::Template::new(&template_data)?;

    for directory in template.directories {
        let res = creator::create_path(out_dir, directory.walk());
        if res.is_err() {
            panic!("can't create directories {}", res.err().unwrap())
        }
    }

    println!("new project template was created in: {out_dir}");

    Ok(())
}
