use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub enum MenuOptions {
    Start,
    SetRule,
    Exit,
}
impl From<MenuOptions> for String {
    fn from(o: MenuOptions) -> Self {
        match o {
            MenuOptions::Start => "Start".to_string(),
            MenuOptions::SetRule => "Set Rule".to_string(),
            MenuOptions::Exit => "Exit".to_string(),
        }
    }
}
impl MenuOptions {
    pub fn iterate() -> std::vec::IntoIter<Self> {
        vec![MenuOptions::Start, MenuOptions::SetRule, MenuOptions::Exit].into_iter()
    }
}

/// 選択中オプション
#[derive(Component)]
pub struct CurrentOption;
