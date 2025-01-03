use std::env;
use std::process;
use std::fs;
use std::fs::DirEntry;
use std::cmp::Ordering;
use std::ffi::OsStr;

type DirEntryCollection = Vec<fs::DirEntry>;
struct DirContents {
    subdirectories : DirEntryCollection,
    files : DirEntryCollection
}

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
    
    for entry in directory_entries.subdirectories {
        print!("[📁 {}] ", entry.file_name().to_str().unwrap());
    }
    for entry in directory_entries.files {
        print!("{} ", entry.file_name().to_str().unwrap());
    }

    print!("\n");
    
    return 0;
}

fn marshall_directory_entries(contents: fs::ReadDir) -> DirContents {
    let mut subdirectories : DirEntryCollection = vec!();
    let mut files : DirEntryCollection = vec!();

    for entry in contents {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            subdirectories.push(entry);
        } else {
            files.push(entry);
        }
    }

    subdirectories.sort_by(sort_direntries);

    return DirContents {
        subdirectories: subdirectories,
        files: files
    };

}

fn sort_direntries(a: &DirEntry, b: &DirEntry) -> Ordering {
    let a_filename = a.file_name();
    let b_filename = b.file_name();
    return sort_filenames(&a_filename.as_os_str(), &b_filename.as_os_str());
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