#![allow(unused)]

use bevy::{
    prelude::*, 
    window::PresentMode,
};
use rand::Rng;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const PADDLE_WIDTH: f32 = 15.0;
pub const PADDLE_HEIGHT: f32 = 80.0;
pub const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH / 2.0;
pub const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT / 2.0;
pub const BALL_WIDTH: f32 = 15.0;
pub const BALL_HEIGHT: f32 = 15.0;
pub const BALL_HALF: f32 = BALL_HEIGHT / 2.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const WINDOW_WIDTH: f32 = 450.0;
pub const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT / 2.0;
pub const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH / 2.0;
pub const BALL_SPEED: f32 = 400.0;
pub const PADDLE_SPEED: f32 = 500.0;
pub const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
pub struct LeftPaddle;
#[derive(Component)]
pub struct RightPaddle;
#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity{
    y: f32
}
#[derive(Component)]
pub struct BallDirection{
    x: f32,
    y: f32
}
#[derive(Component)]
pub struct PaddlePosition {
    pl: f32,
    pr: f32
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: WINDOW_HEIGHT,
            height: WINDOW_WIDTH,
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(left_paddle_move)
        .add_system(right_paddle_move)
        .add_system(ball_move)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rng = rand::thread_rng();

    commands.insert_resource(PaddlePosition {
        pl: 0.0,
        pr: 0.0
    });

    //BALL
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite{
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_WIDTH, BALL_HEIGHT)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    })
    .insert(Ball)
    .insert(BallDirection{
        x: (BALL_SPEED + rng.gen_range(-10.0..10.0)) * rng.gen_range(-0.1..0.1) * 10.0 * TIME_STEP,
        y: (BALL_SPEED + rng.gen_range(-10.0..10.0)) * rng.gen_range(-0.1..0.1) * 10.0 * TIME_STEP});

    //LEFT PADDLE
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite{
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        transform: Transform::from_xyz(-WINDOW_WIDTH_HALF - 2.0 * PADDLE_HEIGHT, 0.0, 0.0),
        ..Default::default()
    })
    .insert(LeftPaddle)
    .insert(Velocity{ y: 1.0 });

    //RIGHT PADDLE
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite{
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        transform: Transform::from_xyz(WINDOW_WIDTH_HALF + 2.0 * PADDLE_HEIGHT, 0.0, 0.0),
        ..Default::default()
    })
    .insert(RightPaddle)
    .insert(Velocity{ y: 1.0 });
}  

fn left_paddle_move(
    mut qeury: Query<(&Velocity, &mut Transform), With<LeftPaddle>>,
    keys: Res<Input<KeyCode>>,
    mut position: ResMut<PaddlePosition>,
){
    for (velocity, mut transform) in qeury.iter_mut() {
        if keys.pressed(KeyCode::W) && transform.translation.y < WINDOW_HEIGHT_HALF / 2.0 - 2.0 * PADDLE_WIDTH{
            let translation = &mut transform.translation;
            translation.y += velocity.y * PADDLE_SPEED * TIME_STEP;
        }
        if keys.pressed(KeyCode::S) && transform.translation.y > -WINDOW_HEIGHT_HALF / 2.0 + 2.0 * PADDLE_WIDTH{
            let translation = &mut transform.translation;
            translation.y -= velocity.y * PADDLE_SPEED * TIME_STEP;
        }
        position.pl = transform.translation.y;
    }
}

fn right_paddle_move(
    mut qeury: Query<(&Velocity, &mut Transform), With<RightPaddle>>,
    keys: Res<Input<KeyCode>>,
    mut position: ResMut<PaddlePosition>,
){
    for (velocity, mut transform) in qeury.iter_mut() {
        if keys.pressed(KeyCode::Up) && transform.translation.y < WINDOW_HEIGHT_HALF / 2.0 - 2.0 * PADDLE_WIDTH{
            let translation = &mut transform.translation;
            translation.y += velocity.y * PADDLE_SPEED * TIME_STEP;
        }
        if keys.pressed(KeyCode::Down) && transform.translation.y > -WINDOW_HEIGHT_HALF / 2.0 + 2.0 * PADDLE_WIDTH{
            let translation = &mut transform.translation;
            translation.y -= velocity.y * PADDLE_SPEED * TIME_STEP;
        }
        position.pr = transform.translation.y;
    }
}

fn ball_move(
    mut qeury: Query<(&mut BallDirection, &mut Transform), With<Ball>>,
    position: ResMut<PaddlePosition>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
){
    for (mut direction, mut transform) in qeury.iter_mut() {
        transform.translation.x +=  direction.x;
        transform.translation.y +=  direction.y;

        let mut rng = rand::thread_rng();

        let hit_wall = asset_server.load("hitwall.ogg");
        let hit_paddle = asset_server.load("hitpaddle.ogg");
        let miss = asset_server.load("miss.ogg");

        //BALL HITTING FLOOR AND CEILING
        if transform.translation.y > (WINDOW_HEIGHT_HALF / 1.85) || transform.translation.y < (-WINDOW_HEIGHT_HALF / 1.85) {
            direction.y = -direction.y;

            audio.play(hit_wall);
        }
        //BALL HITTING PADDLES
        if ((transform.translation.y + BALL_HALF > position.pr - PADDLE_HEIGHT / 2.0 || transform.translation.y - BALL_HALF > position.pr - PADDLE_HEIGHT / 2.0)
        && (transform.translation.y - BALL_HALF < position.pr + PADDLE_HEIGHT / 2.0 || transform.translation.y + BALL_HALF < position.pr + PADDLE_HEIGHT / 2.0))
        && transform.translation.x > (WINDOW_WIDTH - PADDLE_HEIGHT) && (transform.translation.x < (WINDOW_WIDTH - PADDLE_HEIGHT + 5.0))
        ||
        ((transform.translation.y + BALL_HALF > position.pl - PADDLE_HEIGHT / 2.0 || transform.translation.y - BALL_HALF > position.pl - PADDLE_HEIGHT / 2.0)
        && (transform.translation.y - BALL_HALF < position.pl + PADDLE_HEIGHT_HALF || transform.translation.y + BALL_HALF < position.pl + PADDLE_HEIGHT / 2.0))
        && (transform.translation.x < (-WINDOW_WIDTH + PADDLE_HEIGHT) && transform.translation.x > (-WINDOW_WIDTH + PADDLE_HEIGHT - 5.0)){
            direction.x = -direction.x;
            audio.play(hit_paddle);
        }

        //MISSING
        if transform.translation.x < -WINDOW_WIDTH || transform.translation.x > WINDOW_WIDTH + PADDLE_HEIGHT {
            transform.translation.y = 0.0;
            transform.translation.x = 0.0;

            direction.x = (BALL_SPEED + rng.gen_range(-10.0..10.0)) * rng.gen_range(-0.1..0.1) * 10.0 * TIME_STEP;
            direction.y = (BALL_SPEED + rng.gen_range(-10.0..10.0)) * rng.gen_range(-0.1..0.1) * 10.0 * TIME_STEP;

            audio.play(miss);
        }
    }
}
