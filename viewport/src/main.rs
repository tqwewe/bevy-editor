use bevy::{
    prelude::*,
    window::{PresentMode, WindowId},
    winit::WinitWindows,
};

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(60, 60, 60)))
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Immediate,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#viewport".to_string()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(spawn_on_click_system);

    #[cfg(target_arch = "wasm32")]
    app.add_system(bevy_web_resizer::web_resize_system);

    app.run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    const SPACING: f32 = 20.0;
    const LENGTH: f32 = 100000.0;
    const THICKNESS: f32 = 2.0;

    // X axis
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(141, 71, 79),
            custom_size: Some(Vec2::new(LENGTH, THICKNESS)),
            ..Default::default()
        },
        ..Default::default()
    });

    // Y axis
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(112, 139, 62),
            custom_size: Some(Vec2::new(THICKNESS, LENGTH)),
            ..Default::default()
        },
        ..Default::default()
    });

    for i in 1..=100 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(LENGTH, THICKNESS)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, i as f32 * SPACING, 0.0),
            ..Default::default()
        });
    }

    for i in -100..=-1 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(LENGTH, THICKNESS)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, i as f32 * SPACING, 0.0),
            ..Default::default()
        });
    }

    for j in 1..=100 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(THICKNESS, LENGTH)),
                ..Default::default()
            },
            transform: Transform::from_xyz(j as f32 * SPACING, 0.0, 0.0),
            ..Default::default()
        });
    }

    for j in -100..=-1 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(THICKNESS, LENGTH)),
                ..Default::default()
            },
            transform: Transform::from_xyz(j as f32 * SPACING, 0.0, 0.0),
            ..Default::default()
        });
    }
}

fn spawn_on_click_system(
    mut commands: Commands,
    windows: Res<Windows>,
    winit_windows: Res<WinitWindows>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if let Some(window) = windows.get_primary() {
        if let Some(winit_window) = winit_windows.get_window(WindowId::primary()) {
            let size = winit_window.inner_size();

            if mouse_input.just_pressed(MouseButton::Left) {
                if let Some(pos) = window.cursor_position() {
                    commands.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb_u8(141, 71, 79),
                            custom_size: Some(Vec2::new(5.0, 5.0)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(
                            (pos - Vec2::new(window.width() / 2.0, window.height() / 2.0))
                                .extend(0.0),
                        ),
                        ..Default::default()
                    });
                }
            }
        }
    }
}
