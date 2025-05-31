use crate::point::Point;
use crate::tree_state::TreeState;


pub struct Tree {
    pub location: Point,
    pub state: TreeState,
}

pub struct Forest {
    pub trees: Vec<Tree>,
}

impl Forest {
    pub fn new() -> Self {
        Forest { trees: Vec::new() }
    }

    pub fn add_tree(&mut self, location: Point, state: Option<TreeState>) {
        let state = state.unwrap_or_else(|| TreeState::new('e'));
        self.trees.push(Tree { location, state });
    }


    pub fn get_trees(&self) -> &Vec<Tree> {
        &self.trees
    }

    pub fn plant_tree(&mut self, location: Point) {
        for tree in &mut self.trees {
            if tree.location == location {
                if tree.state.get() == 'e' {
                    tree.state = TreeState::new('t');
                }
                return;
            }
        }

        self.add_tree(location, Some(TreeState::new('t')));
    }

     pub fn burn_tree(&mut self, location: Point) {
        let mut stack = Vec::new();

        let hit_existing = if let Some(start) = self.trees.iter_mut().find(|tr| tr.location == location) {
            start.state = TreeState::new('h');
            true
        } else {
            self.trees.push(Tree { location, state: TreeState::new('h') });
            false
        };

        if hit_existing {
            stack.push(location);
            while let Some(pt) = stack.pop() {
                let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dx, dy) in deltas.iter().cloned() {
                    let neighbor = Point::new(pt.x + dx, pt.y + dy);
                    if let Some(tree) = self.trees.iter_mut().find(|tr| tr.location == neighbor) {
                        if tree.state.get() == 't' {
                            tree.state = TreeState::new('b');
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
    }

    fn _trees_percentage(&self) -> f32 {
        let total_trees = self.trees.len();
        let not_burned_trees = self.trees.iter().filter(|tree| tree.state.get() == 't').count();

        let trees_percentage = (not_burned_trees as f32 / total_trees as f32) * 100.0;

        return trees_percentage;

    }

    pub fn get_trees_percentage(&self) -> f32 {
        return self._trees_percentage();
    }

    pub fn print_trees_percentage(&self) -> f32 {
        let percentage = self._trees_percentage();
        println!("Burned percentage: {:.2}%", percentage);
        percentage
    }


    pub fn display(&self) {
        for tree in &self.trees {
                        println!("{} is {}", tree.location, tree.state);
        }
    }

}
