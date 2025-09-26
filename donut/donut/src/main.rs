use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut z = [0.0_f32; 1760];
    let mut screen = [' '; 1760];
    
    // Clear terminal
    print!("\x1b[2J");
    
    loop {
        // Reset buffers
        screen = [' '; 1760];
        z = [0.0; 1760];
        
        // Main rendering loop
        for j in (0..628).map(|j| j as f32 * 0.01) {
            for i in (0..628).map(|i| i as f32 * 0.02) {
                let sin_i = i.sin();
                let cos_j = j.cos();
                let sin_j = j.sin();
                let cos_i = i.cos();
                let sin_a = a.sin();
                let sin_b = b.sin();
                let cos_a = a.cos();
                let cos_b = b.cos();
                
                let h = cos_j + 2.0;
                let d = 1.0 / (sin_i * h * sin_a + sin_j * cos_a + 5.0);
                let t = sin_i * h * cos_a - sin_j * sin_a;
                
                let x = (40.0 + 30.0 * d * (cos_i * h * cos_b - t * sin_b)) as i32;
                let y = (12.0 + 15.0 * d * (cos_i * h * sin_b + t * cos_b)) as i32;
                
                let o = x + 80 * y;
                let n = (8.0 * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b - sin_i * cos_j * sin_a - sin_j * cos_a - cos_i * cos_j * sin_b)) as i32;
                
                if y > 0 && y < 22 && x > 0 && x < 80 && d > z[o as usize] {
                    z[o as usize] = d;
                    let luminance_index = if n > 0 { n } else { 0 };
                    screen[o as usize] = ".,-~:;=!*#$@".chars().nth(luminance_index as usize).unwrap_or('@');
                }
            }
        }
        
        // Print the frame
        print!("\x1b[H");
        for k in 0..1760 {
            if k % 80 != 0 {
                print!("{}", screen[k]);
            } else {
                println!();
            }
        }
        
        // Update rotation angles
        a += 0.04;
        b += 0.02;
        
        // Add small delay to control animation speed
        thread::sleep(Duration::from_millis(20));
        io::stdout().flush().unwrap();
    }
}

