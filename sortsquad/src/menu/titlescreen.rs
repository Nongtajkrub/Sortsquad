use bevy::prelude::*;

use crate::state::MenuState;

use crate::assets::ImageAssets;

// Start button marker.
#[derive(Component)]
pub struct StartButton;

pub fn titlescreen_button(
    assets: Res<ImageAssets>,
    mut state: ResMut<NextState<MenuState>>,
    mut buttons: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<Button>, With<StartButton>)
    >
) {
    for (interaction, mut image) in &mut buttons {
        match *interaction {
            Interaction::Hovered => { 
                image.image = assets.buttonhover.clone(); 
            },
            Interaction::Pressed => {
                state.set(MenuState::Cutscene);
            },
            Interaction::None => {
                image.image = assets.buttonnormal.clone(); 
            },
        };
    }
}
