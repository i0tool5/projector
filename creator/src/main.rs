use std::{
    fs::{
        DirBuilder,
        File,
    },
    path::Path,
};

use clap::{Arg, Command};

use template_parser::{self, entities::DirEntry};

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

fn create_path(
    out_dir: &String,
    subdirs: Vec<DirEntry>,
) -> Result<(), std::io::Error> {
    let mut builder =  DirBuilder::new();
    builder.recursive(true);

    let target = out_dir.to_owned();

    for p in subdirs {
        let target = target.clone() + &p.full_path;
        let pth = Path::new(&target);
        if pth.exists() {
            continue;
        }

        if p.is_dir() {
            builder.create(&pth)?;
        }

        if p.is_file() {
            let parent = pth.parent();
            if parent.is_some() {
                builder.create(parent.unwrap())?;
            }
            let file = match File::create(&pth) {
                Err(why) => return Err(why),
                Ok(file) => file,
            };
            
            return file.sync_all();
        }
    }

    Ok(())
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
        let res = create_path(out_dir, directory.walk());
        if res.is_err() {
            panic!("can't create directories {}", res.err().unwrap())
        }
    }

    Ok(())
}
