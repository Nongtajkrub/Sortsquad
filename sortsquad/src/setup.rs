use bevy::prelude::*;

use crate::util::align::Align;

use crate::assets::GameState;
use crate::assets::ImageAssets;
use crate::assets::FontAssets;

use crate::player::PlayerControlLabel;
use crate::player::PlayerControl;
use crate::player::PlayerBundle;
use crate::player::Player;

use crate::trash::TrashImages;
use crate::trash::TrashKind;

use crate::column::Column;
use crate::column::ColumnResyncEvent;

use crate::score::ScoreText;

pub fn setup_game(
    mut commands: Commands,
    images: Res<ImageAssets>,
    fonts: Res<FontAssets>,
    mut state: ResMut<NextState<GameState>>
) {
    commands.spawn(Camera2d);

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
            sprite: Sprite::from_image(images.bin_general.clone()),
            align: Align::Bottom,
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
            sprite: Sprite::from_image(images.bin_recycle.clone()),
            align: Align::Bottom,
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
            sprite: Sprite::from_image(images.bin_organic.clone()),
            align: Align::Bottom,
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
            sprite: Sprite::from_image(images.bin_hazardous.clone()),
            align: Align::Bottom,
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
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexStart,
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn((
            Node {
                margin: UiRect::all(Val::Px(20.)),
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

    commands.trigger(ColumnResyncEvent);

    state.set(GameState::Playing);
}
