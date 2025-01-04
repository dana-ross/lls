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