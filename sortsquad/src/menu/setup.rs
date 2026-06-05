use bevy::prelude::*;

use crate::state::GameState;
use crate::state::MenuState;

use crate::assets::ImageAssets;

use crate::menu::titlescreen::StartButton;

use crate::menu::cutscene::CutsceneSequence;

#[derive(Component)]
pub struct MenuEntity;

pub fn setup_menu(
    mut commands: Commands,
    mut gstate: ResMut<NextState<GameState>>,
    mut mstate: ResMut<NextState<MenuState>>,
    assets: Res<ImageAssets>
) {
    commands.init_resource::<CutsceneSequence>();

    commands.spawn((
        MenuEntity,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ImageNode {
            image: assets.titlescree.clone(),
            image_mode: NodeImageMode::Stretch,
            ..default()
        },
    ));

    commands.spawn((
        MenuEntity,
        GlobalZIndex(1),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..default()
        }
    ))
    .with_children(|parent| {
        parent.spawn((
            Button,
            StartButton,
            ImageNode {
                image: assets.buttonnormal.clone(),
                ..default()
            },
            Node {
                width: Val::Px(300.0),
                height: Val::Px(150.0),
                margin: UiRect::bottom(Val::Vh(10.0)),
                ..default()
            },
        ));
    });

    gstate.set(GameState::Menu);
    mstate.set(MenuState::Titlescreen);
}

pub fn desetup_menu(
    mut commands: Commands,
    elements: Query<Entity, With<MenuEntity>>
) {
    for entity in &elements {
        commands.entity(entity).despawn();
    }
}
