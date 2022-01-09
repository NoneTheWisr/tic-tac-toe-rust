use crate::logic::{CellState, GameData, GameState};

use tetra::graphics::mesh::Mesh;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::DrawParams;
use tetra::graphics::{self, mesh::ShapeStyle, Color, Rectangle, Texture};
use tetra::input::{self, Key, MouseButton};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

// -------------------------------------------------------------------------- //
// Constants                                                                  //
// -------------------------------------------------------------------------- //

const TITLE: &str = "Tic Tac Toe";

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

const MARK_SIZE: f32 = 64.0;
const SPACING: f32 = 30.0;

const FONT_SIZE: f32 = 64.0;

const BOARD_SIZE: f32 = MARK_SIZE * 3.0 + SPACING * 2.0;
const BOARD_BEG_X: f32 = (SCREEN_WIDTH - BOARD_SIZE) / 2.0;
const BOARD_BEG_Y: f32 = (SCREEN_HEIGHT - BOARD_SIZE) / 2.0;
const BOARD_END_X: f32 = BOARD_BEG_X + BOARD_SIZE;
const BOARD_END_Y: f32 = BOARD_BEG_Y + BOARD_SIZE;

// -------------------------------------------------------------------------- //
// The run function                                                           //
// -------------------------------------------------------------------------- //

pub fn run() -> tetra::Result {
    ContextBuilder::new(TITLE, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(TetraState::new)
}

// -------------------------------------------------------------------------- //
// State struct implementation                                                //
// -------------------------------------------------------------------------- //

struct TetraState {
    game_data: GameData,
    textures: MarkTextures,
    text: Text,
}

struct MarkTextures {
    x: Texture,
    o: Texture,
}

impl TetraState {
    fn new(ctx: &mut Context) -> tetra::Result<TetraState> {
        Ok(TetraState {
            game_data: GameData::new(),
            textures: MarkTextures {
                x: Texture::new(ctx, "resources/x.png")?,
                o: Texture::new(ctx, "resources/o.png")?,
            },
            text: Text::new(
                "",
                Font::vector(ctx, "resources/FredokaOne-Regular.ttf", FONT_SIZE)?,
            ),
        })
    }
}

impl State for TetraState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        let main_color = Color::rgba8(50, 84, 137, 255);

        // Draw the border
        Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(7.0),
            Rectangle::new(
                BOARD_BEG_X - SPACING,
                BOARD_BEG_Y - SPACING,
                BOARD_SIZE + 2.0 * SPACING,
                BOARD_SIZE + 2.0 * SPACING,
            ),
        )?
        .draw(ctx, DrawParams::new().color(main_color));

        // Draw the marks
        for (i, &item) in self.game_data.board().iter().enumerate() {
            let texture;
            match item {
                CellState::X => texture = &self.textures.x,
                CellState::O => texture = &self.textures.o,
                CellState::Empty => continue,
            }

            let x = i % 3;
            let y = i / 3;

            texture.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        BOARD_BEG_X + x as f32 * (MARK_SIZE + SPACING),
                        BOARD_BEG_Y + y as f32 * (MARK_SIZE + SPACING),
                    ))
                    .color(main_color),
            )
        }

        // Draw the status text
        let status = match self.game_data.state() {
            GameState::XsTurn => "X's turn",
            GameState::OsTurn => "O's turn",
            GameState::XWon => "X wins!",
            GameState::OWon => "O wins!",
            GameState::Stalemate => "Stalemate",
        };

        self.text.set_content(status);
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(
                    BOARD_BEG_X - SPACING,
                    BOARD_BEG_Y - FONT_SIZE - 1.5 * SPACING,
                ))
                .color(main_color),
        );

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_pressed(ctx, Key::Enter) {
            self.game_data = GameData::new();
        } else if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let mouse_pos = input::get_mouse_position(ctx);

            if mouse_pos.x >= BOARD_BEG_X
                && mouse_pos.x <= BOARD_END_X
                && mouse_pos.y >= BOARD_BEG_Y
                && mouse_pos.y <= BOARD_END_Y
            {
                let offset_x = mouse_pos.x - BOARD_BEG_X;
                let offset_y = mouse_pos.y - BOARD_BEG_Y;

                let cell_size = BOARD_SIZE / 3.0;

                let board_x = (offset_x / cell_size) as usize;
                let board_y = (offset_y / cell_size) as usize;

                self.game_data.turn(board_y * 3 + board_x);
            }
        }

        Ok(())
    }
}
