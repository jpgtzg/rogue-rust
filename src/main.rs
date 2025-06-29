const WIDTH: usize = 25;
const HEIGHT: usize = 25;

fn main() {
    let mut screen: Vec<Vec<String>> = vec![vec![String::from("."); WIDTH]; HEIGHT];

    random_walk(&mut screen);

    for row in screen.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

fn random_walk(screen: &mut Vec<Vec<String>>) {
    let mut x = WIDTH / 2;
    let mut y = HEIGHT / 2;
    let mut current_direction = (rand::random::<i32>() % 4).abs();
    let mut tunnel_length = get_tunnel_length();
    let mut steps_in_current_direction = 0;

    for _ in 0..100 {
        // Change direction when tunnel length is reached
        if steps_in_current_direction >= tunnel_length {
            current_direction = (rand::random::<i32>() % 4).abs();
            tunnel_length = get_tunnel_length();
            steps_in_current_direction = 0;
        }

        modify_position(&mut x, &mut y, current_direction);
        steps_in_current_direction += 1;

        if x >= WIDTH || y >= HEIGHT {
            break;
        }

        screen[y][x] = String::from("X");
    }
}

fn modify_position(x: &mut usize, y: &mut usize, direction: i32) {
    match direction {
        0 => if *x < WIDTH - 1 { *x += 1 },
        1 => if *x > 0 { *x -= 1 },
        2 => if *y < HEIGHT - 1 { *y += 1 },
        3 => if *y > 0 { *y -= 1 },
        _ => {}
    }
}

fn get_tunnel_length() -> usize {
    let length = (rand::random::<u32>() % 5 + 1) as usize;
    length
}