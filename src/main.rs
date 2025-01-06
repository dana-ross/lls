mod icons;
mod permissions;
mod massage_direntries;
mod constants;

use std::env;
use std::process;
use std::fs;
use std::path::PathBuf;
use std::io::Error;
use std::os::unix::fs::MetadataExt;
use icons::get_icons_for_direntry;
use massage_direntries::marshall_directory_entries;
use clap::Parser;
use uzers::{Users, UsersCache};

#[derive(Parser)]
#[command(name = constants::APP_NAME)]
#[command(version = constants::RELEASE_VERSION)]
#[command(about = constants::SHORT_DESCRIPTION, long_about = constants::LONG_DESCRIPTION)]
struct Args {
    path: Option<String>,

    /// use a long listing format
    #[arg(short, long, default_value_t = false)]
    long: bool,
}

fn main() {
    process::exit(lls());
}

fn lls() -> i32 {
    let args = Args::parse();

    let path = match get_path(&args) {
        Ok(path) => path,
        Err(_) => { println!("Problem getting the target directory");return 1;}
    };

    let read_dir_result = fs::read_dir(path);
    let contents = match read_dir_result {
        Ok(contents) => contents,
        Err(_) => {println!("Problem reading directory contents"); return 2;}
    };

    let directory_entries = marshall_directory_entries(contents);

    if args.long {
        let cache = UsersCache::new();
        for entry in directory_entries {
            long_listing(&entry, &cache);
        }
    } else {
        for entry in directory_entries {
            let permission_effects = permissions::permission_effects_for_direntry(&entry);
            let file_icons = get_icons_for_direntry(&entry, permission_effects);
            short_listing(&entry, &file_icons);
        }
    }
    
    println!("");

    return 0;
}

fn short_listing(entry: &fs::DirEntry, file_icons: &str) -> () {
    print!("[{}{}{}] ", file_icons, if !file_icons.is_empty() { " " } else { "" }, entry.file_name().to_str().unwrap());
}

fn long_listing(entry: &fs::DirEntry, cache: &UsersCache) -> () {
    let file_size = entry.metadata().unwrap().len();
    let file_user_name = cache.get_user_by_uid(entry.metadata().unwrap().uid()).unwrap();
    let file_group_name = cache.get_user_by_uid(entry.metadata().unwrap().gid()).unwrap();
    println!("{} {:>10} {:<10} {:<10} {}", permissions::format_permissions(entry), file_size, file_user_name.name().to_str().unwrap(), file_group_name.name().to_str().unwrap(), entry.file_name().to_str().unwrap());
}

fn get_path(args: &Args) -> Result<PathBuf, Error> {
    let requested_path = &args.path;

    if requested_path.is_some() {
        return Ok(PathBuf::from(requested_path.as_ref().unwrap()));
    } else {
        return env::current_dir();
    }
}