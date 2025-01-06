mod icons;
mod permissions;
mod massage_direntries;
mod constants;

use std::env;
use std::process;
use std::fs;
use icons::get_icons_for_direntry;
use massage_direntries::marshall_directory_entries;

fn main() {
    process::exit(lls());
}

fn lls() -> i32 {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(_) => { println!("Problem getting the current directory");return 1;}
    };

    let read_dir_result = fs::read_dir(current_dir);
    let contents = match read_dir_result {
        Ok(contents) => contents,
        Err(_) => {println!("Problem reading directory contents"); return 2;}
    };

    let directory_entries = marshall_directory_entries(contents);
    
    for entry in directory_entries {
        let permission_effects = permissions::permission_effects_for_direntry(&entry);
        let file_icons = get_icons_for_direntry(&entry, permission_effects);
        
        print!("[{}{}{}] ", file_icons, if !file_icons.is_empty() { " " } else { "" }, entry.file_name().to_str().unwrap());
    }

    print!("\n");
    
    return 0;
}
