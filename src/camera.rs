use bevy::{
    input::mouse::{
        AccumulatedMouseMotion, AccumulatedMouseScroll, MouseMotion, MouseScrollUnit, MouseWheel,
    },
    prelude::*,
    window::CursorGrabMode,
};
use std::{f32::consts::*, fmt};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, toggle_move_mode)
            .add_systems(FixedUpdate, free_fly.run_if(is_free_fly_enabled))
            .add_systems(FixedUpdate, (orbit, zoom).run_if(is_zoom_orbit_enabled));
    }
}

// Pitch-Yaw sensitivy scaling for free-fly mode.
const RADIANS_PER_DOT: f32 = 1.0 / 1000.0;

/// Camera's Z coordinate for Zoom-Orbit mode.
/// Defines the starting zoom level.
const ZOOM_ORBIT_Z: f32 = 11.2;

#[derive(PartialEq, Debug)]
enum MoveMode {
    ZoomOrbit,
    FreeFly,
}

#[derive(Component, Default)]
struct OrbitOrigo {}

/// Camera controller [`Component`].
#[derive(Component)]
struct CameraController {
    // /// Enables this [`CameraController`] when `true`.
    // pub enabled: bool,
    /// Current camera move mode.
    move_mode: MoveMode,
    /// Indicates if this controller has been initialized by the [`CameraControllerPlugin`].
    initialized: bool,
    /// Multiplier for pitch and yaw rotation speed.
    sensitivity: f32,
    /// [`KeyCode`] for forward translation.
    key_forward: KeyCode,
    /// [`KeyCode`] for backward translation.
    key_back: KeyCode,
    /// [`KeyCode`] for left translation.
    key_left: KeyCode,
    /// [`KeyCode`] for right translation.
    key_right: KeyCode,
    /// [`KeyCode`] for up translation.
    key_up: KeyCode,
    /// [`KeyCode`] for down translation.
    key_down: KeyCode,
    /// [`KeyCode`] to use [`run_speed`](CameraController::run_speed) instead of
    /// [`walk_speed`](CameraController::walk_speed) for translation.
    key_run: KeyCode,
    /// [`MouseButton`] for grabbing the mouse focus.
    mouse_key_cursor_grab: MouseButton,
    /// [`KeyCode`] for grabbing the keyboard focus.
    keyboard_key_toggle_cursor_grab: KeyCode,
    /// Multiplier for unmodified translation speed.
    walk_speed: f32,
    /// Multiplier for running translation speed.
    run_speed: f32,
    /// Multiplier for how the mouse scroll wheel modifies [`walk_speed`](CameraController::walk_speed)
    /// and [`run_speed`](CameraController::run_speed).
    scroll_factor: f32,
    /// Friction factor used to exponentially decay [`velocity`](CameraController::velocity) over time.
    friction: f32,
    /// This [`CameraController`]'s pitch rotation.
    pitch: f32,
    /// This [`CameraController`]'s yaw rotation.
    yaw: f32,
    /// This [`CameraController`]'s translation velocity.
    velocity: Vec3,
}

impl CameraController {
    fn toggle_move_mode(&mut self) {
        self.move_mode = match self.move_mode {
            MoveMode::ZoomOrbit => MoveMode::FreeFly,
            MoveMode::FreeFly => MoveMode::ZoomOrbit,
        };
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            move_mode: MoveMode::ZoomOrbit,
            initialized: false,
            sensitivity: 1.0,
            key_forward: KeyCode::KeyW,
            key_back: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyE,
            key_down: KeyCode::KeyQ,
            key_run: KeyCode::ShiftLeft,
            mouse_key_cursor_grab: MouseButton::Left,
            keyboard_key_toggle_cursor_grab: KeyCode::KeyM,
            walk_speed: 5.0,
            run_speed: 15.0,
            scroll_factor: 0.1,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

impl fmt::Display for CameraController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
Freecam Controls:
    Mouse\t- Move camera orientation
    Scroll\t- Adjust movement speed
    {:?}\t- Hold to grab cursor
    {:?}\t- Toggle cursor grab
    {:?} & {:?}\t- Fly forward & backwards
    {:?} & {:?}\t- Fly sideways left & right
    {:?} & {:?}\t- Fly up & down
    {:?}\t- Fly faster while held",
            self.mouse_key_cursor_grab,
            self.keyboard_key_toggle_cursor_grab,
            self.key_forward,
            self.key_back,
            self.key_left,
            self.key_right,
            self.key_up,
            self.key_down,
            self.key_run,
        )
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // origin position marker
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 12.0, 4.0),
    ));

    // camera
    let camera = commands
        .spawn((
            CameraController::default(),
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.0, ZOOM_ORBIT_Z),
        ))
        .id();

    // camera parent for orbit-zoom mode
    let orbit_origo = commands
        .spawn((
            OrbitOrigo::default(),
            Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 3.0)),
        ))
        .id();

    commands.entity(orbit_origo).add_child(camera);
}

fn is_move_mode_enabled(query: &Query<&CameraController>, move_mode: MoveMode) -> bool {
    let camera_ctrl = query.single().unwrap();
    camera_ctrl.move_mode == move_mode
}

fn is_free_fly_enabled(query: Query<&CameraController>) -> bool {
    is_move_mode_enabled(&query, MoveMode::FreeFly)
}

fn is_zoom_orbit_enabled(query: Query<&CameraController>) -> bool {
    is_move_mode_enabled(&query, MoveMode::ZoomOrbit)
}

fn toggle_move_mode(
    key_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<&mut CameraController>,
    orbit_query: Query<Entity, With<OrbitOrigo>>,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    fn setup_zoom_orbit(commands: &mut Commands, orbit_origo: Entity, camera: Entity) {
        commands.entity(orbit_origo).add_child(camera);
        commands
            .entity(camera)
            .insert(Transform::from_xyz(0.0, 0.0, ZOOM_ORBIT_Z));
    }

    fn setup_free_fly(commands: &mut Commands, orbit_origo: Entity) {
        commands.entity(orbit_origo).remove::<Children>();
    }

    if !key_input.just_pressed(KeyCode::Escape) {
        return;
    }

    let mut camera_ctrl = query.single_mut().unwrap();
    camera_ctrl.toggle_move_mode();

    let orbit_origo = orbit_query.single().unwrap();

    match camera_ctrl.move_mode {
        MoveMode::ZoomOrbit => {
            setup_zoom_orbit(&mut commands, orbit_origo, camera_query.single().unwrap())
        }
        MoveMode::FreeFly => {
            setup_free_fly(&mut commands, orbit_origo);
        }
    }
}

fn orbit(
    mut evr_motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Transform, With<OrbitOrigo>>,
) {
    //
    // orbit camera around origin, when
    // mouse is dragged horizontally
    //
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let mut trans = query.single_mut().unwrap();

    for ev in evr_motion.read() {
        trans.rotate_y(ev.delta.x * -0.01);
    }
}

fn zoom(mut evr_wheel: EventReader<MouseWheel>, mut query: Query<&mut Transform, With<Camera3d>>) {
    let mut trans = query.single_mut().unwrap();

    for ev in evr_wheel.read() {
        trans.translation.z -= ev.y * 0.1;
    }
}

fn free_fly(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_cursor_grab: Local<bool>,
    mut mouse_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_secs();

    let Ok((mut transform, mut controller)) = query.single_mut() else {
        return;
    };

    if !controller.initialized {
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        controller.yaw = yaw;
        controller.pitch = pitch;
        controller.initialized = true;
        info!("{}", *controller);
    }

    let mut scroll = 0.0;

    let amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
    };
    scroll += amount;
    controller.walk_speed += scroll * controller.scroll_factor * controller.walk_speed;
    controller.run_speed = controller.walk_speed * 3.0;

    // Handle key input
    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(controller.key_forward) {
        axis_input.z += 1.0;
    }
    if key_input.pressed(controller.key_back) {
        axis_input.z -= 1.0;
    }
    if key_input.pressed(controller.key_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(controller.key_left) {
        axis_input.x -= 1.0;
    }
    if key_input.pressed(controller.key_up) {
        axis_input.y += 1.0;
    }
    if key_input.pressed(controller.key_down) {
        axis_input.y -= 1.0;
    }

    let mut cursor_grab_change = false;
    if key_input.just_pressed(controller.keyboard_key_toggle_cursor_grab) {
        *toggle_cursor_grab = !*toggle_cursor_grab;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_pressed(controller.mouse_key_cursor_grab) {
        *mouse_cursor_grab = true;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_released(controller.mouse_key_cursor_grab) {
        *mouse_cursor_grab = false;
        cursor_grab_change = true;
    }
    let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;

    // Apply movement update
    if axis_input != Vec3::ZERO {
        let max_speed = if key_input.pressed(controller.key_run) {
            controller.run_speed
        } else {
            controller.walk_speed
        };
        controller.velocity = axis_input.normalize() * max_speed;
    } else {
        let friction = controller.friction.clamp(0.0, 1.0);
        controller.velocity *= 1.0 - friction;
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }
    let forward = *transform.forward();
    let right = *transform.right();
    transform.translation += controller.velocity.x * dt * right
        + controller.velocity.y * dt * Vec3::Y
        + controller.velocity.z * dt * forward;

    // Handle cursor grab
    if cursor_grab_change {
        if cursor_grab {
            for mut window in &mut windows {
                if !window.focused {
                    continue;
                }

                window.cursor_options.grab_mode = CursorGrabMode::Locked;
                window.cursor_options.visible = false;
            }
        } else {
            for mut window in &mut windows {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
            }
        }
    }

    // Handle mouse input
    if accumulated_mouse_motion.delta != Vec2::ZERO && cursor_grab {
        // Apply look update
        controller.pitch = (controller.pitch
            - accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * controller.sensitivity)
            .clamp(-PI / 2., PI / 2.);
        controller.yaw -=
            accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * controller.sensitivity;
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
    }
}
