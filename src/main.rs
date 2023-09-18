use macroquad::prelude::*;
#[macroquad::main("Game of life")]
async fn main() {
    //There will be 20 by 20 grid of cells
    let mut state_arr:[u8;400] = [0;400];//I may be should use array of enums
    state_arr[25]=1;
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
    let cell_color = match state{
        0=>GREEN,
        1=>BLUE,
        3=>RED,
        _=>RED
    };
    draw_rectangle(26.0*((index%20) as f32), 26.0*((index/20) as f32), 25.0, 25.0, cell_color);
}
//Todo finish this function
fn get_neigthbour_sum(state_arr:&[u8], indx:usize)->u8{
    let mut sum:u8 =0;
    if indx>=20{
        sum +=state_arr[indx-20];
    }
    sum
}
