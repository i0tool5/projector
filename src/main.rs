use template_parser;
use creator;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::command().get_matches();
    let out_dir: &String = matches
        .get_one("out_dir")
        .unwrap();

    let file_path: &String = matches
        .get_one("template_file")
        .expect("path to template file should be provided");

    let template_data = template_parser::read_file(file_path).unwrap();
    let template = template_parser::Template::new(&template_data)?;

    let mut res: Result<(), std::io::Error>;
    for directory in template.directories {
        res = creator::create_path(out_dir, directory.walk());
        if res.is_err() {
            panic!("can't create directories {}", res.err().unwrap())
        }
    }

    let template_files = template.files
        .unwrap_or_default()
        .into_iter()
        .map(|file| -> template_parser::entities::DirEntry {
            template_parser::entities::DirEntry::new(
                template_parser::entities::EntryType::File, file.name
            )
        })
        .collect();

    res = creator::create_path(out_dir, template_files);
    if res.is_err() {
        panic!("can't create files {}", res.err().unwrap())
    }
    
    println!("new project template was created in: {out_dir}");

    Ok(())
}
