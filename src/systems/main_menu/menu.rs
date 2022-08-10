use std::collections::HashMap;

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::components::game::GameRule;
use crate::components::main_menu::menu::{
    ChangeMenuLayerEvent, CurrentOption, MenuLayerOptionEntities, MenuLayerPos, MenuOptionResource,
    OptionText,
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

fn init_option2(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let menu = menu_options_settings();
    // let mut option_entities = Vec::<Entity>::new();
    let mut layer_option_entities = HashMap::<u32, Vec<Entity>>::new();
    for option_set in menu.option_set.iter() {
        for option in option_set.options.iter() {
            let ent = commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        // position_type: PositionType::Absolute,
                        ..default()
                    },
                    text: Text::with_section(
                        option.name,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..default()
                        },
                    ),
                    ..default()
                })
                .insert(Visibility { is_visible: false })
                .insert(MenuLayerPos(option_set.layer_num))
                .insert(OptionText)
                .id();
            // 初めて見る階層番号なら配列を作成, すでにあるならそこに追加
            if let Some(option_entities) = layer_option_entities.get_mut(&option_set.layer_num) {
                option_entities.push(ent);
            } else {
                layer_option_entities.insert(option_set.layer_num, vec![ent]);
            }
        }
    }
    commands.insert_resource(MenuOptionResource {
        current_layer: 0,
        ..default()
    });
    commands.insert_resource(MenuLayerOptionEntities(layer_option_entities));
}

/// 方向キーで選択肢を選ぶ処理
fn select_options(
    mut commands: Commands,
    key_in: Res<Input<KeyCode>>,
    layer_option_entities: Res<MenuLayerOptionEntities>,
    mut menu_res: ResMut<MenuOptionResource>,
) {
    let option_entities = &layer_option_entities.0[&menu_res.current_layer];
    let option_num = option_entities.len() as u32;

    let prev_num = menu_res.current_option_num;
    if key_in.just_pressed(KeyCode::Left) {
        menu_res.current_option_num = (prev_num + 2) % option_num;
    } else if key_in.just_pressed(KeyCode::Right) {
        menu_res.current_option_num = (prev_num + 1) % option_num;
    }
    // 変更があったときのみ処理
    if prev_num != menu_res.current_option_num {
        let next_num = menu_res.current_option_num;
        let prev_ent = option_entities[prev_num as usize];
        let current_ent = option_entities[next_num as usize];
        commands.entity(prev_ent).remove::<CurrentOption>();
        commands.entity(current_ent).insert(CurrentOption);
    }
}

/// メニュー階層変更イベントが送られた際の処理
/// OptionEntitiesの変更等
fn layer_changed(
    mut commands: Commands,
    mut event_reader: EventReader<ChangeMenuLayerEvent>,
    mut menu_res: ResMut<MenuOptionResource>,
    layer_option_entities: Res<MenuLayerOptionEntities>,
) {
    for ev in event_reader.iter() {
        let option_entities = &layer_option_entities.0[&menu_res.current_layer];
        let current_layer = menu_res.current_layer;
        let next_layer = ev.0;
        // 選択番号を保存する処理
        let current_option = menu_res.current_option_num;
        menu_res
            .layer_choice_table
            .insert(current_layer, current_option);
        // 階層スタックに追加
        menu_res.layer_stack.push(current_layer);
        // すでに選択番号が保存されていたら取得
        let next_init_option = if let Some(opt) = menu_res.layer_choice_table.get(&next_layer) {
            *opt
        } else {
            0
        };
        menu_res.current_layer = next_layer;
        menu_res.current_option_num = next_init_option;
        commands
            .entity(option_entities[next_init_option as usize])
            .insert(CurrentOption);
    }
}

/// キャンセルコマンド処理. Xキーでひとつ上の階層に戻る（選択肢は保存する）
fn back_to_upper_layer(
    key_in: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<ChangeMenuLayerEvent>,
    mut menu_res: ResMut<MenuOptionResource>,
) {
    if key_in.just_pressed(KeyCode::X) {
        if let Some(last_layer) = menu_res.layer_stack.pop() {
            event_writer.send(ChangeMenuLayerEvent(last_layer));
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
                0 => event_writer.send(ChangeMenuLayerEvent(1)),
                1 => event_writer.send(ChangeMenuLayerEvent(2)),
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

/// 現在のレイヤーの選択肢を表示させる. レイヤーは外部から変更すればよい
fn show_current_layer(
    mut query: Query<(&mut Visibility, &MenuLayerPos)>,
    menu_res: Res<MenuOptionResource>,
) {
    for (mut visibility, layer_pos) in query.iter_mut() {
        visibility.is_visible = layer_pos.0 == menu_res.current_layer;
    }
}

fn text_color(
    mut text_query: Query<&mut Text, (With<OptionText>, Without<CurrentOption>)>,
    mut current_query: Query<&mut Text, With<CurrentOption>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].style.color = Color::WHITE;
    }
    for mut text in current_query.iter_mut() {
        text.sections[0].style.color = Color::GREEN;
    }
}

// /// ゲームルールをグローバルリソースとしてセット
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
    commands.remove_resource::<MenuLayerOptionEntities>();
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
