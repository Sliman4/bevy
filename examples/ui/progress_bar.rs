use bevy::prelude::*;
use bevy_internal::ui::widget::ProgressBarAnimation;

/// This example illustrates how to create a progress that changes based on arbitrary data.
/// In this example we use buttons to add progress, but you probably want something else,
/// e.g. assets loading, connection to a server.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut progress_query: Query<&mut Progress>,
) {
    let mut progress = progress_query.single_mut();
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if progress.is_done() {
                    progress.set(0.0);
                    text.sections[0].value = "Add 5%".to_string();
                } else {
                    *progress += 5.0;
                    if progress.is_done() {
                        text.sections[0].value = "Reset".to_string();
                    }
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(200.0)),
                // center it
                margin: Rect::all(Val::Auto),
                // progress bar should be above the button
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            // root node is transparent
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // progress bar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(270.0), Val::Px(50.0)),
                        // center progress bar
                        margin: Rect::all(Val::Auto),
                        // add a black border
                        border: Rect::all(Val::Px(7.0)),
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ProgressBarBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Percent(100.0)),
                                ..Default::default()
                            },
                            progress: Progress::new(20.0),
                            // increase this node's width with its progress
                            resize_animation: ProgressBarAnimation::ResizeWidth,
                            color: Color::GREEN.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // bottom line just to make it look nicer
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(55.0)),
                                    ..Default::default()
                                },
                                color: Color::LIME_GREEN.into(),
                                ..Default::default()
                            });
                        });
                });

            // button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Add 5%",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}
