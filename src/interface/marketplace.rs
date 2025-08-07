use bevy::prelude::*;
use super::{GameState, StatDisplay, StatType};
use crate::game_state::PlayerGameState;

const ORB_COST: u32 = 2;

#[derive(Component)]
pub struct MarketplaceUI;

#[derive(Component)]
pub struct BuyHealthOrbButton;

#[derive(Component)]
pub struct BuyPointOrbButton;

#[derive(Component)]
pub struct NextLevelButton;

pub fn setup_marketplace_ui(mut commands: Commands, player_state: Option<Res<PlayerGameState>>) {
    let level = player_state.as_ref().map(|s| s.level()).unwrap_or(1);
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        MarketplaceUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("MARKETPLACE"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.7, 0.2)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));

        // Level complete message
        parent.spawn((
            Text::new(format!("Level {} Complete!", level)),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ));

        // Stats display container
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(5.0),
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ))
        .with_children(|stats_parent| {
            // Cheddah display
            stats_parent.spawn((
                Text::new("Cheddah: 0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 0.2)),
                StatDisplay { stat_type: StatType::Cheddah },
            ));
            
            // Base orbs for next level
            stats_parent.spawn((
                Text::new("Base Orbs (Next Level): H:5 P:5 B:5"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
            
            // Purchased orbs display (accumulates across all levels)
            stats_parent.spawn((
                Text::new("Total Purchased Orbs: H:0 P:0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 0.2)),
                StatDisplay { stat_type: StatType::Orbs },
            ));
        });

        // Info text about purchased orbs
        parent.spawn((
            Text::new("(Purchased orbs persist across all levels)"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));

        // Shop items container
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(15.0),
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ))
        .with_children(|shop_parent| {
            // Buy Health Orb button
            shop_parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.4, 0.2)),
                BorderColor(Color::srgb(0.4, 0.6, 0.4)),
                BuyHealthOrbButton,
            ))
            .with_children(|button_parent| {
                button_parent.spawn((
                    Text::new(format!("Health (${})", ORB_COST)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            // Buy Point Orb button
            shop_parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.4, 0.4, 0.2)),
                BorderColor(Color::srgb(0.6, 0.6, 0.4)),
                BuyPointOrbButton,
            ))
            .with_children(|button_parent| {
                button_parent.spawn((
                    Text::new(format!("Point (${})", ORB_COST)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });

        // Next Level button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
            BorderColor(Color::srgb(0.4, 0.4, 0.6)),
            NextLevelButton,
        ))
        .with_children(|button_parent| {
            button_parent.spawn((
                Text::new("NEXT LEVEL"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn handle_buy_health_orb(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<BuyHealthOrbButton>),
    >,
    mut player_state: Option<ResMut<PlayerGameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.2, 0.1));
                *border_color = BorderColor(Color::srgb(0.2, 0.4, 0.2));
                
                if let Some(ref mut state) = player_state {
                    if state.purchase_health_orb(ORB_COST) {
                        info!("Purchased Health orb!");
                    } else {
                        info!("Not enough cheddah to purchase Health orb");
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.3, 0.5, 0.3));
                *border_color = BorderColor(Color::srgb(0.5, 0.7, 0.5));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.4, 0.2));
                *border_color = BorderColor(Color::srgb(0.4, 0.6, 0.4));
            }
        }
    }
}

pub fn handle_buy_point_orb(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<BuyPointOrbButton>),
    >,
    mut player_state: Option<ResMut<PlayerGameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.1));
                *border_color = BorderColor(Color::srgb(0.4, 0.4, 0.2));
                
                if let Some(ref mut state) = player_state {
                    if state.purchase_point_orb(ORB_COST) {
                        info!("Purchased Point orb!");
                    } else {
                        info!("Not enough cheddah to purchase Point orb");
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.3));
                *border_color = BorderColor(Color::srgb(0.7, 0.7, 0.5));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.2));
                *border_color = BorderColor(Color::srgb(0.6, 0.6, 0.4));
            }
        }
    }
}

pub fn handle_next_level_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<NextLevelButton>),
    >,
    mut player_state: Option<ResMut<PlayerGameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.2));
                *border_color = BorderColor(Color::srgb(0.2, 0.2, 0.4));
                
                if let Some(ref mut state) = player_state {
                    // Advance to next level (this will reset orbs to base + purchased)
                    state.advance_to_next_level();
                    info!("Advancing to level {} with {} total orbs", state.level(), state.orbs.len());
                }
                
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.5));
                *border_color = BorderColor(Color::srgb(0.5, 0.5, 0.7));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.4));
                *border_color = BorderColor(Color::srgb(0.4, 0.4, 0.6));
            }
        }
    }
}

pub fn update_marketplace_stats(
    player_state: Option<Res<PlayerGameState>>,
    mut stats_query: Query<(&mut Text, &StatDisplay)>,
) {
    if let Some(state) = player_state {
        for (mut text, stat_display) in &mut stats_query {
            **text = match stat_display.stat_type {
                StatType::Cheddah => format!("Cheddah: {}", state.cheddah()),
                StatType::Orbs => format!("Total Purchased Orbs: H:{} P:{}", 
                    state.purchased_orb_count(crate::game_state::orb::Orb::Health), 
                    state.purchased_orb_count(crate::game_state::orb::Orb::Point)
                ),
                _ => continue,
            };
        }
    }
}

pub fn cleanup_marketplace(mut commands: Commands, marketplace_query: Query<Entity, With<MarketplaceUI>>) {
    for entity in &marketplace_query {
        commands.entity(entity).despawn();
    }
}