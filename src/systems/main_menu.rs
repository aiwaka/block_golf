use bevy::app::AppExit;
use bevy::prelude::*;

use crate::components::game::GameRule;
use crate::components::main_menu::{CurrentOption, GameRuleOption, MenuOptions, OptionText};
use crate::AppState;

/// これらはMenu状態におけるリソースとして使用する
struct MenuOptionEntities(Vec<Entity>);
struct GameRuleOptionEntities(Vec<Entity>);
struct ResidentEntities(Vec<Entity>);

fn init_option(mut commands: Commands, entities: Query<Entity>, asset_server: Res<AssetServer>) {
    // 最初に存在しているentityをすべて保存しておく.
    commands.insert_resource(ResidentEntities(entities.iter().collect::<Vec<Entity>>()));
    // let menu_options = vec!["Start", "Set Rule", "Exit"];
    let mut option_entities = Vec::<Entity>::new();
    let mut game_rule_entities = Vec::<Entity>::new();
    for (idx, opt) in MenuOptions::iterate().enumerate() {
        let ent = commands
            .spawn_bundle(TextBundle {
                style: Style {
                    // align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(idx as f32 * 40.0 + 100.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    opt,
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
            .insert(OptionText)
            .insert(opt)
            .id();
        option_entities.push(ent);
    }
    let game_rule_str_vec = vec![
        GameRule::BallScore,
        GameRule::LittleOperation,
        GameRule::TimeAttack,
    ];
    for (idx, opt) in game_rule_str_vec.into_iter().enumerate() {
        let ent = commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(145.0),
                        left: Val::Px(idx as f32 * 200.0 + 200.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    opt,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                    TextAlignment::default(),
                ),
                ..default()
            })
            .insert(OptionText)
            .insert(GameRuleOption(opt))
            .id();
        game_rule_entities.push(ent);
    }
    commands.entity(option_entities[0]).insert(CurrentOption);
    commands.entity(game_rule_entities[0]).insert(CurrentOption);
    commands.insert_resource(MenuOptionEntities(option_entities));
    commands.insert_resource(GameRuleOptionEntities(game_rule_entities));
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

fn move_cursor(
    mut commands: Commands,
    key_in: Res<Input<KeyCode>>,
    option_entities: Res<MenuOptionEntities>,
    mut current_option_num: Local<usize>,
) {
    let prev_num = *current_option_num;
    if key_in.just_pressed(KeyCode::Up) {
        *current_option_num = (*current_option_num + 2) % 3;
    } else if key_in.just_pressed(KeyCode::Down) {
        *current_option_num = (*current_option_num + 1) % 3;
    }
    if prev_num != *current_option_num {
        let prev_ent = option_entities.0[prev_num];
        let current_ent = option_entities.0[*current_option_num];
        commands.entity(prev_ent).remove::<CurrentOption>();
        commands.entity(current_ent).insert(CurrentOption);
    }
}

fn move_game_rule_cursor(
    mut commands: Commands,
    key_in: Res<Input<KeyCode>>,
    option_query: Query<&MenuOptions, With<CurrentOption>>,
    game_rule_option_entities: Res<GameRuleOptionEntities>,
    mut current_option_num: Local<usize>,
) {
    if let &MenuOptions::SetRule = option_query.single() {
        let prev_num = *current_option_num;
        if key_in.just_pressed(KeyCode::Left) {
            *current_option_num = (*current_option_num + 2) % 3;
        } else if key_in.just_pressed(KeyCode::Right) {
            *current_option_num = (*current_option_num + 1) % 3;
        }
        if prev_num != *current_option_num {
            let prev_ent = game_rule_option_entities.0[prev_num];
            let current_ent = game_rule_option_entities.0[*current_option_num];
            commands.entity(prev_ent).remove::<CurrentOption>();
            commands.entity(current_ent).insert(CurrentOption);
        }
    }
}

fn select_option(
    mut app_state: ResMut<State<AppState>>,
    key_in: Res<Input<KeyCode>>,
    current_query: Query<&MenuOptions, With<CurrentOption>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if key_in.just_pressed(KeyCode::Z) {
        let opt = current_query.single();
        match *opt {
            MenuOptions::Start => {
                app_state.set(AppState::Game).unwrap();
            }
            MenuOptions::SetRule => {}
            MenuOptions::Exit => {
                app_exit_events.send_default();
            }
        }
    }
}

/// ゲームルールをグローバルリソースとしてセット
fn set_game_rule(
    mut commands: Commands,
    option_query: Query<&GameRuleOption, With<CurrentOption>>,
) {
    let rule = option_query.single();
    commands.insert_resource(rule.0);
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
    commands.remove_resource::<MenuOptionEntities>();
    commands.remove_resource::<GameRuleOptionEntities>();
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(init_option));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(move_cursor));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(move_game_rule_cursor));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(text_color));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(select_option));
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(set_game_rule.before("deconstruct")),
        );
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(deconstruct_menu.label("deconstruct")),
        );
    }
}
