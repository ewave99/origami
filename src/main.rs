// main function
fn main() -> Result<(), String> {
    // initialise sdl and get the context.
    // The question mark means that if an error occurs, the main function
    // returns here with the error. Better than .unwrap() as less side effects
    // (no panic! is used)
    let sdl_context = sdl2::init()?;
    // get the video subsystem of the sdl context
    let video_subsystem = sdl_context.video()?;

    // using the video subsystem, initialise the window
    let window = video_subsystem
        .window("rust-sdl2 demo", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // create the canvas to draw on
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // a way of getting the events
    let mut event_pump = sdl_context.event_pump()?;

    setup(&mut canvas)?;

    // create a loop with the label 'running. we need to do it in this way
    // because then, when we call break, we break out of the game loop
    // rather than just the event poll loop.
    'running: loop {
        // loop through all events that have been triggered
        for event in event_pump.poll_iter() {
            // do different things based on what type of event it is
            match event {
                // a quit event with any attributes
                sdl2::event::Event::Quit { .. } => {
                    break 'running
                },
                // a keydown event, and a shorthand way of getting its
                // keycode to perform further matches on
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    match keycode {
                        // which has a keycode of Esc
                        Some(sdl2::keyboard::Keycode::Escape) => {
                            // and any other attributes
                            break 'running // break out of the game loop
                        },
                        _ => {}
                    }
                },
                // else do nothing
                _ => {}
            }
        }

        // sleep for 1/30 of a second (there are 1billion ns in 1 second).
        // this means there are 30 frames per second
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 30));
    }

    // nothing has gone wrong so return an Ok value
    Ok(())
}

fn setup(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    let size = canvas.output_size()?;
    let width = size.0 as i32;
    let height = size.1 as i32;

    // fill the canvas
    canvas.set_draw_color(sdl2::pixels::Color::WHITE);
    canvas.clear();

    let mut nodes = Vec::new();
    for _ in 0..2 {
        nodes.push(random_point(width, height));
    }

    let mut edges = Vec::new();
    edges.push((0, 1));

    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    for edge in edges.into_iter() {
        canvas.draw_line(nodes[edge.0], nodes[edge.1])?;
    }

    //canvas.draw_line((width / 2, 0), (width / 2, height))?;

    // update the canvas
    canvas.present();

    Ok(())
}

fn random_point(max_x: i32, max_y: i32) -> (i32, i32) {
    let mut rng = rand::thread_rng();

    let x: i32 = rand::Rng::gen_range(&mut rng, 0..max_x);
    let y: i32 = rand::Rng::gen_range(&mut rng, 0..max_y);

    (x, y)
}
