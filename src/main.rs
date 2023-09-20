use macroquad::prelude::*;
#[macroquad::main("Game of life")]
async fn main() {
    //There will be 20 by 20 grid of cells
    let mut state_arr: [u8; 400] = [0; 400]; //I may be should use array of enums
    //Set initial conditions
    state_arr[25] = 1;
    state_arr[66] = 1;
    state_arr[24] = 1;
    state_arr[65] =1;
    let mut cycle_counter: u8 = 0;
    loop {
        clear_background(WHITE);

        //Do stuff here
        if cycle_counter == 15 {
            let mut s_arr = state_arr.clone();
            for i in 0..399{
                if get_neigthbour_sum(&state_arr, i)>=2{
                    s_arr[i] = 1;
                }else {
                    s_arr[i]=0;
                }

            }
            state_arr = s_arr;
            cycle_counter=0;
        }
        cycle_counter+=1;
        //Draw
        for i in 0..400 {
            draw_cell(i, state_arr[i])
        }
        next_frame().await
    }
}

fn draw_cell(index: usize, state: u8) {
    let cell_color = match state {
        0 => GREEN,
        1 => BLUE,
        _ => RED,
    };
    draw_rectangle(
        26.0 * ((index % 20) as f32),
        26.0 * ((index / 20) as f32),
        25.0,
        25.0,
        cell_color,
    );
}
//Todo finish this function
//Kernel:
// #
//#O#
// #
fn get_neigthbour_sum(state_arr: &[u8], indx: usize) -> u8 {
    let mut sum: u8 = 0;
    //col
    if indx >= 20 {
        sum += state_arr[indx - 20];
    }
    if indx < 380 {
        sum += state_arr[indx + 20]
    }
    //row
    if indx % 20 != 0 {
        sum += state_arr[indx - 1];
    }
    if indx % 20 != 19 {
        sum += state_arr[indx + 1];
    }
    sum
}
