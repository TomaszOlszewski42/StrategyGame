use bevy::{color::palettes::css::{GOLD, GRAY, GREY, RED, SLATE_GREY, WHITE}, prelude::*};

use crate::GameState;


#[derive(Component)]
pub struct FlagUi;

#[derive(Component)]
pub struct PlayerCountryTab;

#[derive(Component)]
pub struct TreasureText {
    pub gold_owner: Option<Entity>,
}

#[derive(Component)]
pub struct UpRecruitsButton;

#[derive(Component)]
pub struct ReduceRecruitsButton;

#[derive(Component)]
pub struct RecruitmentCostText;

#[derive(Component)]
pub struct RecrutimentNumberText;

#[derive(Component)] 
pub struct RecruitButton;

#[derive(Component)]
pub struct BuildMarketButton;

#[derive(Component)]
pub struct EnemyCountryUI;

#[derive(Component)]
pub struct DeclareWarButton;

#[derive(Component)]
pub struct ProposePeaceButton;

fn recruit_button_bundle(arrow_font: &Handle<Font>) -> impl Bundle {
    (
        DespawnOnExit(GameState::Game),
        Button,
        RecruitButton,
        Node {
            width: Val::Px(179.0),
            height: Val::Px(75.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            .. default()
        },
        BackgroundColor(Color::BLACK),
        children![(
            DespawnOnExit(GameState::Game),
            Text::new("Recruit"),
            TextFont {
                font: arrow_font.clone(),
                font_size: 20.0,
                ..default()
            }
        )]
    )
}

fn recruit_options_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Px(179.0),
        height: Val::Px(75.0),
        align_items: AlignItems::Center,
        margin: UiRect {left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(5.0), bottom: Val::Px(0.0) },
        .. default()
    },
    children![arrows(arrow_font), recruit_tab_numbers(arrow_font)])
}

fn recruit_tab_numbers(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        height: Val::Percent(100.0),
        width: Val::Px(155.0),
        justify_self: JustifySelf::Stretch,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        .. default()
    },
    children![soldiers_num(arrow_font), recruit_cost(arrow_font)],
)}

fn soldiers_num(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(70.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    BackgroundColor(Color::Srgba(SLATE_GREY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("+0"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 16.0,
            ..default()
        },
        RecrutimentNumberText,
    )],
)}

fn recruit_cost(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(30.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    BackgroundColor(Color::Srgba(GOLD)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("-0G"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 16.0,
            ..default()
        },
        RecruitmentCostText,
    )],
)} 

fn arrows(arrow_font: &Handle<Font>) -> impl Bundle {
    (
        DespawnOnExit(GameState::Game),
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            width: Val::Px(25.0),
            height: Val::Percent(100.0),
            .. default()
        },
        children![up_arrow(arrow_font), down_arrow(arrow_font)]   
    )
}

fn up_arrow(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        height: Val::Percent(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    UpRecruitsButton,
    Button,
    BackgroundColor(Color::Srgba(GREY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("↑"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 20.0,
            ..default()
        }
    )]
)
}

fn down_arrow(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        height: Val::Percent(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    ReduceRecruitsButton,
    Button,
    BackgroundColor(Color::Srgba(GREY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("↓"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 20.0,
            ..default()
        }
    )]
)}

fn build_market_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(15.0), bottom: Val::Px(0.0) },
        .. default()
    },
    children![build_market_text_bundle(arrow_font), build_market_button_bundle(arrow_font)],
)}

fn build_market_text_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        height: Val::Px(35.0),
        width: Val::Percent(65.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    BackgroundColor(Color::Srgba(SLATE_GREY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Market (-90G)"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 16.0,
            ..default()
        },
    )]
)} 

fn build_market_button_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        height: Val::Px(35.0),
        width: Val::Percent(30.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    BackgroundColor(Color::BLACK),
    Button,
    BuildMarketButton,
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Build"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 16.0,
            ..default()
        },
    )],
)}

fn player_province_tab_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(15.0), bottom: Val::Px(0.0) },
        flex_direction: FlexDirection::Column,
        .. default()
    },
    children![
        recruit_button_bundle(arrow_font), 
        recruit_options_bundle(arrow_font),
        build_market_bundle(arrow_font),
    ],
)}

fn flag_bundle() -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    ImageNode {
        .. default()
    },
    Node {
        width: Val::Px(150.0),
        height: Val::Px(100.0),
        margin: UiRect { left: Val::Px(15.0), right: Val::Px(25.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
        .. default()
    },
    FlagUi,
)} 

fn gold_amount_bundle() -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(15.0), bottom: Val::Px(0.0) },
        .. default()
    },
    Text::new("Gold: "),
    TreasureText {gold_owner: None},
)}

fn war_button_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Px(179.0),
        height: Val::Px(50.0),
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(15.0), bottom: Val::Px(0.0) },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    children![war_button_button_bundle(arrow_font)],
)}

fn war_button_button_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Px(179.0),
        height: Val::Px(50.0),
        ..default()
    },
    Visibility::Inherited,
    BackgroundColor(Color::Srgba(RED)),
    DeclareWarButton,
    Button,
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Declare war"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 20.0,
            ..default()
        },
    )],
)}

fn peace_button_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Px(179.0),
        height: Val::Px(50.0),
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(15.0), bottom: Val::Px(0.0) },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    children![peace_button_button_bundle(arrow_font)],
)}

fn peace_button_button_bundle(arrow_font: &Handle<Font>) -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Px(179.0),
        height: Val::Px(50.0),
        ..default()
    },
    Visibility::Inherited,
    BackgroundColor(Color::Srgba(WHITE)),
    ProposePeaceButton,
    Button,
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Propose peace"),
        TextFont {
            font: arrow_font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::Srgba(GRAY)),
    )],
)}

fn container_node() -> Node {
    Node {
        width: Val::Px(230.0),
        height: Val::Percent(90.0),
        justify_content: JustifyContent::Center,
        padding: UiRect { left: Val::Px(5.0), right: Val::Px(5.0), top: Val::Percent(2.0), bottom: Val::Px(0.0) },
        .. default()
    }
}

fn tab_bundle() -> impl Bundle {
    (
        DespawnOnExit(GameState::Game),
        BackgroundColor(Color::srgb(0.2, 0.12, 0.06)),
        Node {
            width: Val::Px(200.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect { left: Val::Px(10.0), right: Val::Px(0.0), top: Val::Px(20.0), bottom: Val::Px(0.0) },
            .. default() 
        }
    )
}

#[derive(Component)]
pub struct EnemyFlag(pub usize);

pub fn war_list_bundle() -> impl Bundle {(
    Node {
        width: Val::Percent(90.0),
        height: Val::Px(40.0),
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        .. default()
    },
    children![(
        Text::new("War:")), (
        Node {
            width: Val::Px(45.0),
            height: Val::Px(30.0),
            .. default()
        },
        ImageNode { .. default() },
        EnemyFlag(0),
    ), (
        Node {
            width: Val::Px(45.0),
            height: Val::Px(30.0),
            margin: UiRect { left: Val::Px(5.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
            .. default()
        },
        ImageNode {
            .. default()
        },
        EnemyFlag(1),
    )]
)}

pub fn spawn_player_country_tabs(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    let container = container_node();
    let tab = tab_bundle();

    let arrow_font: Handle<Font> = asset_server.load("Montserrat-Regular.ttf");

    commands.spawn((DespawnOnExit(GameState::Game), container, PlayerCountryTab, Visibility::Hidden))
        .with_children(|parent|{ 
            parent.spawn(tab).with_children(|parent| {
                parent.spawn(flag_bundle());
                parent.spawn(war_list_bundle());
                parent.spawn(gold_amount_bundle());
                parent.spawn(player_province_tab_bundle(&arrow_font));
            });
        });

    commands.spawn((DespawnOnExit(GameState::Game), container_node(), EnemyCountryUI, Visibility::Hidden, 
        children![(
            tab_bundle(), children![
                flag_bundle(),
                war_list_bundle(),
                gold_amount_bundle(),
                war_button_bundle(&arrow_font),
                peace_button_bundle(&arrow_font),
            ]
        )]
    ));
}
