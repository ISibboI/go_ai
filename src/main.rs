use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics;
use go::GoGame;
use go::board::{GoStone, GoCoordinates};
use ui::button::Button;
use ui::label::Label;

pub mod go;
pub mod ui;

const BACKGROUND_COLOR: graphics::Color = graphics::Color::new(0.831, 0.776, 0.509, 1.0);
const BOARD_LINE_COLOR: graphics::Color = graphics::Color::new(0.2, 0.1, 0.1, 1.0);
const WHITE_STONE_COLOR: graphics::Color = graphics::Color::new(0.9, 0.9, 0.9, 1.0);
const BLACK_STONE_COLOR: graphics::Color = graphics::Color::new(0.1, 0.1, 0.1, 1.0);
const NO_TEINT: graphics::Color = graphics::Color::WHITE;
const GHOST_TEINT: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 0.5);

const WINDOW_WIDTH: u16 = 800;
const WINDOW_HEIGHT: u16 = 600;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, event_loop) =
        ContextBuilder::new("Go AI", "Sebastian Schmidt")
            .build()
            .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    /*match event::run(ctx, event_loop, my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(_) => println!("Error"), //println!("Error occured: {}", e)
    }*/

    event::run(ctx, event_loop, my_game)
}

struct MyGame {
    grid: [f32; 9],
    grid_box_len: f32,
    game: GoGame,
    mouse_x: f32,
    mouse_y: f32,
    undo_button: Button,
    black_captures_label: Label,
    white_captures_label: Label,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        let mut grid = [0.0; 9];

        let square_side_len = WINDOW_WIDTH.min(WINDOW_HEIGHT) as f32;
        let grid_box_len = square_side_len / 9.0;
        for i in 0..9 {
            grid[i] = (i as f32 + 0.5) * grid_box_len;
        }

        let undo_button = Button::new(Label::new("Undo", [610.0, 10.0].into()));
        let black_captures_label = Label::new("Black captures: 0", [610.0, 30.0].into());
        let white_captures_label = Label::new("White captures: 0", [610.0, 50.0].into());

        MyGame { grid, grid_box_len, game: GoGame::new(), mouse_x: -1.0, mouse_y: -1.0, undo_button, black_captures_label, white_captures_label }
    }
}

impl EventHandler for MyGame {
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32
    ) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) {
        if button == MouseButton::Left && x >= 0.0 && y >= 0.0 && x <= self.grid_box_len * 9.0 && y <= self.grid_box_len * 9.0 {
            let x = ((x / self.grid_box_len).max(0.0) as usize).min(8);
            let y = ((y / self.grid_box_len).max(0.0) as usize).min(8);
            match self.game.play_stone(GoCoordinates::new_usize(x, y)) {
                Ok(_) => {}
                Err(_) => println!("Could not play stone")
            }
        }

        self.undo_button.mouse_button_up_event(ctx, button, x, y);
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.undo_button.consume_was_clicked() {
            self.game.undo().unwrap_or_else(|()| println!("Could not undo"));
        }

        self.black_captures_label.set_text(&format!("Black captures: {}", self.game.black_captures()));
        self.white_captures_label.set_text(&format!("White captures: {}", self.game.white_captures()));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BACKGROUND_COLOR);

        let mut board = graphics::MeshBuilder::new();
        for i in 0..9 {
            board.line(&[graphics::mint::Point2::from([self.grid[0], self.grid[i]]), graphics::mint::Point2::from([self.grid[8], self.grid[i]])], 2.0, BOARD_LINE_COLOR)?;
            board.line(&[graphics::mint::Point2::from([self.grid[i], self.grid[0]]), graphics::mint::Point2::from([self.grid[i], self.grid[8]])], 2.0, BOARD_LINE_COLOR)?;
        }
        board.circle(graphics::DrawMode::fill(), graphics::mint::Point2::from([self.grid[2], self.grid[2]]), 6.0, 1.0, BOARD_LINE_COLOR)?;
        board.circle(graphics::DrawMode::fill(), graphics::mint::Point2::from([self.grid[2], self.grid[6]]), 6.0, 1.0, BOARD_LINE_COLOR)?;
        board.circle(graphics::DrawMode::fill(), graphics::mint::Point2::from([self.grid[6], self.grid[2]]), 6.0, 1.0, BOARD_LINE_COLOR)?;
        board.circle(graphics::DrawMode::fill(), graphics::mint::Point2::from([self.grid[6], self.grid[6]]), 6.0, 1.0, BOARD_LINE_COLOR)?;

        let board = board.build(ctx)?;
        graphics::draw(ctx, &board, (graphics::mint::Point2::from([0.0, 0.0]), 0.0, NO_TEINT))?;

        let black_stone = graphics::MeshBuilder::new().circle(graphics::DrawMode::fill(), graphics::mint::Point2::from([0.0, 0.0]), self.grid_box_len / 2.0, 0.5, BLACK_STONE_COLOR)?.build(ctx)?;
        let white_stone = graphics::MeshBuilder::new().circle(graphics::DrawMode::fill(), graphics::mint::Point2::from([0.0, 0.0]), self.grid_box_len / 2.0, 0.5, WHITE_STONE_COLOR)?.build(ctx)?;
        for x in 0..9 {
            for y in 0..9 {
                let c = GoCoordinates::new_usize(x, y);
                match self.game.current_board().get_stone(c) {
                    GoStone::BLACK => graphics::draw(ctx, &black_stone, (graphics::mint::Point2::from([self.grid[x], self.grid[y]]), 0.0, NO_TEINT))?,
                    GoStone::WHITE => graphics::draw(ctx, &white_stone, (graphics::mint::Point2::from([self.grid[x], self.grid[y]]), 0.0, NO_TEINT))?,
                    GoStone::NONE => {
                        let x_f32 = x as f32;
                        let y_f32 = y as f32;

                        if self.grid_box_len * x_f32 < self.mouse_x && self.mouse_x < self.grid_box_len * (x_f32 + 1.0) &&
                            self.grid_box_len * y_f32 < self.mouse_y && self.mouse_y < self.grid_box_len * (y_f32 + 1.0) {
                            match self.game.current_turn() {
                                GoStone::BLACK => graphics::draw(ctx, &black_stone, (graphics::mint::Point2::from([self.grid[x], self.grid[y]]), 0.0, GHOST_TEINT))?,
                                GoStone::WHITE => graphics::draw(ctx, &white_stone, (graphics::mint::Point2::from([self.grid[x], self.grid[y]]), 0.0, GHOST_TEINT))?,
                                GoStone::NONE => {}
                            }
                        }
                    }
                }
            }
        }

        self.undo_button.draw(ctx)?;
        self.black_captures_label.draw(ctx)?;
        self.white_captures_label.draw(ctx)?;

        graphics::draw_queued_text(ctx, graphics::DrawParam::default(), None, graphics::FilterMode::Nearest)?;
        graphics::present(ctx)
    }
}