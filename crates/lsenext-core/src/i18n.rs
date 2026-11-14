use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct Strings {
    pub pick_source: String,
    pub drop_symlink: String,
    pub drop_relative_symlink: String,
    pub drop_junction: String,
    pub drop_hardlink: String,
    pub create_symlink: String,
    pub create_relative_symlink: String,
    pub create_junction: String,
    pub create_hardlink: String,
    pub clear_source: String,
    pub diagnostics: String,
    pub about_description: String,
    pub developer_mode_enabled: String,
    pub developer_mode_disabled: String,
    pub developer_mode_question: String,
    pub developer_mode_prompt: String,
    pub source_required: String,
    pub destination_exists: String,
    pub junction_dirs_only: String,
}

static STRINGS: OnceLock<Strings> = OnceLock::new();

#[cfg(windows)]
fn is_chinese_ui() -> bool {
    use windows_sys::Win32::Globalization::GetUserDefaultUILanguage;
    let lang_id = unsafe { GetUserDefaultUILanguage() };
    (lang_id & 0x00FF) == 0x04
}

#[cfg(not(windows))]
fn is_chinese_ui() -> bool {
    false
}

pub fn strings() -> &'static Strings {
    STRINGS.get_or_init(|| {
        let json = if is_chinese_ui() {
            include_str!("../../../resources/strings/zh-CN.json")
        } else {
            include_str!("../../../resources/strings/en-US.json")
        };

        serde_json::from_str(json).unwrap_or_else(|_| {
            serde_json::from_str(include_str!("../../../resources/strings/en-US.json"))
                .expect("English localization JSON must be valid")
        })
    })
}