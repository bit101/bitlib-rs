use std::process::Command;
use canvas::Canvas;

pub fn make_gif(canvas: &Canvas,
                frame_count: i32,
                fps: u8,
                temp_dir: &str,
                output_file: &str,
                render_fn: fn(&Canvas, f64)) {
    let frames_spec = format!("{}/*.png", temp_dir);
    make_frames(canvas, frame_count, temp_dir, render_fn);
    convert_gif(fps, &frames_spec, output_file);
}

// requires imagemagick to be installed.
pub fn convert_gif(fps: u8, input: &str, output: &str) {
    let delay = 100.0 / fps as f64;
    Command::new("sh")
        .arg("-c")
        .arg(format!("convert -delay {} {} {}", delay, input, output))
        .output()
        .expect("Unable to make animated gif");
}

pub fn make_frames(canvas: &Canvas, num_frames: i32, frames_path: &str, render_fn: fn(&Canvas, f64)) {
    for i in 0..num_frames {
        render_fn(canvas, i as f64 / num_frames as f64);
        let filename = format!("{}/anim_{:03}.png", frames_path, i);
        canvas.write(filename.as_str());
    }
}

