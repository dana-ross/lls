mod icons;
mod permissions;

use std::env;
use std::process;
use std::fs;
use std::fs::DirEntry;
use std::cmp::Ordering;
use std::ffi::OsStr;
use icons::IconType;

use icons::get_icon_by_type;

type DirEntryCollection = Vec<fs::DirEntry>;

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
        let file_icon: IconType = 
            if entry.file_type().unwrap().is_dir() {
                IconType::Directory
            } else if permission_effects.no_access {
                IconType::NoAccess
            } else if permission_effects.executable {
                IconType::Executable
            } else if permission_effects.no_write {
                IconType::NoWrite
            } else {
                IconType::None
            };
        
        print!("[{} {}] ", get_icon_by_type(file_icon), entry.file_name().to_str().unwrap());
    }

    print!("\n");
    
    return 0;
}

fn marshall_directory_entries(contents: fs::ReadDir) -> DirEntryCollection {
    let mut files : DirEntryCollection = vec!();

    for entry in contents {
        let entry = entry.unwrap();
            files.push(entry);
    }

    files.sort_by(sort_direntries);

    return files;
}

fn sort_direntries(a: &DirEntry, b: &DirEntry) -> Ordering {
    let a_filename = a.file_name();
    let b_filename = b.file_name();

    let a_is_dir = a.file_type().unwrap().is_dir();
    let b_is_dir = b.file_type().unwrap().is_dir();

    if a_is_dir && !b_is_dir {
        return Ordering::Less;
    } else if b_is_dir && !a_is_dir {
        return Ordering::Greater;
    } else {
        return sort_filenames(&a_filename.as_os_str(), &b_filename.as_os_str());
    }
}

fn sort_filenames(a: &OsStr, b: &OsStr) -> Ordering {
    let a_is_dotfile = a.to_str().unwrap().chars().next().unwrap() == '.';
    let b_is_dotfile = b.to_str().unwrap().chars().next().unwrap() == '.';

    // Force dotfiles (hidden on *nix) to the top
    if a_is_dotfile == b_is_dotfile {
        return a.cmp(&b);
    } else if a_is_dotfile && !b_is_dotfile {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}