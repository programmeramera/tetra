// Loosely based on https://github.com/jonhoo/tetris-tutorial.
// The scene stack implementation is inspired by Amethyst's state system
// and the ggez-goodies scene stack.

use rand::{self, Rng};
use tetra::audio::Sound;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Color, DrawParams, Font, Text, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

fn main() -> tetra::Result {
    ContextBuilder::new("Tetras", 640, 480)
        .resizable(true)
        .quit_on_escape(false)
        .build()?
        .run(GameState::new)
}

// === Scene Management ===

// This trait extends the normal signature of a 'State' with the ability
// to return a transition, effectively making it function like a state
// machine. Later versions of Tetra will probably provide a way to
// do this without defining your own trait!

trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context, dt: f64) -> tetra::Result;
}

enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
}

// Boxing/dynamic dispatch could be avoided here by defining an enum for all
// of your scenes, but that adds a bit of extra boilerplate - your choice!

struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    scaler: ScreenScaler,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let initial_scene = MenuScene::new(ctx)?;

        Ok(GameState {
            scenes: vec![Box::new(initial_scene)],
            scaler: ScreenScaler::new(ctx, 640, 480, ScalingMode::ShowAllPixelPerfect)?,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.scaler.update(ctx);

        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None => window::quit(ctx),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, dt: f64) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());

        match self.scenes.last_mut() {
            Some(active_scene) => active_scene.draw(ctx, dt)?,
            None => window::quit(ctx),
        };

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        graphics::draw(ctx, &self.scaler, Vec2::new(0.0, 0.0));

        Ok(())
    }
}

// === Splash Scene

// === Loading Scene

// === Pause Scene

// === Options Scene

// === Popup Scene

// === Menu Scene ===

struct MenuScene {
    title_text: Text
}

impl MenuScene {
    fn new(ctx: &mut Context) -> tetra::Result<MenuScene> {
        Ok(MenuScene {
            title_text: Text::new("Main Menu", Font::default(), 36.0)
        })
    }
}

impl Scene for MenuScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_key_pressed(ctx, Key::Space) {
            Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
        } else if input::is_key_pressed(ctx, Key::Escape) {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));
        graphics::draw(ctx, &self.title_text, Vec2::new(16.0, 16.0));

        Ok(())
    }
}

// === Game Scene ===

struct GameScene {
    text: Text,
}

impl GameScene {
    fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
        Ok(GameScene {
            text: Text::new("Game Play", Font::default(), 16.0),
        })
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.094, 0.51, 0.56));
        graphics::draw(ctx, &self.text, Vec2::new(16.0, 16.0));

        Ok(())
    }
}
