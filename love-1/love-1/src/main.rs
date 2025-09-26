fn main() {
    let mut x = 10;
    let mut y = 7;
    let mut z = 5;
    let mut w = 7;
    let mut k = 11;
    let mut l = 23;
    let mut m = 17;
    let mut n = 5;
    let mut o = 8;
    let mut r = 16;
    let mut s = 6;

    println!("\n\n");

    // Top border
    for _t in 1..=2 {
        print!("   ");
        for _v in 1..=76 {
            print!("=");
        }
        println!();
    }
    println!();

    // First pattern section
    for a in 1..=3 {
        print!("     ");
        
        // First block of asterisks
        for _b in 1..=9 {
            print!("*");
        }
        
        // Spaces between first and second blocks
        for _c in 1..=x {
            print!(" ");
        }
        x -= 1;
        
        // Second block of asterisks
        for _d in 1..=y {
            print!("*");
        }
        y += 2;
        
        // Spaces between second and third blocks
        for _e in 1..=z {
            print!(" ");
        }
        z -= 2;
        
        // Third block of asterisks
        for _f in 1..=w {
            print!("*");
        }
        w += 2;
        
        // Additional spaces
        for g in 3..=3 {
            if g >= a {
                print!(" ");
            }
        }
        
        // More spaces
        for _h in 1..=7 {
            print!(" ");
        }
        
        // Fourth block of asterisks
        for _i in 1..=9 {
            print!("*");
        }
        
        // Spaces between fourth and fifth blocks
        for _p in 1..=6 {
            print!(" ");
        }
        
        // Fifth block of asterisks
        for _q in 1..=9 {
            print!("*");
        }
        println!();
    }

    // Middle pattern section
    for a in 1..=9 {
        print!("     ");
        
        // Spaces before first block
        for _b in 1..=3 {
            print!(" ");
        }
        
        // First block of asterisks
        for _c in 1..=3 {
            print!("*");
        }
        
        // Spaces between first and second blocks
        for _d in 1..=k {
            print!(" ");
        }
        k += 1;
        
        // Second block of asterisks
        for _e in 1..=l {
            print!("*");
        }
        l -= 2;
        
        // Additional spaces
        for _g in 1..=a {
            print!(" ");
        }
        
        // More spaces
        for _h in 1..=7 {
            print!(" ");
        }
        
        // Spaces before third block
        for _i in 1..=3 {
            print!(" ");
        }
        
        // Third block of asterisks
        for _j in 1..=3 {
            print!("*");
        }
        
        // Spaces between third and fourth blocks
        for _p in 1..=12 {
            print!(" ");
        }
        
        // Fourth block of asterisks
        for _q in 1..=3 {
            print!("*");
        }
        println!();
    }

    // Last pattern section
    for _a in 1..=3 {
        print!("     ");
        
        // First block of asterisks
        for _b in 1..=9 {
            print!("*");
        }
        
        // Spaces between first and second blocks
        for _c in 1..=m {
            print!(" ");
        }
        m += 1;
        
        // Second block of asterisks
        for _d in 1..=n {
            print!("*");
        }
        n -= 2;
        
        // Additional spaces
        for _g in 1..=o {
            print!(" ");
        }
        o += 1;
        
        // More spaces
        for _h in 1..=7 {
            print!(" ");
        }
        
        // Further spacing
        for _i in 1..=s {
            print!(" ");
        }
        s += 2;
        
        // Final block of asterisks
        for _q in 1..=r {
            print!("*");
        }
        r -= 4;
        println!();
    }

    println!();

    // Bottom border
    for _t in 1..=2 {
        print!("   ");
        for _v in 1..=76 {
            print!("=");
        }
        println!();
    }
    println!();
    println!("chez hongsen");
}