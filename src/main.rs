use tetra::graphics::mesh::Mesh;
use tetra::graphics::DrawParams;
use tetra::graphics::{self, mesh::ShapeStyle, Color, Rectangle, Texture};
use tetra::input::{self, MouseButton};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const TITLE: &str = "Tic Tac Toe";

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct GameState {
    current_turn: Turn,
    grid: [CellState; 9],
    textures: MarkTextures,
}

#[derive(Clone, Copy)]
enum Turn {
    X,
    O,
}

#[derive(Clone, Copy)]
enum CellState {
    Empty,
    X,
    O,
}

impl From<Turn> for CellState {
    fn from(turn: Turn) -> Self {
        match turn {
            Turn::O => CellState::O,
            Turn::X => CellState::X,
        }
    }
}

struct MarkTextures {
    x: Texture,
    o: Texture,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            current_turn: Turn::X,
            grid: [CellState::Empty; 9],
            textures: MarkTextures {
                x: Texture::new(ctx, "resources/x.png")?,
                o: Texture::new(ctx, "resources/o.png")?,
            },
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        let mark_size = self.textures.o.width() as f32;
        let gap_size: f32 = 15.0;
        let bbox_side: f32 = mark_size * 3.0 + gap_size * 2.0;

        let start_x = (SCREEN_WIDTH - bbox_side) / 2.0;
        let start_y = (SCREEN_HEIGHT - bbox_side) / 2.0;

        let mark_color = Color::rgba8(50, 84, 137, 255);
        let border_color = mark_color;

        Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(7.0),
            Rectangle::new(
                start_x - 15.0,
                start_y - 15.0,
                bbox_side + 30.0,
                bbox_side + 30.0,
            ),
        )?
        .draw(ctx, DrawParams::new().color(border_color));

        for (i, &item) in self.grid.iter().enumerate() {
            let x = i % 3;
            let y = i / 3;

            if let CellState::Empty = item {
                continue;
            }
            let texture = match item {
                CellState::X => &self.textures.x,
                CellState::O => &self.textures.o,
                CellState::Empty => unreachable!(),
            };

            texture.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        start_x + x as f32 * (mark_size + gap_size),
                        start_y + y as f32 * (mark_size + gap_size),
                    ))
                    .color(mark_color),
            )
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            let mouse_pos = input::get_mouse_position(ctx);

            let mark_size = self.textures.o.width() as f32;
            let gap_size: f32 = 15.0;
            let bbox_side: f32 = mark_size * 3.0 + gap_size * 2.0;

            let start_x = (SCREEN_WIDTH - bbox_side) / 2.0;
            let start_y = (SCREEN_HEIGHT - bbox_side) / 2.0;

            let end_x = start_x + bbox_side;
            let end_y = start_y + bbox_side;

            if mouse_pos.x >= start_x
                && mouse_pos.x <= end_x
                && mouse_pos.y >= start_y
                && mouse_pos.y <= end_y
            {
                let offset_x = mouse_pos.x - start_x;
                let offset_y = mouse_pos.y - start_y;

                let cell_side = bbox_side / 3.0;

                let ix = (offset_x / cell_side) as usize;
                let iy = (offset_y / cell_side) as usize;

                let ref mut cell = self.grid[iy * 3 + ix];
                if let CellState::Empty = cell {
                    *cell = self.current_turn.into();
                    self.current_turn = match self.current_turn {
                        Turn::O => Turn::X,
                        Turn::X => Turn::O,
                    }
                }
            }
        }

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
