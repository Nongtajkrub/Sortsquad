use bevy::prelude::*;
use bevy::camera::ScalingMode;

use crate::powerup::PowerupKind;
use crate::powerup::PowerupText;
use crate::state::RoundState;
use crate::util::achor::SpriteAchorBottom;

use crate::state::GameState;
use crate::assets::ImageAssets;
use crate::assets::FontAssets;

use crate::player::PlayerControlLabel;
use crate::player::PlayerControl;
use crate::player::PlayerBundle;
use crate::player::Player;

use crate::trashes::TrashImages;
use crate::trashes::TrashKind;

use crate::column::Column;

use crate::score::ScoreText;

pub const VIEW_PORT_WIDTH: f32 = 1000.;

pub fn setup_game(
    mut commands: Commands,
    images: Res<ImageAssets>,
    fonts: Res<FontAssets>,
    mut gstate: ResMut<NextState<GameState>>,
    mut rstate: ResMut<NextState<RoundState>>
) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal { 
                viewport_width: VIEW_PORT_WIDTH 
            }, 
            ..OrthographicProjection::default_2d()
        })
    ));

    // Spawn player entities
    let general_player_id = commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::General,
            col: Column::new(0),
            control: PlayerControl {
                left: KeyCode::KeyA,
                right: KeyCode::KeyD,
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0., 0.)),
                image: images.bin_general.clone(),
                ..Default::default()
            },
            achor: SpriteAchorBottom,
        }
    ).id();
    commands.spawn((
        PlayerControlLabel(general_player_id),
        Column::with_size_factor(0, 0.7),
        Sprite::from_image(images.control_a_d.clone()),
        Transform::from_xyz(0., 0., 1.),
    ));

    let recycle_player_id = commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Recycle,
            col: Column::new(1),
            control: PlayerControl {
                left: KeyCode::KeyG,
                right: KeyCode::KeyH,
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0., 0.)),
                image: images.bin_recycle.clone(),
                ..Default::default()
            },
            achor: SpriteAchorBottom,
        }
    ).id();
    commands.spawn((
        PlayerControlLabel(recycle_player_id),
        Column::with_size_factor(1, 0.7),
        Sprite::from_image(images.control_g_h.clone()),
        Transform::from_xyz(0., 0., 1.),
    ));

    let organic_player_id = commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Organic,
            col: Column::new(2),
            control: PlayerControl {
                left: KeyCode::ArrowLeft,
                right: KeyCode::ArrowRight,
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0., 0.)),
                image: images.bin_organic.clone(),
                ..Default::default()
            },
            achor: SpriteAchorBottom,
        }
    ).id();
    commands.spawn((
        PlayerControlLabel(organic_player_id),
        Column::with_size_factor(2, 0.7),
        Sprite::from_image(images.control_al_ar.clone()),
        Transform::from_xyz(0., 0., 1.),
    ));

    let hazardous_player_id = commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Hazardous,
            col: Column::new(3),
            control: PlayerControl {
                left: KeyCode::BracketLeft,
                right: KeyCode::BracketRight,
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0., 0.)),
                image: images.bin_hazardous.clone(),
                ..Default::default()
            },
            achor: SpriteAchorBottom,
        }
    ).id();
    commands.spawn((
        PlayerControlLabel(hazardous_player_id),
        Column::with_size_factor(2, 0.7),
        Sprite::from_image(images.control_bl_br.clone()),
        Transform::from_xyz(0., 0., 1.),
    ));

    // Initialize UI elements.
    commands.spawn(Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn((
            Node {
                margin: UiRect::top(Val::Px(10.)),
                ..Default::default()
            },
            ScoreText,
            Text::new('0'),
            TextFont {
                font: fonts.font.clone(),
                font_size: 128.,
                ..Default::default()
            },
            TextLayout::new_with_justify(Justify::Center),
        ));
        parent.spawn((
            PowerupText,
            Text::new(PowerupKind::NoPowerup.to_text()),
            TextFont {
                font: fonts.font.clone(),
                font_size: 32.,
                ..Default::default()
            },
            TextLayout::new_with_justify(Justify::Center),
        ));
    });

    // Initialize trash images resource.
    commands.insert_resource(TrashImages {
        general: vec![
            images.trash_ciggarette.clone(),
            images.trash_shoe.clone(),
            images.trash_tissue.clone(),
        ],
        recycle: vec![
            images.trash_coke.clone(),
            images.trash_newspaper.clone(),
            images.trash_tissue.clone(),
        ],
        organic: vec![
            images.trash_apple.clone(),
            images.trash_fishbone.clone(),
            images.trash_vegatable.clone(),
        ],
        hazardous: vec![
            images.trash_battery.clone(),
            images.trash_bleach.clone(),
            images.trash_electronic.clone(),
        ],
    });

    gstate.set(GameState::Playing);
    rstate.set(RoundState::RoundStarting);
}
