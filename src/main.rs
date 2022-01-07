use tetra::graphics::{self, Color, Texture, DrawParams};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const TITLE: &str = "Tic Tac Toe";

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct MarkTextures {
    x: Texture,
    o: Texture,
}
struct GameState {
    textures: MarkTextures,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let textures = MarkTextures {
            x: Texture::new(ctx, "resources/x.png")?,
            o: Texture::new(ctx, "resources/o.png")?,
        };

        Ok(GameState {
            textures
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        let total_width: f32 = 64.0 * 2.0 + 32.0;
        let total_height: f32 = 64.0;

        let start_x = (SCREEN_WIDTH - total_width) / 2.0;
        let start_y = (SCREEN_HEIGHT - total_height) / 2.0;

        self.textures.x.draw(ctx, Vec2::new(start_x, start_y));
        self.textures.o.draw(ctx, Vec2::new(start_x + 64.0 + 32.0, start_y));

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new(TITLE, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
