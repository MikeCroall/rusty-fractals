use std::cmp;

pub struct MandelbrotSettings {
    pub max_iterations: i32,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    modified: bool,
}

impl Default for MandelbrotSettings {
    fn default() -> Self {
        Self {
            max_iterations: 35,
            min_x: -2.0,
            max_x: 1.0,
            min_y: -1.0,
            max_y: 1.0,
            modified: false,
        }
    }
}

impl MandelbrotSettings {
    pub fn add_iterations(&mut self, delta: i32) {
        self.max_iterations = cmp::max(1, self.max_iterations + delta);
        self.modified = true;
    }

    pub fn needs_re_render(&self) -> bool {
        self.modified
    }

    pub fn notify_rendered(&mut self) {
        self.modified = false;
    }
}
