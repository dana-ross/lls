use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::fmt::Write;
use uzers::{get_effective_uid, get_effective_gid};

pub struct PermissionEffects {
    pub no_access: bool,
    pub no_write: bool,
    pub executable: bool,
}

pub fn permission_effects_for_direntry(entry: &DirEntry) -> PermissionEffects {
    let permissions = entry.metadata().unwrap().permissions().mode();
    let file_uid = entry.metadata().unwrap().uid();
    let file_gid = entry.metadata().unwrap().gid();

    let user_uid = get_effective_uid();
    let user_gid = get_effective_gid();

    let user_id_match = user_uid == file_uid;
    let user_gid_match = user_gid == file_gid;

    let ur = user_id_match && ((permissions & 0o400) != 0);
    let uw = user_id_match && ((permissions & 0o200) != 0);
    let ux = user_id_match && ((permissions & 0o100) != 0);
    let gr = user_gid_match && ((permissions & 0o040) != 0);
    let gw = user_gid_match && ((permissions & 0o020) != 0);
    let gx = user_gid_match && ((permissions & 0o010) != 0);
    let or = !(user_id_match || user_gid_match) && ((permissions & 0o004) != 0);
    let ow = !(user_id_match || user_gid_match) && ((permissions & 0o002) != 0);
    let ox = !(user_id_match || user_gid_match) && ((permissions & 0o001) != 0);

    let no_access = !(ur || uw || ux || gr || gw || gx || or || ow || ox);
    let no_write = !(uw || gw || ow);
    let executable = ux || gx || ox;

    return PermissionEffects {
        no_access: no_access,
        no_write: no_write,
        executable: executable,
    }
}

pub fn format_permissions(entry: &DirEntry) -> String {
    let permissions = entry.metadata().unwrap().permissions().mode();

    let is_dir = entry.file_type().unwrap().is_dir();

    let file_uid = entry.metadata().unwrap().uid();
    let file_gid = entry.metadata().unwrap().gid();

    let user_uid = get_effective_uid();
    let user_gid = get_effective_gid();

    let user_id_match = user_uid == file_uid;
    let user_gid_match = user_gid == file_gid;

    let ur = user_id_match && ((permissions & 0o400) != 0);
    let uw = user_id_match && ((permissions & 0o200) != 0);
    let ux = user_id_match && ((permissions & 0o100) != 0);
    let gr = user_gid_match && ((permissions & 0o040) != 0);
    let gw = user_gid_match && ((permissions & 0o020) != 0);
    let gx = user_gid_match && ((permissions & 0o010) != 0);
    let or = !(user_id_match || user_gid_match) && ((permissions & 0o004) != 0);
    let ow = !(user_id_match || user_gid_match) && ((permissions & 0o002) != 0);
    let ox = !(user_id_match || user_gid_match) && ((permissions & 0o001) != 0);

    let mut permissions = String::new();
    write!(
        &mut permissions,
        "{}{}{}{}{}{}{}{}{}{}",
        if is_dir { 'd' } else { '-' },
        if ur { 'r' } else { '-' },
        if uw { 'w' } else { '-' },
        if ux { 'x' } else { '-' },
        if gr { 'r' } else { '-' },
        if gw { 'w' } else { '-' },
        if gx { 'x' } else { '-' },
        if or { 'r' } else { '-' },
        if ow { 'w' } else { '-' },
        if ox { 'x' } else { '-' },
    ).unwrap();

    return permissions;
}