use fltk::{
    prelude::*,
    *,
};
use fltk_grid::Grid;

pub struct Greet {
    pub grid: Grid,
    pub name: input::Input,
    // btn: button::Button,
}

impl Greet {
    pub fn default() -> Self {
        let mut grid = Grid::default_fill();
        grid.set_layout(8, 7);

        let name = input::Input::default();

        let mut g = Self {
            grid,
            name,
        };
        g.fill();
        g
    }
    fn fill(&mut self) {
        let grid = &mut self.grid;
        grid.debug(true);

        let mut title = frame::Frame::default().with_label("Greeting!");
        title.set_frame(enums::FrameType::FlatBox);
        title.set_color(enums::Color::DarkBlue);
        title.set_label_color(enums::Color::White);
        title.set_label_size(40);
        grid.insert(&mut title, 0..5, 1);

        grid.insert(&mut frame::Frame::default().with_label("name"), 2..4, 1);

        grid.insert(&mut self.name, 2..4, 4);

    }
}


