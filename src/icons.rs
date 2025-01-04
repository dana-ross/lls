#[derive(Eq, Hash, PartialEq)]
pub enum IconType {
    None,
    Directory,
    Executable,
    NoWrite,
    NoAccess,
}

pub fn get_icon_by_type(icon_type: IconType) -> String {
    return match icon_type {
        IconType::None => "",
        IconType::Directory => "📁",
        IconType::Executable => "⚙️ ",
        IconType::NoWrite => "🔏",
        IconType::NoAccess => "🚫"
    }.to_string();
}