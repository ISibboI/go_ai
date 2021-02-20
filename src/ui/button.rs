use ggez::graphics;
use ggez::{Context, GameResult};
use crate::ui::label::Label;
use ggez::event::MouseButton;

pub struct Button {
    label: Label,
    was_clicked: bool,
}

impl Button {
    pub fn new(label: Label) -> Self {
        Self {label: label, was_clicked: false}
    }

    pub fn set_label(&mut self, label: Label) {
        self.label = label;
    }

    pub fn dimensions(&self, ctx: &mut Context) -> graphics::Rect {
        let mut bounds = self.label.text().dimensions(ctx);
        let grow = 2.0;
        bounds.x -= grow;
        bounds.y -= grow;
        bounds.w += 2.0 * grow;
        bounds.h += 2.0 * grow;
        bounds
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.label.draw(ctx)?;
        let bounds = self.dimensions(ctx);
        let rectangle_mesh = graphics::MeshBuilder::new().rectangle(graphics::DrawMode::stroke(1.0), bounds, graphics::Color::BLACK)?.build(ctx)?;
        graphics::draw(ctx, &rectangle_mesh, (self.label.coordinates().clone(), 0.0, crate::NO_TEINT))?;

        Ok(())
    }

    pub fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) {
        let mut bounds = self.dimensions(ctx);
        let coordinates = self.label.coordinates();
        bounds.translate(coordinates.clone());
        if button == MouseButton::Left && bounds.contains([x, y]) {
            self.was_clicked = true;
        }
    }

    pub fn consume_was_clicked(&mut self) -> bool {
        let result = self.was_clicked;
        self.was_clicked = false;
        result
    }
}