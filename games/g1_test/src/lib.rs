use bevy::prelude::*;

#[derive(Component)]
pub struct EventSquare;

pub fn build_square(size: u32, color: Color) -> Sprite {
    Sprite {
        color,
        custom_size: Some(Vec2::new(size as f32, size as f32)),
        ..Default::default()
    }
}

#[derive(Component)]
pub struct SquareColor {
    color_list: Vec<Color>,
    current_color_idx: usize,
}

impl SquareColor {
    pub fn new() -> SquareColor {
        SquareColor {
            color_list: vec![
                Color::srgb_u32(0x_FF6188),
                Color::srgb_u32(0x_FC9867),
                Color::srgb_u32(0x_FFD866),
                Color::srgb_u32(0x_A9DC76),
                Color::srgb_u32(0x_78DCE8),
                Color::srgb_u32(0x_AB9DF2),
            ],
            current_color_idx: 0,
        }
    }

    pub fn next_color(&mut self) -> Color {
        let len = self.color_list.len();
        let new_index = (self.current_color_idx + 1) % len;

        self.current_color_idx = new_index;

        self.color_list.get(self.current_color_idx).unwrap().clone()
    }

    pub fn current(&self) -> Color {
        self.color_list[self.current_color_idx]
    }
}
