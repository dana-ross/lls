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

struct DirEntryPermissionDetails {
    is_dir: bool,

    ur: bool,
    uw: bool,
    ux: bool,
    gr: bool,
    gw: bool,
    gx: bool,
    or: bool,
    ow: bool,
    ox: bool,
}

pub fn permission_effects_for_direntry(entry: &DirEntry) -> PermissionEffects {
    let permission_details = get_direntry_permission_details(entry);

    let no_access = !(permission_details.ur || permission_details.uw || permission_details.ux || permission_details.gr || permission_details.gw || permission_details.gx || permission_details.or || permission_details.ow || permission_details.ox);
    let no_write = !(permission_details.uw || permission_details.gw || permission_details.ow);
    let executable = permission_details.ux || permission_details.gx || permission_details.ox;

    return PermissionEffects {
        no_access: no_access,
        no_write: no_write,
        executable: executable,
    }
}

pub fn format_permissions(entry: &DirEntry) -> String {
    let permission_details = get_direntry_permission_details(entry);
    let mut permissions = String::new();
    write!(
        &mut permissions,
        "{}{}{}{}{}{}{}{}{}{}",
        if permission_details.is_dir { 'd' } else { '-' },
        if permission_details.ur { 'r' } else { '-' },
        if permission_details.uw { 'w' } else { '-' },
        if permission_details.ux { 'x' } else { '-' },
        if permission_details.gr { 'r' } else { '-' },
        if permission_details.gw { 'w' } else { '-' },
        if permission_details.gx { 'x' } else { '-' },
        if permission_details.or { 'r' } else { '-' },
        if permission_details.ow { 'w' } else { '-' },
        if permission_details.ox { 'x' } else { '-' },
    ).unwrap();

    return permissions;
}

fn get_direntry_permission_details(entry: &DirEntry) -> DirEntryPermissionDetails {
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

    return DirEntryPermissionDetails {
        is_dir: is_dir,

        ur: ur,
        uw: uw,
        ux: ux,
        gr: gr,
        gw: gw,
        gx: gx,
        or: or,
        ow: ow,
        ox: ox,    
    };
}