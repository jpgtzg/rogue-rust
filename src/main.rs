use rand::Rng;

const WIDTH: usize = 10;
const HEIGHT: usize = 25;

const MAX_ROOM_WIDTH: usize = 6;
const MAX_ROOM_HEIGHT: usize = 6;
const MIN_ROOM_WIDTH: usize = 3;
const MIN_ROOM_HEIGHT: usize = 3;

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[" . "; WIDTH]; HEIGHT];

    generate_room(&mut screen);
    display_screen(&screen);
}

fn display_screen(screen: &[[&str; WIDTH]; HEIGHT]) {
    for row in screen.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

fn generate_room(screen: &mut [[&str; WIDTH]; HEIGHT]) {

    let x_i = rand::rng().random_range(0..WIDTH);
    let y_i = rand::rng().random_range(0..HEIGHT);

    let mut width = rand::rng().random_range(MIN_ROOM_WIDTH..MAX_ROOM_WIDTH);
    let mut height = rand::rng().random_range(MIN_ROOM_HEIGHT..MAX_ROOM_HEIGHT);

    if x_i + width > WIDTH {
        width = WIDTH - x_i;
    }
    if y_i + height > HEIGHT {
        height = HEIGHT - y_i;
    }

    for x in x_i..x_i + width {
        for y in y_i..y_i + height {
            screen[y][x] = " # ";
        }
    }

    println!("x_i: {}", x_i);
    println!("y_i: {}", y_i);
    println!("width: {}", width);
    println!("height: {}", height);
}
