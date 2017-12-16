extern crate cairo;
extern crate rand;

use self::cairo::{Context, ImageSurface, Format};
use std::fs::File;
use color::Color;
use geom::point::Point;
use math::{ clamp, PI, TWO_PI, HALF_PI };

pub struct Canvas {
    pub width: f64,
    pub height: f64,
    context: Context,
    surface: ImageSurface,
}

impl Canvas {
    pub fn create(width: f64, height: f64) -> Canvas {
        let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32)
            .expect("couldn't create a surface, yo");
        let context = Context::new(&surface);
        Canvas { 
            width: width,
            height: height,
            surface: surface,
            context: context,
        }
    }

    pub fn write(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        self.surface.write_to_png(&mut file)
            .expect("Couldn't write to png");
    }

    pub fn get_context(&self) -> &Context {
        &self.context
    }
}

pub trait BitContext {
    fn clear_rgb(&self, r: f64, g: f64, b: f64);
    fn clear_color(&self, color: &Color);

    fn set_source_color(&self, color: &Color);

    fn fill_rectangle(&self, x: f64, y: f64, w: f64, h: f64);
    fn stroke_rectangle(&self, x: f64, y: f64, w: f64, h: f64);
    fn round_rectangle(&self, x: f64, y: f64, w: f64, h: f64, r: f64);
    fn stroke_round_rectangle(&self, x: f64, y: f64, w: f64, h: f64, r: f64);
    fn fill_round_rectangle(&self, x: f64, y: f64, w: f64, h: f64, r: f64);

    fn line(&self, x0: f64, y0: f64, x1: f64, y1: f64);

    fn circle(&self, x: f64, y: f64, r: f64);
    fn fill_circle(&self, x: f64, y: f64, r: f64);
    fn stroke_circle(&self, x: f64, y: f64, r: f64);

    fn ellipse(&self, x: f64, y: f64, xr: f64, yr: f64);
    fn fill_ellipse(&self, x: f64, y: f64, xr: f64, yr: f64);
    fn stroke_ellipse(&self, x: f64, y: f64, xr: f64, yr: f64);

    fn path(&self, points: &[Point]);
    fn fill_path(&self, points: &[Point]);
    fn stroke_path(&self, points: &[Point], close: bool);

	fn polygon(&self, x: f64, y: f64, r: f64, sides: i32, rotation: f64);
	fn stroke_polygon(&self, x: f64, y: f64, r: f64, sides: i32, rotation: f64);
	fn fill_polygon(&self, x: f64, y: f64, r: f64, sides: i32, rotation: f64);

	fn star(&self, x: f64, y: f64, r0: f64, r1: f64, points: i32, rotation: f64);
	fn stroke_star(&self, x: f64, y: f64, r0: f64, r1: f64, points: i32, rotation: f64);
	fn fill_star(&self, x: f64, y: f64, r0: f64, r1: f64, points: i32, rotation: f64);

    fn splat(&self, x: f64, y: f64, num_nodes: i32, radius: f64, inner_radius: f64, variation: f64);
    fn stroke_splat(&self, x: f64, y: f64, num_nodes: i32, radius: f64, inner_radius: f64, variation: f64);
    fn fill_splat(&self, x: f64, y: f64, num_nodes: i32, radius: f64, inner_radius: f64, variation: f64);

    fn fractal_line(&self, x1: f64, y1: f64, x2: f64, y2: f64, roughness: f64, iterations: i32);
    fn stroke_fractal_line(&self, x1: f64, y1: f64, x2: f64, y2: f64, roughness: f64, iterations: i32);

    fn heart(&self, x: f64, y: f64, w: f64, h: f64, r: f64);
    fn fill_heart(&self, x: f64, y: f64, w: f64, h: f64, r: f64);
    fn stroke_heart(&self, x: f64, y: f64, w: f64, h: f64, r: f64);

    fn grid(&self, x: f64, y: f64, w: f64, h: f64, xres: f64, yres: f64);

    fn points(&self, points: &[Point], radius: f64);

    fn stroke_curve_to(&self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64);
    fn quadratic_curve_to(&self, x0: f64, y0: f64, x1: f64, y1: f64);
    fn stroke_quadratic_curve_to(&self, x0: f64, y0: f64, x1: f64, y1: f64);
    fn multi_curve(&self, points: &[Point]);
    fn stroke_multi_curve(&self, points: &[Point]);
    fn multi_loop(&self, points: &[Point]);
    fn fill_multi_loop(&self, points: &[Point]);
    fn stroke_multi_loop(&self, points: &[Point]);
}

impl BitContext for Context{
    fn clear_rgb(&self, r: f64, g: f64, b: f64) {
        self.save();
        // todo: set identity transform
        self.set_source_rgb(r, g, b);
        self.paint();
        self.restore();
    }

    fn clear_color(&self, color: &Color) {
        self.clear_rgb(color.r, color.g, color.b);
    }

    fn set_source_color(&self, color: &Color) {
        self.set_source_rgba(color.r, color.g, color.b, color.a);
    }

    fn line(&self, x0: f64, y0: f64, x1: f64, y1: f64) {
        self.move_to(x0, y0);
        self.line_to(x1, y1);
        self.stroke();
    }

    fn fill_rectangle(&self, x: f64, y: f64, w: f64, h: f64) {
        self.rectangle(x, y, w, h);
        self.fill();
    }

    fn stroke_rectangle(&self, x: f64, y: f64, w: f64, h: f64) {
        self.rectangle(x, y, w, h);
        self.stroke();
    }

    fn round_rectangle(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.move_to(x + r, y);
        self.line_to(x + w - r, y);
        self.arc(x + w - r, y + r, r, -HALF_PI, 0.0);
        self.line_to(x + w, y + h - r);
        self.arc(x + w - r, y + h - r, r, 0.0, HALF_PI);
        self.line_to(x + r, y + h);
        self.arc(x + r, y + h - r, r, HALF_PI, PI);
        self.line_to(x, y + r);
        self.arc(x + r, y + r, r, PI, -HALF_PI);
    }

    fn stroke_round_rectangle(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.round_rectangle(x, y, w, h, r);
        self.stroke();
    }

    fn fill_round_rectangle(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.round_rectangle(x, y, w, h, r);
        self.fill();
    }

    fn circle(&self, x: f64, y: f64, r: f64) {
        self.arc(x, y, r, 0.0, TWO_PI);
    }

    fn fill_circle(&self, x: f64, y: f64, r: f64) {
        self.circle(x, y, r);
        self.fill();
    }

    fn stroke_circle(&self, x: f64, y: f64, r: f64) {
        self.circle(x, y, r);
        self.stroke();
    }

    fn ellipse(&self, x: f64, y: f64, xr: f64, yr: f64) {
        self.save();
        self.translate(x, y);
        self.scale(xr, yr);
        self.circle(0.0, 0.0, 1.0);
        self.restore();
    }

    fn fill_ellipse(&self, x: f64, y: f64, xr: f64, yr: f64) {
        self.ellipse(x, y, xr, yr);
        self.fill();
    }

    fn stroke_ellipse(&self, x: f64, y: f64, xr: f64, yr: f64) {
        self.ellipse(x, y, xr, yr);
        self.stroke();
    }

    fn path(&self, points: &[Point]) {
        for point in points.iter() {
            self.line_to(point.x, point.y);
        }
    }

    fn fill_path(&self, points: &[Point]) {
        self.path(points);
        self.fill();
    }

    fn stroke_path(&self, points: &[Point], close: bool) {
        self.path(points);
        if close {
            self.close_path();
        }
        self.stroke();
    }

	fn polygon(&self, x: f64, y: f64, r: f64, sides: i32, rotation: f64) {
        self.save();
        self.translate(x, y);
        self.rotate(rotation);
        self.move_to(r, 0.0);
        for i in 0..sides {
            let angle = TWO_PI / sides as f64 * i as f64;
            self.line_to(angle.cos() * r, angle.sin() * r);
        }
        self.line_to(r, 0.0);
        self.restore();
    }

	fn stroke_polygon(&self, x: f64, y: f64, r: f64, sides: i32, rotation: f64) {
        self.polygon(x, y, r, sides, rotation);
        self.stroke();
    }

	fn fill_polygon(&self, x: f64, y: f64, r: f64, sides: i32, rotation: f64) {
        self.polygon(x, y, r, sides, rotation);
        self.fill();
    }

	fn star(&self, x: f64, y: f64, r0: f64, r1: f64, points: i32, rotation: f64) {
        self.save();
        self.translate(x, y);
        self.rotate(rotation);
        for i in 0..points * 2 {
            let mut r = r1;
            if i % 2 == 1 {
                r = r0;
            }
            let angle = PI / points as f64 * i as f64;
            self.line_to(angle.cos() * r, angle.sin() * r);
        }
        self.close_path();
        self.restore();
    }

	fn stroke_star(&self, x: f64, y: f64, r0: f64, r1: f64, points: i32, rotation: f64) {
        self.star(x, y, r0, r1, points, rotation);
        self.stroke();
    }

	fn fill_star(&self, x: f64, y: f64, r0: f64, r1: f64, points: i32, rotation: f64) {
        self.star(x, y, r0, r1, points, rotation);
        self.fill();
    }

    fn splat(&self, x: f64, y: f64, num_nodes: i32, radius: f64, inner_radius: f64, variation: f64) {
        let mut points: Vec<Point> = Vec::new();
        let slice = TWO_PI / (num_nodes * 2) as f64;
        let mut angle = 0.0;
        let curve = 0.3;
        let radius_range = radius - inner_radius;
        let variation = clamp(variation, 0.0, 1.0);
        for _i in 0..num_nodes {
            let radius = radius + variation * (rand::random::<f64>() * radius_range * 2.0 - radius_range);
            let radius_range = radius - inner_radius;
            points.push(make_point(angle - slice * (1.0 + curve), inner_radius));
            points.push(make_point(angle + slice * curve, inner_radius));
            points.push(make_point(angle - slice * curve, inner_radius + radius_range * 0.8));
            points.push(make_point(angle + slice / 2.0, radius));
            points.push(make_point(angle + slice * (1.0 + curve), inner_radius + radius_range * 0.8));
            angle += slice * 2.0;
        }

        self.save();
        self.translate(x, y);
        self.multi_loop(&points);
        self.restore();
        
        fn make_point(angle: f64, radius: f64) -> Point {
           Point {
                x: angle.cos() * radius, 
                y: angle.sin() * radius
            }
        }
    }

    fn stroke_splat(&self, x: f64, y: f64, num_nodes: i32, radius: f64, inner_radius: f64, variation: f64) {
        self.splat(x, y, num_nodes, radius, inner_radius, variation);
        self.stroke();
    }

    fn fill_splat(&self, x: f64, y: f64, num_nodes: i32, radius: f64, inner_radius: f64, variation: f64) {
        self.splat(x, y, num_nodes, radius, inner_radius, variation);
        self.fill();
    }

    fn fractal_line(&self, x1: f64, y1: f64, x2: f64, y2: f64, roughness: f64, iterations: i32) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let mut offset = (dx * dx + dy * dy).sqrt() * 0.15;

        let mut path: Vec<Point> = Vec::new();
        path.push(Point::new(x1, y1));
        path.push(Point::new(x2, y2));
        for _i in 0..iterations {
            let mut new_path: Vec<Point> = Vec::new();
            for (j, point) in path.iter().enumerate() {
                new_path.push(Point::new(point.x, point.y));
                if j < path.len() - 1 {
                    let x = (point.x + path[j + 1].x) / 2.0 + rand::random::<f64>() * offset * 2.0 - offset;
                    let y = (point.y + path[j + 1].y) / 2.0 + rand::random::<f64>() * offset * 2.0 - offset;
                    new_path.push(Point::new(x, y));
                }
            }
            offset *= roughness;
            path = new_path;
        }
        self.path(&path);
    }

    fn stroke_fractal_line(&self, x1: f64, y1: f64, x2: f64, y2: f64, roughness: f64, iterations: i32) {
        self.fractal_line(x1, y1, x2, y2, roughness, iterations);
        self.stroke();
    }

    fn heart(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.save();
        self.translate(x, y);
        self.rotate(r);
        let mut path: Vec<Point> = Vec::new();
        let res = (w * h).sqrt() as i32;
        for i in 0..res {
            let a = TWO_PI * i as f64 / res as f64;
            let x = w * a.sin().powf(3.0);
            let y = h * (0.8125 * a.cos()
                       - 0.3125 * (2.0 * a).cos()
                       - 0.125 * (3.0 * a).cos()
                       - 0.0625 * (4.0 * a).cos());
            path.push(Point::new(x, -y));
        }
        self.path(&path);
        self.restore()
    }

    fn fill_heart(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.heart(x, y, w, h, r);
        self.fill();
    }

    fn stroke_heart(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.heart(x, y, w, h, r);
        self.stroke();
    }

    fn points(&self, points: &[Point], radius: f64) {
        for point in points.iter() {
            self.fill_circle(point.x, point.y, radius);
        }
    }

    fn stroke_curve_to(&self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.curve_to(x0, y0, x1, y1, x2, y2);
        self.stroke();
    }

    fn quadratic_curve_to(&self, x0: f64, y0: f64, x1: f64, y1: f64) {
        let p = self.get_current_point();
        self.curve_to(
            2.0 / 3.0 * x0 + 1.0 / 3.0 * p.0,
            2.0 / 3.0 * y0 + 1.0 / 3.0 * p.1,
            2.0 / 3.0 * x0 + 1.0 / 3.0 * x1,
            2.0 / 3.0 * y0 + 1.0 / 3.0 * y1,
            x1, y1);
    }

    fn stroke_quadratic_curve_to(&self, x0: f64, y0: f64, x1: f64, y1: f64) {
        self.quadratic_curve_to(x0, y0, x1, y1);
        self.stroke();
    }

    fn multi_curve(&self, points: &[Point]) {
        self.move_to(points[0].x, points[0].y);
        self.line_to((points[0].x + points[1].x) / 2.0,
                     (points[0].y + points[1].y) / 2.0);
        let mut i = 1;
        while i < points.len() - 1 {
            let p0 = &points[i];
            let p1 = &points[i + 1];
            let midx = (p0.x + p1.x) / 2.0;
            let midy = (p0.y + p1.y) / 2.0;
            self.quadratic_curve_to(p0.x, p0.y, midx, midy);
            i = i + 1;

        }
        let p = &points[points.len() - 1];
        self.line_to(p.x, p.y);
    }

    fn stroke_multi_curve(&self, points: &[Point]) {
        self.multi_curve(points);
        self.stroke();
    }

    fn multi_loop(&self, points: &[Point]) {
        let p_a = &points[0];
        let p_z = &points[points.len() - 1];
        let mid1x = (p_z.x + p_a.x) / 2.0;
        let mid1y = (p_z.y + p_a.y) / 2.0;
        self.move_to(mid1x, mid1y);
        for i in 0..points.len() - 1 {
            let p0 = &points[i];
            let p1 = &points[i + 1];
            let midx = (p0.x + p1.x) / 2.0;
            let midy = (p0.y + p1.y) / 2.0;
            self.quadratic_curve_to(p0.x, p0.y, midx, midy);
        }
        self.quadratic_curve_to(p_z.x, p_z.y, mid1x, mid1y);
    }

    fn fill_multi_loop(&self, points: &[Point]) {
        self.multi_loop(points);
        self.fill();
    }

    fn stroke_multi_loop(&self, points: &[Point]) {
        self.multi_loop(points);
        self.stroke();
    }

    fn grid(&self, x: f64, y: f64, w: f64, h: f64, xres: f64, yres: f64) {
        let mut xx = x;
        let mut yy = y;
        while xx <= x + w {
            self.move_to(xx, y);
            self.line_to(xx, y + h);
            xx += xres;
        }
        while yy <= y + h {
            self.move_to(x, yy);
            self.line_to(x + w, yy);
            yy += yres;
        }
        self.stroke();
    }
}
