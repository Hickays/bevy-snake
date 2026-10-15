//! 点击变色功能: 方块被点击时在调色板里循环切换颜色

use bevy::prelude::*;

/// 挂上 `SquareColor` 时, 立刻把方块颜色刷成当前颜色 (即调色板的第一个颜色)
pub fn init_square_color(
    add: On<Add<SquareColor>>,
    mut squares: Query<(&SquareColor, &mut Sprite)>,
) {
    if let Ok((color, mut sprite)) = squares.get_mut(add.entity) {
        sprite.color = color.current();
    }
}

/// 点击方块时切换到下一个颜色
pub fn click_square(click: On<PointerClick>, mut squares: Query<(&mut Sprite, &mut SquareColor)>) {
    if let Ok((mut sprite, mut color)) = squares.get_mut(click.entity) {
        sprite.color = color.next_color();
    }
}

/// 方块颜色: 方块颜色的唯一来源
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

    /// 当前颜色
    pub fn current(&self) -> Color {
        self.color_list[self.current_color_idx]
    }

    /// 前进到下一个颜色, 并把它作为当前颜色返回
    pub fn next_color(&mut self) -> Color {
        self.current_color_idx = (self.current_color_idx + 1) % self.color_list.len();
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_applies_current_color() {
        let mut app = App::new();
        app.add_observer(init_square_color);

        let expected = SquareColor::new().current();
        let entity = app
            .world_mut()
            .spawn((Sprite::default(), SquareColor::new()))
            .id();

        assert_eq!(app.world().get::<Sprite>(entity).unwrap().color, expected);
    }

    #[test]
    fn next_color_advances_then_returns() {
        let mut color = SquareColor::new();
        let first = color.current();

        let next = color.next_color();

        assert_ne!(next, first, "第一次切换必须换成另一个颜色");
        assert_eq!(color.current(), next);
    }
}
