use bevy::prelude::*;

use crate::assets::ImageAssets;

use crate::state::GameState;
use crate::state::MenuState;

#[derive(Resource)]
pub struct CutsceneSequence {
    scene: usize,
    images: Vec<Handle<Image>>,
    presenting: Option<Entity>,
}

impl FromWorld for CutsceneSequence {
    fn from_world(world: &mut World) -> Self {
        let images = world.resource::<ImageAssets>();

        Self {
            scene: 0,
            images: vec![
                images.cutscene_1.clone(),
                images.cutscene_2.clone(),
                images.cutscene_3.clone(),
                images.cutscene_4.clone(),
                images.cutscene_5.clone(),
                images.cutscene_6.clone(),
                images.cutscene_7.clone(),
                images.cutscene_8.clone(),
                images.cutscene_9.clone(),
                images.cutscene_10.clone(),
                images.cutscene_11.clone(),
                images.cutscene_12.clone(),
                images.tutorial.clone(),
                images.readying.clone(),
            ],
            presenting: None, 
        }
    }
}

pub fn cutscene_play(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut gstate: ResMut<NextState<GameState>>,
    mut mstate: ResMut<NextState<MenuState>>,
    mut cutscene: ResMut<CutsceneSequence>
) {
    if cutscene.images.is_empty() {
        return;
    }

    if mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        cutscene.scene += 1;

        if let Some(entity) = cutscene.presenting {
            commands.entity(entity).despawn();
            cutscene.presenting = None;
        }
    }

    if cutscene.scene > cutscene.images.len() - 1 {
        gstate.set(GameState::GameSetup);
        mstate.set(MenuState::CutsceneEnded);
        return;
    }

    if cutscene.presenting.is_none() {
        cutscene.presenting = Some(commands.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|parent| {
            parent.spawn((
                ImageNode::new(cutscene.images[cutscene.scene].clone()),
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    max_width: Val::Percent(100.0),
                    max_height: Val::Percent(100.0),
                    ..default()
                },
            ));
        }).id());
    }
}
