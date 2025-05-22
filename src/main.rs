use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::render::BlendMode;
use std::{
    error::Error,
    time::{Duration, Instant},
};

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    dragged: bool,
}

impl Ball {
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Ball {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            radius,
            dragged: false,
        }
    }
}

const SDL_WINDOW_TRANSPARENT: u32 = 0x40000000;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl = sdl3::init()?;
    let video = sdl.video()?;

    // Get display mode for fullscreen dimensions
    let display_mode = video.get_primary_display()?.get_mode()?;
    let (width, height) = (display_mode.w as f32, display_mode.h as f32);

    // Create transparent window
    let window = video
        .window("Bouncing Ball", width as u32, height as u32)
        .set_window_flags(SDL_WINDOW_TRANSPARENT)
        .borderless()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();

    canvas.set_blend_mode(BlendMode::Blend);

    let mut ball = Ball::new(width / 2.0, height / 2.0, 25.0);
    let mut last_update = Instant::now();
    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseMotion {
                    x, y, mousestate, ..
                } => {
                    if mousestate.left() {
                        let dx = x as f32 - ball.x;
                        let dy = y as f32 - ball.y;

                        // Check if clicking on the ball
                        if dx.powi(2) + dy.powi(2) <= ball.radius.powi(2) || ball.dragged {
                            ball.dragged = true;
                            ball.x = x as f32;
                            ball.y = y as f32;
                            ball.vx = 0.0;
                            ball.vy = 0.0;
                        }
                    }
                }

                Event::MouseButtonUp { .. } => {
                    ball.dragged = false;
                }

                _ => {}
            }
        }

        // Physics update
        let delta_time = last_update.elapsed().as_secs_f32();
        last_update = Instant::now();

        if !ball.dragged {
            // Apply gravity
            ball.vy += 500.0 * delta_time;

            // Update position
            ball.x += ball.vx * delta_time;
            ball.y += ball.vy * delta_time;

            // Window bounds collision
            if ball.x < ball.radius {
                ball.x = ball.radius;
                ball.vx *= -0.8;
            } else if ball.x > width - ball.radius {
                ball.x = width - ball.radius;
                ball.vx *= -0.8;
            }

            if ball.y < ball.radius {
                ball.y = ball.radius;
                ball.vy *= -0.8;
            } else if ball.y > height - ball.radius {
                ball.y = height - ball.radius;
                ball.vy *= -0.8;
            }

            // Air resistance
            ball.vx *= 1.0 - (0.5 * delta_time);
            ball.vy *= 1.0 - (0.5 * delta_time);
        }

        // Drawing
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();

        // Draw ball
        canvas.set_draw_color(Color::RGBA(255, 100, 100, 255));
        draw_circle(
            &mut canvas,
            ball.x as i32,
            ball.y as i32,
            ball.radius as i32,
        )?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn draw_circle(
    canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    x: i32,
    y: i32,
    radius: i32,
) -> Result<(), Box<dyn Error>> {
    let diameter = radius * 2;
    for w in 0..diameter {
        for h in 0..diameter {
            let dx = radius - w;
            let dy = radius - h;
            if (dx * dx + dy * dy) <= (radius * radius) {
                canvas.draw_point(Point::new(x + dx, y + dy))?;
            }
        }
    }
    Ok(())
}
