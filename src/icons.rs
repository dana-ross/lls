use crate::permissions::PermissionEffects;
use std::fs::DirEntry;

pub enum IconType {
    Directory,
    Executable,
    NoWrite,
    NoAccess,
}

pub fn get_icon_by_type(icon_type: IconType) -> String {
    return match icon_type {
        IconType::Directory => "📁",
        IconType::Executable => "⚙️ ",
        IconType::NoWrite => "🔏",
        IconType::NoAccess => "🚫"
    }.to_string();
}

pub fn get_icons_for_direntry(entry: &DirEntry, permission_effects: PermissionEffects) -> String {
    let mut file_icons : String = String::from("");
    let is_dir = entry.file_type().unwrap().is_dir();

    if is_dir {
       file_icons.push_str(get_icon_by_type(IconType::Directory).as_str());
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