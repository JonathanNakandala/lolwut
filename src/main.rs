extern crate rand;

use std::char::from_u32;
use std::f32::consts::PI;

use rand::Rng;

fn main() {
    let console_dimensions = [66, 49];
    let squares_per_row = 8;
    let squares_per_col = 12;

    fn draw_schotter(usizeimensions: i32, squares_per_row: i32, squares_per_col: i32) {
        let console_dimensions = [66, 49];
        let canvas_width = usizeimensions * 2;
        let padding = if canvas_width > 4 { 2 } else { 0 };
        let square_side: f32 = (canvas_width - padding * 2) as f32 / squares_per_row as f32;
        let canvas_height = (square_side * squares_per_col as f32 * padding as f32 * 2.0) as i32;
        let rand_max = 2_147_483_647;
        let mut canvas = vec![0; (canvas_width * canvas_height) as usize];
        let mut output_canvas: Vec<Vec<char>> = vec![vec![Default::default(); 0]; 0];
        for y in 0..(squares_per_col - 1) {
            for x in 0..(squares_per_row - 1) {
                let mut sx: i32 = x * square_side as i32 + square_side as i32 / 2 + padding;
                let mut sy: i32 = y * square_side as i32 + square_side as i32 / 2 + padding;
                //println!("x:{} y: {}", sx, sy);

                let mut angle: f32 = 0.0;
                if y > 1 {
                    let mut r1: f32 = rand::thread_rng().gen_range(0, rand_max) as f32
                        / rand_max as f32
                        / squares_per_col as f32
                        * y as f32;
                    let mut r2: f32 = rand::thread_rng().gen_range(0, rand_max) as f32
                        / rand_max as f32
                        / squares_per_col as f32
                        * y as f32;
                    let mut r3: f32 = rand::thread_rng().gen_range(0, rand_max) as f32
                        / rand_max as f32
                        / squares_per_col as f32
                        * y as f32;

                    if rand::thread_rng().gen_range(0, 1) != 0 {
                        r1 = -r1;
                    }
                    if rand::thread_rng().gen_range(0, 1) != 0 {
                        r2 = -r2;
                    }
                    if rand::thread_rng().gen_range(0, 1) != 0 {
                        r3 = -r3;
                    }
                    angle = r1;
                    sx += (r2 * square_side) as i32 / 3;
                    sy += (r3 * square_side) as i32 / 3;
                }
                draw_square(&mut canvas, sx, sy, square_side, angle, 1);
            }
        }
        //println!("{:?}", canvas);
        render_canvas(&canvas, &mut output_canvas);
        print_generated_braille(&output_canvas);
    }

    fn draw_square(
        mut canvas: &mut Vec<i32>,
        x: i32,
        y: i32,
        mut size: f32,
        angle: f32,
        color: i32,
    ) {
        let mut px: [i32; 4] = [Default::default(); 4];
        let mut py: [i32; 4] = [Default::default(); 4];
        size /= std::f64::consts::SQRT_2 as f32;
        let size_int = size.round() as i32;

        let mut k = PI / 4.0 + angle;
        for j in 0..3 {
            px[j] = k.sin().round() as i32 * size_int + x;
            py[j] = k.cos().round() as i32 * size_int + y;
            //println!("px: {}, py: {}", px[j], py[j]);
            k += PI / 2.0;
        }

        //println!("Wooo!: {}", k);

        for j in 0..3 {
            let x2 = px[(j + 1) % 4];
            let y2 = py[(j + 1) % 4];
            //println!("x2: {}, y2: {}", x2, y2);
            draw_line(&mut canvas, px[j], py[j], x2, y2, color);
        }
    }

    fn draw_line(
        mut canvas: &mut Vec<i32>,
        mut x1: i32,
        mut y1: i32,
        x2: i32,
        y2: i32,
        color: i32,
    ) -> &mut std::vec::Vec<i32> {
        let dx: i32 = (x2 - x1).abs();
        let dy: i32 = (y2 - y1).abs();
        let sx: i32 = if x1 < x2 { 1 } else { -1 };
        let sy: i32 = if y1 < y2 { 1 } else { -1 };
        let mut err: i32 = dx - dy;
        let mut e2: i32 = 0;
        loop {
            draw_pixel(&mut canvas, x1, y1, color);
            if x1 == x2 && y1 == y2 {
                break;
            };
            e2 = err * 2;
            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }
            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
        canvas
    }

    fn draw_pixel(mut canvas: &mut Vec<i32>, x: i32, y: i32, color: i32) {
        let canvas_width = 66 * 2;
        let canvas_height = 390;

        let index = x + (y * canvas_width);
        //println!(": {}", index);
        if x < 0 || x >= canvas_width || y < 0 || y >= canvas_height {
            //println!("OR TRUE : x: {}, Y {}", x, y);
            return;
        }
        //println!("x: {}, Y {}", x, y);
        canvas[(x + (y * canvas_width)) as usize] = color
    }

    fn get_pixel(canvas: &Vec<i32>, x: i32, y: i32) -> i32 {
        let canvas_width = 66 * 2;

        if x < 0 || x >= canvas_width || y < 0 || y >= canvas_width {
            return 0;
        }
        return canvas[(x + (y * canvas_width)) as usize];
    }

    fn integer_is_one(value: i32) -> bool {
        value == 1
    }

    fn render_canvas(canvas: &Vec<i32>, output_canvas: &mut Vec<Vec<char>>) {
        let canvas_width = 66 * 2;
        let canvas_height = 390;

        for y in (0..canvas_height - 1).step_by(4) {
            let mut braille_row: Vec<char> = Vec::new();
            for x in (0..canvas_width - 1).step_by(2) {
                let mut byte: u8 = 0;

                if integer_is_one(get_pixel(canvas, x, y)) {
                    byte |= 1 << 0
                };
                if integer_is_one(get_pixel(canvas, x, y + 1)) {
                    byte |= 1 << 1
                };
                if integer_is_one(get_pixel(canvas, x, y + 2)) {
                    byte |= 1 << 2
                };
                if integer_is_one(get_pixel(canvas, x + 1, y)) {
                    byte |= 1 << 3
                };
                if integer_is_one(get_pixel(canvas, x + 1, y + 1)) {
                    byte |= 1 << 4
                };
                if integer_is_one(get_pixel(canvas, x + 1, y + 2)) {
                    byte |= 1 << 5
                };
                if integer_is_one(get_pixel(canvas, x, y + 3)) {
                    byte |= 1 << 6
                };
                if integer_is_one(get_pixel(canvas, x, y + 3)) {
                    byte |= 1 << 7
                };
                translate_to_braille(byte, &mut braille_row);
            }
            //println!("Here's the row!!!!");
            //println!("{:?}", braille_row);
            output_canvas.push(braille_row);
        }
    }

    fn translate_to_braille(byte: u8, braille_row: &mut Vec<char>) {
        let code: u32 = 0x2800 + byte as u32;
        let unicode_char = from_u32(code as u32).unwrap();
        //print!("{}", unicode_char);
        braille_row.push(unicode_char);
    }

    fn print_generated_braille(output_canvas: &Vec<Vec<char>>) {
        let console_dimensions = [66, 49];
        let mut print_row: Vec<&[char]> = Vec::new();
        for row in output_canvas {
            print_row.push(row);
        }

        let slice: &[&[char]] = &print_row;
        for row in slice {
            for col in *row {
                print!("{}", col)
            }
            println!();
        }

        println!(
            "Width: {} Height: {}",
            console_dimensions[0], console_dimensions[1]
        );
        println!("\nGeorg Nees - schotter, plotter on paper, 1968. Jono ver. ");
    }

    draw_schotter(
        console_dimensions[0] as i32,
        squares_per_row,
        squares_per_col,
    );
}
