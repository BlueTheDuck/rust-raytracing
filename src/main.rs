extern crate raytracer as rt;

use std::{time::SystemTime, path::{Path, PathBuf}};

use clap::Parser;
use glium::{
    glutin::{self, dpi::LogicalSize},
    uniform, Display, Surface,
};
use image::{ImageBuffer, Rgb, RgbImage};
use rt::{
    scene::{render, Camera},
    shapes::*,
};
use serde::Serialize;

mod window;

fn render_texture(width: u32, height: u32, display: &Display, objects: &[Shape]) -> glium::texture::SrgbTexture2d {
    println!("Rendering {width}x{height}");
    /* let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64(); */

    let origin = Vector::new(0.0, 0.0, -1.0);
    let upguide = Vector::new(0.0, -1.0, -1.0);
    let camera = Camera::new(60.0, Vector::zeros(), origin);

    let mut framebuffer = ImageBuffer::new(width, height);
    // This panics for some reason:
    // framebuffer.save("out.png").unwrap();

    render(&mut framebuffer, &objects, &camera);

    let raw_image = {
        let dims = framebuffer.dimensions();
        glium::texture::RawImage2d::from_raw_rgb_reversed(&framebuffer.into_raw(), dims)
    };

    let texture = glium::texture::SrgbTexture2d::new(display, raw_image).unwrap();

    return texture;
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = 640)]
    width: u32,

    #[clap(short, long, default_value_t = 360)]
    height: u32,

    #[clap(long, default_value = "scene.json", value_parser)]
    scene: PathBuf
}

fn main() {
    let mut args = Args::parse();

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(args.scene)
        .unwrap();
    let objects: Vec<Shape> = serde_json::from_reader(file).unwrap();
        

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Raytracer")
        .with_inner_size(LogicalSize::new(args.width, args.height));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &window::RECT).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        &display,
        include_str!("vertex.glsl"),
        include_str!("fragment.glsl"),
        None,
    )
    .unwrap();

    let mut texture = render_texture(args.width, args.height, &display, &objects);
    event_loop.run(move |ev, _, control_flow| {
        /* let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        control_flow.set_wait_until(next_frame_time); */
        control_flow.set_poll();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                    return;
                }
                glutin::event::WindowEvent::Resized(size) => {
                    // println!("Resized to {w}x{h}", w = size.width, h = size.height);
                    args.width = size.width;
                    args.height = size.height;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached {
                    start,
                    requested_resume,
                } => {
                    println!("Rendering new frame");
                    texture = render_texture(args.width, args.height, &display, &objects);
                }
                glutin::event::StartCause::WaitCancelled {
                    start,
                    requested_resume,
                } => (),
                glutin::event::StartCause::Poll => {
                    texture = render_texture(args.width, args.height, &display, &objects);
                }
                glutin::event::StartCause::Init => (),
            },
            glutin::event::Event::RedrawRequested(_) => {
                println!("Redraw requested");
                texture = render_texture(args.width, args.height, &display, &objects);
            }
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! {
                    tex: &texture
                },
                &Default::default(),
            )
            .unwrap();
        target.finish().expect("Failed to draw");
    });
    /* for (px, py, pixel) in img.enumerate_pixels_mut() {
        let i = (py * (args.width as u32) + px) as usize;
        let color = framebuffer[i].gamma(1.0, 2.2).as_rgb8();
        *pixel = *Rgb::from_slice(&color);
    }
    img.save("out.png").unwrap(); */
}
