use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, MouseButton};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct GameState {
    texture: Texture,
    position: Vec2<f32>,
    scale: Vec2<f32>,
    rotation: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            texture: Texture::new(ctx, "./examples/resources/player.png")?,
            position: Vec2::new(32.0, 32.0),
            scale: Vec2::new(2.0, 2.0),
            rotation: 0.0,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.position = input::get_mouse_position(ctx).round();

        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            self.scale = Vec2::new(4.0, 4.0);
            self.rotation += 0.1;
        } else {
            self.scale = Vec2::new(2.0, 2.0);
            self.rotation = 0.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(8.0, 8.0))
                .scale(self.scale)
                .rotation(self.rotation),
        );

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Mouse Input", 640, 480)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
