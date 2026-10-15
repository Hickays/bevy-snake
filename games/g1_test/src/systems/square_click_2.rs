use bevy::prelude::*;
use g1_test::SquareColor;

pub fn click_square(
    click: On<PointerClick>,
    //
    mut squares: Query<(&mut Sprite, &mut SquareColor)>,
) {
    if let Ok((mut square, mut square_color)) = squares.get_mut(click.entity) {
        info!("点击了方块! (全局拾取)");
        square.color = square_color.next_color();
    }
}
