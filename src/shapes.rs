use sdl2::pixels::Color;

trait ShapeRenderer {
    fn new(amount: u32) -> Self;
    fn iterate(frame: usize);
    fn render(canvas: &mut Canvas<Window>);
}

struct Circle {
    x: u32,
    y: u32,
    r: u32,
    c: Color,
}

impl Circle {
    fn default() -> Circle {
        Circle {
            x: -10,
            y: -10,
            r: 1,
            c: Color::RGB(0, 0, 0),
        }
    }
}

struct CircleRenderer {
    circles: Vec::new(),
}

impl ShapeRenderer for CircleRenderer {
    fn new(amount: u32) -> CircleRenderer {
        let mut circles = Vec::new();

        for _ in 0..amount {
            circles.push(Circle::default());
        }

        CircleRenderer {
            circles: circles,
        }
    }

    fn iterate(frame: usize) {

    }

    fn render(canvas: &mut Canvas<Window>) {
        for circle in circles.iter() {
            canvas.filled_circle(circle.x, circle.y, circle.r, circle.c);
        }
    }
}