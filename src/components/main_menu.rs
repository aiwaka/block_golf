use bevy::prelude::*;

use super::game::GameRule;

/// 各種オプションをゲームステートに受け渡すためのリソース
pub struct MenuSettings;

/// メニューとして表示される選択肢セットの識別番号
#[derive(Component)]
pub struct MenuLayer(pub u32);

// リソースとして使う？
pub struct MenuOptionSets {
    pub option_set: Vec<MenuOptionSet>,
    pub current_layer: u32,
}

pub struct MenuOptionSet {
    pub options: Vec<MenuOption>,
    pub layer_num: u32,
}

// TODO: enumをこれに変換するマクロとかあったらよさそう
pub struct MenuOption {
    pub name: &'static str,
    pub disabled: bool,
}
impl MenuOption {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            disabled: false,
        }
    }
}

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

impl From<GameRule> for String {
    fn from(r: GameRule) -> Self {
        match r {
            GameRule::BallScore => "BallScore".to_string(),
            GameRule::LittleOperation => "LittleOperation".to_string(),
            GameRule::TimeAttack => "TimeAttack".to_string(),
        }
    }
}

/// 選択肢のテキストであることを表す.
#[derive(Component)]
pub struct OptionText;
/// 選択中オプション
#[derive(Component)]
pub struct CurrentOption;
#[derive(Component)]
pub struct GameRuleOption(pub GameRule);
