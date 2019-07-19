#![feature(never_type)]

extern crate graphics as gfx;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod clock;
mod graphics;

pub fn main() {
    let tiles = clockmain::tiles::Tile::get_all_prerendered();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("miniclock2", 512, 128)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let disp = graphics::SDLGraphics::new(canvas, &tiles[..]);

    let clock = clock::SystemClock();

    let mut app = clockmain::App::new(clock, disp);
    let mut events = clockmain::Events::default();

    let event_subsystem = sdl_context.event().unwrap();
    event_subsystem.register_custom_event::<TimerEvent>().unwrap();

    let timer_subsystem = sdl_context.timer().unwrap();
    let _timer = timer_subsystem.add_timer(500, Box::from(|| {
        event_subsystem.push_custom_event(TimerEvent(true)).unwrap();
        500
    }));
 
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::User { .. } => {
                    // Our only user event type is our timer event, so we'll
                    // just assume that's what we've got here.
                    events.tick = true;
                }
                _ => {}
            }

            app.update(&events);
            events = clockmain::Events::default();
            app.redraw();
        }
    }
}

struct TimerEvent(bool);
