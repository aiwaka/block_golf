use std::collections::HashMap;

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::components::game::GameRule;
use crate::components::main_menu::menu::{
    ChangeMenuLayerEvent, CurrentOption, MenuLayerOptionEntities, MenuLayerPos, MenuOptionResource,
    OptionText,
};
use crate::stages::select_stage;
use crate::{AppState, SCREEN_HEIGHT, SCREEN_WIDTH};

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

fn text_style_from_pos(left: f32, top: f32) -> Style {
    Style {
        position_type: PositionType::Absolute,
        position: Rect {
            left: Val::Px(left),
            top: Val::Px(top),
            ..Default::default()
        },
        ..default()
    }
}

/// メニューの初期化
fn init_option2(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let menu = menu_options_settings();
    // レイヤーにおける設定保存用ハッシュマップ. 最初にすべて0で初期化しておく
    let mut layer_choice_table = HashMap::<u32, u32>::new();
    // レイヤー内のエンティティ保存用ハッシュマップ. 最初にすべて空ベクトルで初期化しておく
    let mut layer_option_entities = HashMap::<u32, Vec<Entity>>::new();
    for option_set in menu.option_set.iter() {
        layer_option_entities.insert(option_set.layer_id, vec![]);
    }
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };
    for option_set in menu.option_set.iter() {
        // テキストの配置位置を決定するための変数
        let mut text_width_sum = SCREEN_WIDTH * 0.1;
        let mut text_height_sum = SCREEN_HEIGHT * 0.2;
        for option in option_set.options.iter() {
            let mut text_bundle = TextBundle {
                text: Text::with_section(
                    option.name,
                    text_style.clone(),
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            };
            let text_width = 30.0 * option.name.len() as f32;

            if text_width_sum + text_width > SCREEN_WIDTH * 0.8 {
                // 次を置いたら画面外に出てしまうなら更新してからスタイルを設定
                text_width_sum = SCREEN_WIDTH * 0.1;
                text_height_sum += 50.0;
            }
            text_bundle.style = text_style_from_pos(text_width_sum, text_height_sum);
            text_width_sum += text_width;

            let ent = commands
                .spawn_bundle(text_bundle)
                .insert(Visibility { is_visible: false })
                .insert(MenuLayerPos(option_set.layer_id))
                .insert(OptionText)
                .id();
            // レイヤー番号のところの配列にエンティティを追加
            layer_option_entities
                .get_mut(&option_set.layer_id)
                .unwrap()
                .push(ent);
            layer_choice_table.insert(option_set.layer_id, 0u32);
        }
    }
    commands.insert_resource(MenuOptionResource {
        current_layer: 0,
        layer_choice_table,
        ..default()
    });
    commands
        .entity(layer_option_entities[&0u32][0])
        .insert(CurrentOption);
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
        menu_res.current_option_num = (prev_num + option_num - 1) % option_num;
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
    current_option_query: Query<Entity, With<CurrentOption>>,
    mut menu_res: ResMut<MenuOptionResource>,
    layer_option_entities: Res<MenuLayerOptionEntities>,
) {
    for ev in event_reader.iter() {
        // 新旧のレイヤーを取得
        let current_layer = menu_res.current_layer;
        let next_layer = ev.0;
        // 新レイヤーのエンティティのセットを取得
        let option_entities = &layer_option_entities.0[&next_layer];
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
        for ent in current_option_query.iter() {
            commands.entity(ent).remove::<CurrentOption>();
        }
        // 更新処理
        menu_res.current_layer = next_layer;
        menu_res.current_option_num = next_init_option;
        // ここでCurrentOptionを挿入
        commands
            .entity(option_entities[next_init_option as usize])
            .insert(CurrentOption);
    }
}

/// ひとつ上の階層に戻る
fn back_to_upper_layer(
    event_writer: &mut EventWriter<ChangeMenuLayerEvent>,
    menu_res: &mut ResMut<MenuOptionResource>,
) {
    // popできたなら移動処理, できなければ無視でOK
    if let Some(last_layer) = menu_res.layer_stack.pop() {
        event_writer.send(ChangeMenuLayerEvent(last_layer, false));
    }
}

/// キャンセルコマンド処理. Xキーでひとつ上の階層に戻る
fn cancel_select_option(
    key_in: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<ChangeMenuLayerEvent>,
    mut menu_res: ResMut<MenuOptionResource>,
) {
    if key_in.just_pressed(KeyCode::X) {
        back_to_upper_layer(&mut event_writer, &mut menu_res)
    }
}

/// 決定キーが押されたときのレイヤーと選択肢位置から処理を行う
fn each_option_processing(
    mut commands: Commands,
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
                0 => event_writer.send(ChangeMenuLayerEvent::move_to(1)),
                1 => event_writer.send(ChangeMenuLayerEvent::move_to(2)),
                2 => {
                    app_exit_events.send_default();
                }
                _ => {}
            },
            1 => {
                let stage_idx = pos as usize;
                let stage_info = select_stage(stage_idx);
                // ステージ情報をリソースとして挟む
                commands.insert_resource(stage_info);
                // ゲームルールを追加
                let rule_num = menu_res.layer_choice_table[&2];
                commands.insert_resource(GameRule::from(rule_num));
                app_state.set(AppState::Loading).unwrap();
            }
            _ => {}
        }
    }
}

// NOTE: 0.8でVisibilityの意味が変化したようです
/// 現在のレイヤーの選択肢を表示させる.
fn show_current_layer(
    mut query: Query<(&mut Visibility, &MenuLayerPos)>,
    menu_res: Res<MenuOptionResource>,
) {
    for (mut visibility, layer_pos) in query.iter_mut() {
        visibility.is_visible = layer_pos.0 == menu_res.current_layer;
    }
}

/// 現在選択されているオプションにのみ緑色を表示しほかは白色に戻す処理
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
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(cancel_select_option));
        app.add_system_set(
            SystemSet::on_update(AppState::Menu).with_system(each_option_processing),
        );
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(show_current_layer));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(text_color));
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(deconstruct_menu.label("deconstruct")),
        );
    }
}
