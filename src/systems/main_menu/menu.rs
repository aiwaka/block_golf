use std::collections::HashMap;

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::components::game::GameRule;
use crate::components::main_menu::menu::{
    ChangeMenuLayerEvent, CurrentLayer, MenuLayerPos, MenuOptionResource, OptionText,
};
use crate::AppState;

use super::menu_contents::menu_options_settings;

/// メニュー状態に入ったときに存在したエンティティを保持しておく
/// （抜けるときにここに入っていないエンティティを削除する）
struct ResidentEntities(Vec<Entity>);

/// シーン進入時の初期化システム
fn init_menu_scene(
    mut commands: Commands,
    entities: Query<Entity>,
    asset_server: Res<AssetServer>,
) {
    // 最初に存在しているentityをすべて保存しておく.
    commands.insert_resource(ResidentEntities(entities.iter().collect::<Vec<Entity>>()));

    init_option2(&mut commands, &asset_server);
}

/// メニューの初期化
fn init_option2(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let menu = menu_options_settings();
    // レイヤーにおける設定保存用ハッシュマップ. 最初にすべて0で初期化しておく
    let mut layer_choice_table = HashMap::<u32, u32>::new();
    // レイヤー内のエンティティ保存用ハッシュマップ.
    let mut layer_option_entities = HashMap::<u32, Entity>::new();
    // レイヤーにおける選択肢数保存用のハッシュマップ.
    let mut layer_option_num = HashMap::<u32, u32>::new();
    for option_set in menu.option_set.iter() {
        let mut options_text_bundle = TextBundle {
            text: Text::default(),
            ..Default::default()
        };
        for option in option_set.options.iter() {
            options_text_bundle.text.sections.push(TextSection {
                value: option.name.to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            });
        }
        let ent = commands
            .spawn_bundle(options_text_bundle)
            // .insert(Visibility { is_visible: false })
            .insert(MenuLayerPos(option_set.layer_id))
            .insert(OptionText)
            .id();
        let layer_id = option_set.layer_id;
        layer_option_entities.insert(layer_id, ent);
        layer_option_num.insert(layer_id, option_set.options.len() as u32);
        layer_choice_table.insert(layer_id, 0u32);
    }
    commands.insert_resource(MenuOptionResource {
        current_layer: 0,
        layer_choice_table,
        layer_option_entities,
        layer_option_num,
        ..default()
    });
}

/// 方向キーで選択肢を選ぶ処理
fn select_options(
    key_in: Res<Input<KeyCode>>,
    // layer_option_entities: Res<MenuLayerOptionEntities>,
    mut menu_res: ResMut<MenuOptionResource>,
) {
    let option_num = menu_res.layer_option_num[&menu_res.current_layer];

    let prev_num = menu_res.current_option_num;
    if key_in.just_pressed(KeyCode::Left) {
        menu_res.current_option_num = (prev_num + 2) % option_num;
    } else if key_in.just_pressed(KeyCode::Right) {
        menu_res.current_option_num = (prev_num + 1) % option_num;
    }
}

/// メニュー階層変更イベントが送られた際の処理
/// OptionEntitiesの変更等
fn layer_changed(
    mut commands: Commands,
    mut event_reader: EventReader<ChangeMenuLayerEvent>,
    current_layer_query: Query<Entity, With<CurrentLayer>>,
    mut menu_res: ResMut<MenuOptionResource>,
    // layer_option_entities: Res<MenuLayerOptionEntities>,
) {
    for ev in event_reader.iter() {
        // 新旧のレイヤーを取得
        let current_layer = menu_res.current_layer;
        let next_layer = ev.0;
        // 現在の選択肢番号を保存する処理
        let current_option = menu_res.current_option_num;
        menu_res
            .layer_choice_table
            .insert(current_layer, current_option);
        // レイヤースタックに追加する処理
        if ev.1 {
            menu_res.layer_stack.push(current_layer);
        }
        // 新レイヤーの選択肢番号を取得
        let next_init_option = *menu_res.layer_choice_table.get(&next_layer).unwrap();
        // CurrentOptionコンポーネントを削除
        for ent in current_layer_query.iter() {
            commands.entity(ent).remove::<CurrentLayer>();
        }
        // 更新処理
        menu_res.current_layer = next_layer;
        menu_res.current_option_num = next_init_option;
        // 新レイヤーにCurrentOptionを挿入
        let layer_ent = menu_res.layer_option_entities[&next_layer];
        commands.entity(layer_ent).insert(CurrentLayer);
    }
}

/// キャンセルコマンド処理. Xキーでひとつ上の階層に戻る（選択肢は保存する）
fn back_to_upper_layer(
    key_in: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<ChangeMenuLayerEvent>,
    mut menu_res: ResMut<MenuOptionResource>,
) {
    if key_in.just_pressed(KeyCode::X) {
        // popできたなら移動処理, できなければ無視でOK
        if let Some(last_layer) = menu_res.layer_stack.pop() {
            event_writer.send(ChangeMenuLayerEvent(last_layer, false));
        }
    }
}

/// 決定キーが押されたときのレイヤーと選択肢位置から処理を行う
fn each_option_processing(
    mut app_state: ResMut<State<AppState>>,
    key_in: Res<Input<KeyCode>>,
    menu_res: Res<MenuOptionResource>,
    mut event_writer: EventWriter<ChangeMenuLayerEvent>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if key_in.just_pressed(KeyCode::Z) {
        let layer = menu_res.current_layer;
        let pos = menu_res.current_option_num;
        match layer {
            0 => match pos {
                0 => event_writer.send(ChangeMenuLayerEvent::moveTo(1)),
                1 => event_writer.send(ChangeMenuLayerEvent::moveTo(2)),
                2 => {
                    app_exit_events.send_default();
                }
                _ => {}
            },
            1 => {
                app_state.set(AppState::Game).unwrap();
                info!("{}", pos);
            }
            _ => {}
        }
    }
}

/// 現在のレイヤーの選択肢を表示させる.
fn show_current_layer(
    mut current_layer_query: Query<&mut Visibility, With<CurrentLayer>>,
    mut not_current_layer_query: Query<&mut Visibility, Without<CurrentLayer>>,
) {
    for mut visibility in current_layer_query.iter_mut() {
        visibility.is_visible = true;
    }
    for mut visibility in not_current_layer_query.iter_mut() {
        visibility.is_visible = false;
    }
}

/// 現在選択されているオプションにのみ緑色を表示しほかは白色に戻す処理
fn text_color(
    mut current_query: Query<&mut Text, With<CurrentLayer>>,
    menu_res: Res<MenuOptionResource>,
) {
    for mut text in current_query.iter_mut() {
        for opt_num in 0..menu_res.layer_option_num[&menu_res.current_layer] {
            text.sections[opt_num as usize].style.color = if opt_num == menu_res.current_option_num
            {
                Color::GREEN
            } else {
                Color::WHITE
            };
        }
    }
}

/// ゲームルールをグローバルリソースとしてセット
fn set_game_rule(mut commands: Commands, menu_res: Res<MenuOptionResource>) {
    let rule_num = if let Some(layer_choice) = menu_res.layer_choice_table.get(&1) {
        *layer_choice
    } else {
        0u32
    };
    commands.insert_resource(GameRule::from(rule_num));
}

/// Menu状態の初期からあったものを除いたすべてのEntityを削除する
fn deconstruct_menu(
    mut commands: Commands,
    entities: Query<Entity>,
    resident_entities: Res<ResidentEntities>,
) {
    for ent in entities.iter() {
        if !resident_entities.0.contains(&ent) {
            commands.entity(ent).despawn();
        }
    }
    commands.remove_resource::<MenuOptionResource>();
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(init_menu_scene));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(select_options));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(layer_changed));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(back_to_upper_layer));
        app.add_system_set(
            SystemSet::on_update(AppState::Menu).with_system(each_option_processing),
        );
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(show_current_layer));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(text_color));
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(set_game_rule.before("deconstruct")),
        );
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(deconstruct_menu.label("deconstruct")),
        );
    }
}
