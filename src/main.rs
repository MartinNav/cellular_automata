use macroquad::prelude::*;
#[macroquad::main("Game of life")]
async fn main() {
    loop {
   clear_background(WHITE);

//Do stuff here

   next_frame().await
}
}
