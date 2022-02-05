use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::marching::{Camera, Geom, Image, Smooth, Sphere, Translation, Vec3};

mod marching;

fn render(canvas: &mut WindowCanvas, img: &Image) {
    let color = Color::RGB(127, 64, 255);
    canvas.set_draw_color(color);
    canvas.clear();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_target(PixelFormatEnum::RGB24, img.width, img.height)
        .expect("could not create texture");
    for i in 0..img.width {
        for j in 0..img.height {
            let pixel = img.data[i as usize][j as usize];
            let pixel_data = [pixel.r, pixel.g, pixel.b];
            texture.update(Rect::new(i as i32, j as i32, 1, 1), &pixel_data, 3)
                .expect("could not write to texture");
        }
    }
    canvas.copy(&mut texture, None, None)
        .expect("could not write texture to canvas");
    canvas.present();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust ray", 800, 800)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    // State
    let size = canvas.window().size();
    let mut event_pump = sdl_context.event_pump()?;
    let cam = Camera::new(size.0, size.1);
    let sphere1 = geom!(Translation::new(Vec3::new(-1.25, 4., -1.25), geom!(Sphere::new(1.))));
    let sphere2 = geom!(Translation::new(Vec3::new(1.25, 4., -1.25), geom!(Sphere::new(1.))));
    let sphere3 = geom!(Translation::new(Vec3::new(-1.25, 4., 1.25), geom!(Sphere::new(1.))));
    let sphere4 = geom!(Translation::new(Vec3::new(1.25, 4., 1.25), geom!(Sphere::new(1.))));
    let sphere5 = geom!(Translation::new(Vec3::new(0., 4., 0.), geom!(Sphere::new(1.))));

    let k = 1.;
    let geometry = geom!(Smooth::new(k, sphere1, sphere2));
    let geometry = geom!(Smooth::new(k, geometry, sphere3));
    let geometry = geom!(Smooth::new(k, geometry, sphere4));
    let geometry = geom!(Smooth::new(k, geometry, sphere5));

    let now = Instant::now();
    let img = cam.render(&geometry);
    let elapsed = now.elapsed();
    println!("Rendering took: {:.2?}", elapsed);

    'running: loop {
        // Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Update

        // Render
        render(&mut canvas, &img);

        // Sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
