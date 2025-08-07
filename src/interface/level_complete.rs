use bevy::prelude::*;
use super::{GameState, LevelCompleteUI};
use crate::game_state::PlayerGameState;
use crate::game_state::orb::Orb;

#[derive(Component)]
pub struct GoToMarketButton;

pub fn setup_level_complete_ui(
    mut commands: Commands,
    player_state: Option<Res<PlayerGameState>>,
) {
    let (current_level, points_achieved) = if let Some(ref state) = player_state {
        (state.level(), state.points())
    } else {
        (1, 0)
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
        LevelCompleteUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ))
        .with_children(|content_parent| {
            content_parent.spawn((
                Text::new(format!("LEVEL {} COMPLETE!", current_level)),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 0.2)),
            ));

            content_parent.spawn((
                Text::new(format!("Points Achieved: {}", points_achieved)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            content_parent.spawn((
                Text::new(format!("Earned {} Cheddah!", points_achieved)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 0.2)),
            ));

            // Pull History Display
            if let Some(ref state) = player_state {
                content_parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                ))
                .with_children(|history_parent| {
                    history_parent.spawn((
                        Text::new("Last Pull History:"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                    
                    history_parent.spawn((
                        Node {
                            width: Val::Px(190.0), // Container width for 5 orbs (30px each + 10px gaps)
                            height: Val::Px(30.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.0),
                            margin: UiRect::top(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.2)),
                    ))
                    .with_children(|container_parent| {
                        // Show the last 5 orbs from pull history
                        for orb in state.pull_history().iter() {
                            let (color, symbol) = match orb {
                                Orb::Health => (Color::srgb(0.2, 0.8, 0.2), "H"),
                                Orb::Point => (Color::srgb(0.2, 0.2, 0.8), "P"),
                                Orb::Bomb => (Color::srgb(0.8, 0.2, 0.2), "B"),
                            };
                            
                            container_parent.spawn((
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(color.with_alpha(0.3)),
                                BorderColor(color),
                            ))
                            .with_children(|orb_parent| {
                                orb_parent.spawn((
                                    Text::new(symbol),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                        }
                    });
                });
            }
        });

        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(70.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
            BorderColor(Color::srgb(0.4, 0.8, 0.4)),
            GoToMarketButton,
        ))
        .with_children(|button_parent| {
            button_parent.spawn((
                Text::new("SHOP"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ));
        });
    });
}

pub fn handle_go_to_market_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<GoToMarketButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.4, 0.1));
                *border_color = BorderColor(Color::srgb(0.2, 0.6, 0.2));
                info!("Going to marketplace");
                next_state.set(GameState::Marketplace);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.3, 0.7, 0.3));
                *border_color = BorderColor(Color::srgb(0.5, 0.9, 0.5));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
                *border_color = BorderColor(Color::srgb(0.4, 0.8, 0.4));
            }
        }
    }
}

pub fn cleanup_level_complete(mut commands: Commands, level_complete_query: Query<Entity, With<LevelCompleteUI>>) {
    for entity in &level_complete_query {
        commands.entity(entity).despawn();
    }
}