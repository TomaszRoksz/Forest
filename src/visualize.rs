use piston_window::*;
use crate::forest::Forest;

pub fn visualize(forest: &Forest, grid_cols: u32, grid_rows: u32) {
    let mut window: PistonWindow = WindowSettings::new("Forest Visualizer", [0, 0])
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .expect("Nie udało się zbudować PistonWindow");

    while let Some(event) = window.next() {
        let Size { width: win_w, height: win_h } = window.size();

        let cell_w = win_w as f64 / grid_cols as f64;
        let cell_h = win_h as f64 / grid_rows as f64;
        
        let cell_size: f64;

        if cell_w != cell_h {
            cell_size = cell_w.min(cell_h);
        } 
        else {
            cell_size = cell_w;
        }

        window.draw_2d(&event, |c, g, _device| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            for tree in forest.get_trees() {
                let x = tree.location.x as f64 * cell_size;
                let y = tree.location.y as f64 * cell_size;
                let color = match tree.state.get() {
                    'e' => [0.8, 0.8, 0.8, 1.0],
                    't' => [0.0, 1.0, 0.0, 1.0],
                    'b' => [1.0, 0.0, 0.0, 1.0],
                    'h' => [0.0, 0.0, 1.0, 1.0],
                    _   => [1.0, 0.0, 1.0, 1.0],
                };
                rectangle(
                    color,
                    [x, y, cell_size, cell_size],
                    c.transform,
                    g,
                );
            }
        });
    }
}
