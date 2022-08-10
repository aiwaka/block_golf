//! タイトルメニューの構造定義を行うファイル

use crate::components::main_menu::menu::{MenuOption, MenuOptionSet, MenuOptionSets};

pub fn menu_options_settings() -> MenuOptionSets {
    let main_option = MenuOptionSet {
        options: vec![
            MenuOption::new("Start"),
            MenuOption::new("Set Rule"),
            MenuOption::new("Exit"),
        ],
        layer_num: 0,
    };
    let stage_option = MenuOptionSet {
        options: vec![MenuOption::new("0")],
        layer_num: 1,
    };
    let set_rule_option = MenuOptionSet {
        options: vec![
            MenuOption::new("BallScore"),
            MenuOption::new("LittleOperation"),
            MenuOption::new("TimeAttack"),
        ],
        layer_num: 2,
    };
    MenuOptionSets {
        option_set: vec![main_option, stage_option, set_rule_option],
    }
}
