use piston_window::*;
use crate::forest::Forest;


pub fn visualize(forest: &Forest, width: usize, height: usize) {
    let cell_size = 8;
    let window_width = (width * cell_size) as u32;
    let window_height = (height * cell_size) as u32;

    let mut window: PistonWindow = WindowSettings::new(
        "Forest Visualizer",
        [window_width, window_height],
    )
    .exit_on_esc(true)
    .build()
    .expect("Failed to build PistonWindow");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _device| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            for tree in forest.get_trees() {
                let x = tree.location.x as f64 * cell_size as f64;
                let y = tree.location.y as f64 * cell_size as f64;
                let color = match tree.state.get() {
                    'e' => [0.8, 0.8, 0.8, 1.0],
                    't' => [0.0, 1.0, 0.0, 1.0],
                    'b' => [1.0, 0.0, 0.0, 1.0],
                    'h' => [0.0, 0.0, 1.0, 1.0],
                    _   => [1.0, 0.0, 1.0, 1.0],
                };
                rectangle(
                    color,
                    [x, y, cell_size as f64, cell_size as f64],
                    c.transform,
                    g,
                );
            }
        });
    }
}
