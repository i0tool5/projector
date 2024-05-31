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
    entries: Vec<DirEntry>,
) -> Result<(), std::io::Error> {
    let mut builder =  DirBuilder::new();
    builder.recursive(true);

    let target = out_dir.to_owned();
    
    for entry in entries {
        let target = target.clone() + &entry.full_path;
        let pth = Path::new(&target);
        if pth.exists() {
            continue;
        }

        if entry.is_dir() {
            builder.create(&pth)?;
        }

        if entry.is_file() {
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
