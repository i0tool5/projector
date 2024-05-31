use std::{
    fs::{
        DirBuilder,
        File,
    },
    path::Path,
};

use template_parser::entities::DirEntry;

pub fn create_path(
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
