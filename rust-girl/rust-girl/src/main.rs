use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Man and Woman Animation", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut x = 0.0;
    let mut direction = 1.0;

    while let Some(event) = window.next() {
        if let Some(_) = event.update_args() {
            x += 2.0 * direction;
            if x > 600.0 || x < 0.0 {
                direction *= -1.0;
            }
        }

        window.draw_2d(&event, |ctx, g, _| {
            clear([1.0; 4], g); // White background
            draw_ground(ctx, g);
            draw_woman(ctx, g);
            draw_man(ctx, g, x);
            draw_rose(ctx, g, x);
        });
    }
}

fn draw_ground(ctx: Context, g: &mut G2d) {
    line([0.0, 0.0, 0.0, 1.0], 2.0, [0.0, 400.0, 800.0, 400.0], ctx.transform, g);
}

fn draw_man(ctx: Context, g: &mut G2d, x: f64) {
    let x = 100.0 + x;
    ellipse([0.0, 0.0, 0.0, 1.0], [x - 15.0, 285.0, 30.0, 30.0], ctx.transform, g); // head
    line([0.0, 0.0, 0.0, 1.0], 2.0, [x, 315.0, x, 360.0], ctx.transform, g); // body
    line([0.0, 0.0, 0.0, 1.0], 2.0, [x, 360.0, x - 10.0, 400.0], ctx.transform, g); // left leg
    line([0.0, 0.0, 0.0, 1.0], 2.0, [x, 360.0, x + 10.0, 400.0], ctx.transform, g); // right leg
    line([0.0, 0.0, 0.0, 1.0], 2.0, [x, 330.0, x + 15.0, 340.0], ctx.transform, g); // arm
}

fn draw_woman(ctx: Context, g: &mut G2d) {
    let x = 400.0;
    ellipse([0.0, 0.0, 0.0, 1.0], [x - 15.0, 285.0, 30.0, 30.0], ctx.transform, g); // head
    polygon(
        [0.8, 0.2, 0.5, 1.0],
        &[[x - 20.0, 315.0], [x + 20.0, 315.0], [x, 365.0]],
        ctx.transform,
        g,
    );
    line([0.0, 0.0, 0.0, 1.0], 2.0, [x - 10.0, 365.0, x - 10.0, 400.0], ctx.transform, g); // leg
    line([0.0, 0.0, 0.0, 1.0], 2.0, [x + 10.0, 365.0, x + 10.0, 400.0], ctx.transform, g); // leg
}

fn draw_rose(ctx: Context, g: &mut G2d, x: f64) {
    let x = 130.0 + x;
    line([0.0, 1.0, 0.0, 1.0], 2.0, [x, 323.0, x, 335.0], ctx.transform, g); // stem
    ellipse([1.0, 0.0, 0.0, 1.0], [x - 2.0, 319.0, 4.0, 4.0], ctx.transform, g); // rose
}
