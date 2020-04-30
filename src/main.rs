extern crate rand;

use rand::Rng;
use std::char::from_u32;
use std::f32::consts::PI;
fn main() {
    let console_dimensions = [66, 49];
    let squares_per_row = 8;
    let squares_per_col = 12;

    let mut array: Vec<Vec<char>> =
        vec![vec![Default::default(); console_dimensions[0]]; console_dimensions[1]];
    let something = '\u{2813}';
    print!("{}", something);

    fn drawSchotter(usizeimensions: i32, squares_per_row: i32, squares_per_col: i32) {
        let canvas_width = usizeimensions * 2;
        let padding = if canvas_width > 4 { 2 } else { 0 };
        let square_side: f32 = (canvas_width - padding * 2) as f32 / squares_per_row as f32;
        let canvas_height = (square_side * squares_per_col as f32 * padding as f32 * 2.0) as i32;
        let RAND_MAX = 2147483647;
        let mut canvas = vec![0; (0) as usize];
        for y in 0..(squares_per_col - 1) {
            for x in 0..(squares_per_row - 1) {
                let mut sx: i32 = x * square_side as i32 + square_side as i32 / 2 + padding;
                let mut sy: i32 = y * square_side as i32 + square_side as i32 / 2 + padding;
                //println!("x:{} y: {}", sx, sy);

                let mut angle: f32 = 0.0;
                if y > 1 {
                    let mut r1: f32 = rand::thread_rng().gen_range(0, RAND_MAX) as f32
                        / RAND_MAX as f32
                        / squares_per_col as f32
                        * y as f32;
                    let mut r2: f32 = rand::thread_rng().gen_range(0, RAND_MAX) as f32
                        / RAND_MAX as f32
                        / squares_per_col as f32
                        * y as f32;
                    let mut r3: f32 = rand::thread_rng().gen_range(0, RAND_MAX) as f32
                        / RAND_MAX as f32
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
                drawSquare(&mut canvas, sx, sy, square_side, angle, 1);
            }
        }
        println!("{:?}", canvas);
    }

    fn createCanvas(canvas_width: i32, canvas_height: i32, color: i32) {}

    fn drawSquare(
        mut canvas: &mut Vec<i32>,
        x: i32,
        y: i32,
        mut size: f32,
        angle: f32,
        color: i32,
    ) {
        let mut px: [i32; 4] = [Default::default(); 4];
        let mut py: [i32; 4] = [Default::default(); 4];
        size = size / 1.4142135623;
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
            println!("x2: {}, y2: {}", x2, y2);
            drawLine(&mut canvas, px[j], py[j], x2, y2, color);
        }
    }

    fn drawLine(
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
        let mut e2: i32 = err;
        let mut mod_canvas = vec![0; (0) as usize];
        mod_canvas = canvas.clone();
        loop {
            mod_canvas = drawPixel(mod_canvas, x1, y1, color);
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
        return canvas;
    }

    fn drawPixel(mut canvas: &mut Vec<i32>, x: i32, y: i32, color: i32) -> &mut std::vec::Vec<i32> {
        //println!("{},{}", x, y);
        let canvas_width = 66 * 2;
        let canvas_height = 390;

        if (x < 0 || x >= canvas_width || y < 0 || y >= canvas_height) {
            canvas.push(color);
        }
        //println!("{:?}", mod_canvas);
        return canvas;
    }

    fn pixel_to_braille(byte: u8) -> i32 {
        return 2;
    }

    let mut cnt = 2813;
    for row in &mut array {
        // row: &mut [u8; 3]
        for col in row {
            // col: &mut u8
            cnt += 1;
            let thecharacter = from_u32(0x2800 + 4).unwrap();
            *col = thecharacter;
        }
    }

    let mut print_row: Vec<&[char]> = Vec::new();
    for row in &array {
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

    drawSchotter(
        console_dimensions[0] as i32,
        squares_per_row,
        squares_per_col,
    );
}
