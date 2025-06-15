use piston_window::*;
use crate::forest::Forest;

pub fn visualize(forest: &Forest, grid_cols: u32, grid_rows: u32) {
    let window_size = [800, 600];
    let mut window: PistonWindow = WindowSettings::new("Forest Visualizer", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    
    while let Some(event) = window.next() {
        let Size { width: win_w, height: win_h } = window.size();
        let cell_w = win_w as f64 / grid_cols as f64;
        let cell_h = win_h as f64 / grid_rows as f64;
        let cell_size = cell_w.min(cell_h);
        let total_w = cell_size * grid_cols as f64;
        let total_h = cell_size * grid_rows as f64;
        let offset_x = (win_w as f64 - total_w) / 2.0;
        let offset_y = (win_h as f64 - total_h) / 2.0;

        window.draw_2d(&event, |c, g, _device| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            for (idx, &state_option) in forest.get_grid().iter().enumerate() {
                if let Some(state) = state_option {
                    let col = (idx % grid_cols as usize) as f64;
                    let row = (idx / grid_cols as usize) as f64;
                    let px = offset_x + col * cell_size;
                    let py = offset_y + row * cell_size;
                    let color = match state.get() {
                        't' => [0.0, 1.0, 0.0, 1.0], // Green
                        'b' => [1.0, 0.0, 0.0, 1.0], // Red
                        'h' => [0.0, 0.0, 1.0, 1.0], // Blue
                        _ => [1.0, 0.0, 1.0, 1.0], // Magenta
                    };
                    rectangle(color, [px, py, cell_size, cell_size], c.transform, g);
                }
            }
        });
    }
}