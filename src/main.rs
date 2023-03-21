use std::{io::{self, Write}, fs::File};

#[derive(Copy, Clone, Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn convert(&self) -> (u8, u8, u8) {
        (
            (self.r * 255f32) as u8,
            (self.g * 255f32) as u8,
            (self.b * 255f32) as u8
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

fn main() -> io::Result<()> {
    const WIDTH: usize = 300;
    const HEIGHT: usize = 200;

    let img_buffer = [[Color {
        r: 1.0,
        g: 0.1,
        b: 0.5,
    }; WIDTH]; HEIGHT];


    for (j, row) in img_buffer.iter().enumerate() {
        for (i, cell) in rows.iter().enumerate() {
            let x = i as f32 - ((WIDTH as f32) / 2f32);
            let y = j as f32 - ((HEIGHT as f32) / 2f32);

            let p = Point {x, y, z: 0f32};

            let camera = Point (x: 0f32, y: 0f32, z: -1f32);

        }
    }

    write_ppm(&img_buffer);

    Ok(())
}

fn write_ppm(img_buffer: &[[Color; 300]; 200]) {

    let fstr = img_buffer
        .iter()
        .map(|row| {
            row.iter().map(|cell| {
                let (r,g,b) = cell.convert();
                [
                    r.to_string(),
                    g.to_string(),
                    b.to_string()
                ].join(" ")
            })
        })
        .flatten()
        .fold(String::new(), |a, b| a + &b + "\n");

    // println!("{fstr}");

    let mut file = File::create("test.ppm").expect("File could not be opened");
    file.write_all(fstr.as_bytes()).expect("File could not be written");
}
