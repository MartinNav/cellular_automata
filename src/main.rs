use macroquad::prelude::*;
#[macroquad::main("Game of life")]
async fn main() {
    //There will be 10 by 10 grid of cells
    let mut state_arr:[u8;400] = [0;400];//I may be should use array of enums
    loop {
   clear_background(WHITE);

//Do stuff here

for i in 0..400{
    draw_cell(i, state_arr[i])
}
   next_frame().await
}
}

fn draw_cell(index:usize, state:u8){
    draw_rectangle(26.0*((index%20) as f32), 26.0*((index/20) as f32), 25.0, 25.0, RED);
}
