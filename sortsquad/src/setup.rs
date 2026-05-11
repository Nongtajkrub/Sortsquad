use bevy::prelude::*;

use crate::assets::GameState;
use crate::util::align::Align;

use crate::assets::ImageAssets;

use crate::player::PlayerControl;
use crate::player::PlayerBundle;
use crate::player::Player;

use crate::trash::TrashImages;
use crate::trash::TrashKind;
use crate::trash::TrashYPos;

use crate::column::Column;
use crate::column::ColumnResyncEvent;

use crate::score::Score;

pub fn setup(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut state: ResMut<NextState<GameState>>
) {
    commands.spawn(Camera2d);

    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::General,
            col: Column::new(0),
            control: PlayerControl {
                left: KeyCode::KeyA,
                right: KeyCode::KeyD,
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.bin_general.clone()),
            align: Align::Bottom,
        }
    );
    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Recycle,
            col: Column::new(1),
            control: PlayerControl {
                left: KeyCode::KeyG,
                right: KeyCode::KeyH
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.bin_recycle.clone()),
            align: Align::Bottom,
        }
    );
    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Organic,
            col: Column::new(2),
            control: PlayerControl {
                left: KeyCode::ArrowLeft,
                right: KeyCode::ArrowRight
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.bin_organic.clone()),
            align: Align::Bottom,
        }
    );
    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Hazardous,
            col: Column::new(3),
            control: PlayerControl {
                left: KeyCode::BracketLeft,
                right: KeyCode::BracketRight
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.bin_hazardous.clone()),
            align: Align::Bottom,
        }
    );

    println!("General: A, D");
    println!("Recycle: G, H");
    println!("Organic: <-, ->");
    println!("Hazardous: [, ]");

    commands.insert_resource(TrashImages {
        general: vec![
            assets.trash_ciggarette.clone(),
            assets.trash_shoe.clone(),
            assets.trash_tissue.clone(),
        ],
        recycle: vec![
            assets.trash_coke.clone(),
            assets.trash_newspaper.clone(),
            assets.trash_tissue.clone(),
        ],
        organic: vec![
            assets.trash_apple.clone(),
            assets.trash_fishbone.clone(),
            assets.trash_vegatable.clone(),
        ],
        hazardous: vec![
            assets.trash_battery.clone(),
            assets.trash_bleach.clone(),
            assets.trash_electronic.clone(),
        ],
    });
    commands.insert_resource(TrashYPos(0.));
    commands.insert_resource(Score(0));

    commands.trigger(ColumnResyncEvent);

    state.set(GameState::Playing);
}
