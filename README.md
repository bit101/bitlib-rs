# bitlib-rs
A drawing, color, geometry and math library geared towards creating graphics and animation with cairo-rs

Minimal example:

Cargo.toml
---

    [package]
    name = "minimal_example"
    version = "0.1.0"
    authors = ["Your Name <you@your_email.com>"]

    [dependencies]
    bitlib = { git = "https://github.com/bit101/bitlib-rs.git" }

main.rs
---

    extern crate bitlib;

    use bitlib::canvas::Canvas;
    use bitlib::canvas::BitContext;
    use bitlib::color::Color;
    use bitlib::random::Random;
    use bitlib::file::open;

    fn main() {
        let canvas = Canvas::create(600.0, 600.0);
        let context = canvas.get_context();
        let mut rand = Random::new();

        for _i in 0..1000 {
            let x = rand.float(0.0, 600.0);
            let y = rand.float(0.0, 600.0);
            let r = rand.float(5.0, 50.0);
            context.set_source_color(&Color::random_rgb());
            context.fill_circle(x, y, r);
        }
        canvas.write("output.png");
        open("output.png");
    }
