use bevy::prelude::*;

use crate::assets::FontAssets;

use crate::game::score::Score;

pub fn setup_end(
    mut commands: Commands,
    assets: Res<FontAssets>,
    score: Res<Score>,
    mut color: ResMut<ClearColor>,
) {
    color.0 = Color::from(Srgba::hex("#242424").unwrap());

    commands.spawn(
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        }
    )
    .with_children(|parent| {
        parent.spawn((
            Node {
                margin: UiRect::top(Val::Px(10.)),
                ..Default::default()
            },
            Text::new("FINAL SCORE!"),
            TextFont {
                font: assets.font.clone(),
                font_size: 64.,
                ..Default::default()
            },
            TextColor::from(Color::from(Srgba::hex("#e1d600").unwrap())),
            TextLayout::new_with_justify(Justify::Center),
        ));
        parent.spawn((
            Text::new(format!("{}", score.0)),
            TextFont {
                font: assets.font.clone(),
                font_size: 128.,
                ..Default::default()
            },
            TextLayout::new_with_justify(Justify::Center),
        ));
    });
}
