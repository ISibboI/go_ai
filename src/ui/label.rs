use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Label {
    text: graphics::Text,
    coordinates: graphics::mint::Point2<f32>,
}

impl Label {
    pub fn new(text: &str, coordinates: graphics::mint::Point2<f32>) -> Self {
        Self {text: graphics::Text::new(text), coordinates}
    }

    pub fn set_text(&mut self, text: &str) {
        if text == self.text.contents() {
            return;
        }

        self.text = graphics::Text::new(text);
    }

    pub fn text(&self) -> &graphics::Text {
        &self.text
    }

    pub fn coordinates(&self) -> &graphics::mint::Point2<f32> {
        &self.coordinates
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::queue_text(ctx, &self.text, self.coordinates, Some(graphics::Color::BLACK));
        Ok(())
    }
}