use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::fs;

pub type DirEntryCollection = Vec<fs::DirEntry>;

pub fn marshall_directory_entries(contents: fs::ReadDir) -> DirEntryCollection {
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