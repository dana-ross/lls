use crate::permissions::PermissionEffects;
use crate::constants::*;
use std::fs::DirEntry;
use console::Emoji;

pub enum IconType {
    Directory,
    Symlink,
    Executable,
    NoWrite,
    NoAccess,
}

pub fn get_icon_by_type(icon_type: IconType) -> String {
    return match icon_type {
        IconType::Directory => Emoji("📁", EMOJI_FALLBACK_DIRECTORY),
        IconType::Symlink => Emoji("🔗", "=>"),
        IconType::Executable => Emoji("⚙️ ", "Exe"),
        IconType::NoWrite => Emoji("🔏", "RO"),
        IconType::NoAccess => Emoji("🚫", "No"),
    }.to_string();
}

pub fn get_icons_for_direntry(entry: &DirEntry, permission_effects: PermissionEffects) -> String {
    let mut file_icons : String = String::from("");
    let is_dir = entry.file_type().unwrap().is_dir();
    let is_symlink = entry.file_type().unwrap().is_symlink();

    if is_dir {
       file_icons.push_str(get_icon_by_type(IconType::Directory).as_str());
    }

    if is_symlink {
        file_icons.push_str(get_icon_by_type(IconType::Symlink).as_str());
    }

    if permission_effects.no_access {
        file_icons.push_str(get_icon_by_type(IconType::NoAccess).as_str());
    } else if permission_effects.no_write {
        file_icons.push_str(get_icon_by_type(IconType::NoWrite).as_str());
    } else if !is_dir && permission_effects.executable {
        file_icons.push_str(get_icon_by_type(IconType::Executable).as_str());
    }

    return file_icons;
}