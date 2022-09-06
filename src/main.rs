use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    high_score: u32,
    score: u32,
    ferris_index: i32,
    spawn_timer: Timer,
    // health_left: i32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState { 
            high_score: 0,
            score: 0, 
            ferris_index: 0, 
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.window_settings(WindowDescriptor{
        title: "Tutorial!".to_string(),
        width: 1400.0,
        height: 500.0,
        ..Default::default()   
    });

    game.audio_manager.
        play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(-500.0, 0.0);
    player.rotation = RIGHT;
    player.scale = 0.75;
    player.collision = true;

    let score = game.add_text("Score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("High Score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);
    
    game.add_logic(game_logic);

    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // quit when Q is pressed or esc
    if engine.keyboard_state.just_pressed_any(&[KeyCode::Q, KeyCode::Escape]) {
        engine.should_exit = true;
    }

    // keep text near the edges
    let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;
    let score = engine.texts.get_mut("Score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;
    let high_score = engine.texts.get_mut("High Score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    // handling collision
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;
            let score = engine.texts.get_mut("Score").unwrap();
            score.value = format!("Score: {}", game_state.score);

            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("High Score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);
            }
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.6);
        }
    }

    // handling movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 80.0;
    if engine.keyboard_state.
        pressed_any(&[KeyCode::W, KeyCode::Up]) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.
        pressed_any(&[KeyCode::S, KeyCode::Down]) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.
        pressed_any(&[KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.
        pressed_any(&[KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("ferris{}", game_state.ferris_index);
            game_state.ferris_index += 1;
            let ferris = engine.add_sprite(label.clone(), "virus-sprite.png");
            ferris.translation = mouse_location;
            ferris.collision = true;
            ferris.scale = 0.5;
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferris{}", game_state.ferris_index);
        game_state.ferris_index += 1;
        let ferris = engine.add_sprite(label.clone(), "pasfoto.png");
        ferris.translation.x = thread_rng().gen_range(-550.0..550.0);
        ferris.translation.y = thread_rng().gen_range(-325.0..325.0);

        ferris.collision = true;
        ferris.scale = 0.5;
    }

    // reset score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("Score").unwrap();
        score.value = "Score: 0".to_string();
    }
}
