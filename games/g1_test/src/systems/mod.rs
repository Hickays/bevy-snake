mod square_move;
pub use square_move::move_square;

mod square_click_1;
mod square_click_2;

pub use square_click_1::click_square as multi_square_click;
pub use square_click_2::click_square as picking_square_click;
