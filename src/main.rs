const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

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
        .window("rust-sdl2 demo", WIDTH, HEIGHT)
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

    // initialise nodes
    let mut nodes = Vec::new();
    for _ in 0..2 {
        nodes.push(random_point(WIDTH, HEIGHT));
    }

    // initialise edges
    let mut edges = Vec::new();
    edges.push((0, 1));

    setup(&mut canvas, &nodes, &edges)?;

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
                        Some(sdl2::keyboard::Keycode::Return) => {
                            let node = random_point(WIDTH, HEIGHT);

                            draw_node(&mut canvas, &node)?;

                            let next_node_index: i32 = rand::Rng::gen_range(
                                &mut rand::thread_rng(),
                                0..(nodes.len() as i32)
                            );

                            nodes.push(node);

                            let edge = (nodes.len() as i32 - 1, next_node_index);

                            draw_edge(&mut canvas, &nodes, &edge)?;

                            edges.push(edge);

                            canvas.present();
                        }
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

fn setup(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    nodes: &Vec<(i32, i32)>,
    edges: &Vec<(i32, i32)>
) -> Result<(), String> {
    // fill the canvas
    canvas.set_draw_color(sdl2::pixels::Color::WHITE);
    canvas.clear();

    draw_edges(canvas, &nodes, &edges)?;
    draw_nodes(canvas, &nodes)?;

    // update the canvas
    canvas.present();

    Ok(())
}

fn random_point(max_x: u32, max_y: u32) -> (i32, i32) {
    let mut rng = rand::thread_rng();

    let x: u32 = rand::Rng::gen_range(&mut rng, 0..max_x);
    let y: u32 = rand::Rng::gen_range(&mut rng, 0..max_y);

    (x as i32, y as i32)
}

fn draw_edges(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    nodes: &Vec<(i32, i32)>,
    edges: &Vec<(i32, i32)>
) -> Result<(), String> {
    for edge in edges.into_iter() {
        draw_edge(canvas, nodes, edge)?;
    }

    Ok(())
}

fn draw_edge(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    nodes: &Vec<(i32, i32)>,
    edge: &(i32, i32)
) -> Result<(), String> {
    sdl2::gfx::primitives::DrawRenderer::line(
        canvas,
        nodes[edge.0 as usize].0 as i16,
        nodes[edge.0 as usize].1 as i16,
        nodes[edge.1 as usize].0 as i16,
        nodes[edge.1 as usize].1 as i16,
        sdl2::pixels::Color::BLACK
    )?;

    Ok(())
}

fn draw_nodes(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    nodes: &Vec<(i32, i32)>
) -> Result<(), String> {
    for node in nodes.into_iter() {
        draw_node(canvas, node)?;
    }

    Ok(())
}

fn draw_node(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    node: &(i32, i32)
) -> Result<(), String> {
    sdl2::gfx::primitives::DrawRenderer::circle(
        canvas,
        node.0 as i16,
        node.1 as i16,
        8,
        sdl2::pixels::Color::BLACK
    )?;

    Ok(())
}
