use crate::point::Point;
use crate::tree_state::TreeState;

pub struct Forest {
    grid: Vec<Option<TreeState>>,
    size: usize,
    tree_state: TreeState,
    burned_tree_state: TreeState,
    hitted_tree_state: TreeState,
}

impl Forest {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Forest {
            grid: vec![None; size],
            size,
            tree_state: TreeState::new('t'),
            burned_tree_state: TreeState::new('b'),
            hitted_tree_state: TreeState::new('h'),
        }
    }

    pub fn add_tree(&mut self, location: Point) {
        let idx = location.x as usize;
        if idx < self.size {
            self.grid[idx] = Some(self.tree_state);
        }
    }

    pub fn get_grid(&self) -> &Vec<Option<TreeState>> {
        &self.grid
    }

    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            *cell = None;
        }
    }

    pub fn burn_tree(&mut self, location: Point, width: i32) {
        let idx = location.x as usize;
        if self.grid[idx].is_none() {
            self.add_tree(location);
            self.grid[idx] = Some(self.hitted_tree_state);
            return;
        }

        self.grid[idx] = Some(self.hitted_tree_state);
        let mut stack = Vec::new();
        stack.push(location);

        while let Some(pt) = stack.pop() {
            let deltas = [-1, -width, 1, width];
            for &dx in deltas.iter() {
                let new_x = pt.x + dx;
                if new_x < 0 || new_x >= self.size as i32 {
                    continue;
                }
                let neighbor_idx = new_x as usize;
                if let Some(state) = self.grid[neighbor_idx] {
                    if state.get() == 't' {
                        self.grid[neighbor_idx] = Some(self.burned_tree_state);
                        stack.push(Point::new(new_x));
                    }
                }
            }
        }
    }

    fn _trees_percentage(&self, afforestation: f32) -> f32 {
        let total_trees = self.grid.iter().filter(|&&t| t.is_some()).count();
        let not_burned_trees = self
            .grid
            .iter()
            .filter(|&&t| t.map_or(false, |s| s.get() == 't'))
            .count();
        if total_trees == 0 {
            0.0
        } else {
            not_burned_trees as f32 / total_trees as f32 * afforestation
        }
    }

    pub fn get_trees_percentage(&self, afforestation: f32) -> f32 {
        self._trees_percentage(afforestation)
    }
}

