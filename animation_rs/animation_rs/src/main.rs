use std::f32;
use std::thread;
use std::time::Duration;

struct Renderer {
    a: f32,
    b: f32,
    c: f32,
    cube_width: f32,
    width: usize,
    height: usize,
    z_buffer: Vec<f32>,
    buffer: Vec<char>,
    background_ascii_code: char,
    distance_from_cam: f32,
    horizontal_offset: f32,
    k1: f32,
    increment_speed: f32,
}

impl Renderer {
    fn new() -> Self {
        let width = 160;
        let height = 44;
        Renderer {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            cube_width: 20.0,
            width,
            height,
            z_buffer: vec![0.0; width * height],
            buffer: vec![' '; width * height],
            background_ascii_code: ' ',
            distance_from_cam: 100.0,
            horizontal_offset: 0.0,
            k1: 40.0,
            increment_speed: 0.6,
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.a.sin() * self.b.sin() * self.c.cos()
            - k * self.a.cos() * self.b.sin() * self.c.cos()
            + j * self.a.cos() * self.c.sin()
            + k * self.a.sin() * self.c.sin()
            + i * self.b.cos() * self.c.cos()
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.a.cos() * self.c.cos()
            + k * self.a.sin() * self.c.cos()
            - j * self.a.sin() * self.b.sin() * self.c.sin()
            + k * self.a.cos() * self.b.sin() * self.c.sin()
            - i * self.b.cos() * self.c.sin()
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        k * self.a.cos() * self.b.cos() - j * self.a.sin() * self.b.cos() + i * self.b.sin()
    }

    fn calculate_for_surface(&mut self, cube_x: f32, cube_y: f32, cube_z: f32, ch: char) {
        let x = self.calculate_x(cube_x, cube_y, cube_z);
        let y = self.calculate_y(cube_x, cube_y, cube_z);
        let z = self.calculate_z(cube_x, cube_y, cube_z) + self.distance_from_cam;

        let ooz = 1.0 / z;

        let xp = (self.width as f32 / 2.0 + self.horizontal_offset + self.k1 * ooz * x * 2.0) as i32;
        let yp = (self.height as f32 / 2.0 + self.k1 * ooz * y) as i32;

        if xp >= 0 && xp < self.width as i32 && yp >= 0 && yp < self.height as i32 {
            let idx = xp as usize + yp as usize * self.width;
            if idx < self.width * self.height && ooz > self.z_buffer[idx] {
                self.z_buffer[idx] = ooz;
                self.buffer[idx] = ch;
            }
        }
    }

    fn render_cube(&mut self, cube_width: f32, horizontal_offset: f32) {
        self.cube_width = cube_width;
        self.horizontal_offset = horizontal_offset;

        let mut cube_x = -cube_width;
        while cube_x < cube_width {
            let mut cube_y = -cube_width;
            while cube_y < cube_width {
                self.calculate_for_surface(cube_x, cube_y, -cube_width, '@');
                self.calculate_for_surface(cube_width, cube_y, cube_x, '$');
                self.calculate_for_surface(-cube_width, cube_y, -cube_x, '~');
                self.calculate_for_surface(-cube_x, cube_y, cube_width, '#');
                self.calculate_for_surface(cube_x, -cube_width, -cube_y, ';');
                self.calculate_for_surface(cube_x, cube_width, cube_y, '+');
                
                cube_y += self.increment_speed;
            }
            cube_x += self.increment_speed;
        }
    }

    fn clear_buffers(&mut self) {
        for i in 0..self.width * self.height {
            self.buffer[i] = self.background_ascii_code;
            self.z_buffer[i] = 0.0;
        }
    }

    fn display(&self) {
        print!("\x1b[H"); // Move cursor to home position
        for k in 0..self.width * self.height {
            if k % self.width == 0 && k != 0 {
                print!("\n");
            } else {
                print!("{}", self.buffer[k]);
            }
        }
    }

    fn update_rotation(&mut self) {
        self.a += 0.05;
        self.b += 0.05;
        self.c += 0.01;
    }

    fn run(&mut self) {
        // Clear screen
        print!("\x1b[2J");
        
        loop {
            self.clear_buffers();

            // First cube
            self.render_cube(20.0, -2.0 * 20.0);

            // Second cube
            self.render_cube(10.0, 1.0 * 10.0);

            // Third cube
            self.render_cube(5.0, 8.0 * 5.0);

            self.display();
            self.update_rotation();

            thread::sleep(Duration::from_millis(16)); // ~60 FPS (16ms = 8000 * 2 microseconds)
        }
    }
}

fn main() {
    let mut renderer = Renderer::new();
    renderer.run();
}