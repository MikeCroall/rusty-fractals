use std::cmp;

pub(crate) struct MandelbrotSettings {
    pub max_iterations: i32,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    zoom_factor: f64,
    pan_factor: f64,
    modified: bool,
}

impl Default for MandelbrotSettings {
    fn default() -> Self {
        Self {
            max_iterations: 35,
            min_x: -2.0,
            max_x: 0.5,
            min_y: -1.0,
            max_y: 1.0,
            zoom_factor: 1.01,
            pan_factor: 0.05,
            modified: false,
        }
    }
}

impl MandelbrotSettings {
    pub(crate) fn add_iterations(&mut self, delta: i32) {
        self.max_iterations = cmp::max(1, self.max_iterations + delta);
        self.modified = true;
    }

    pub(crate) fn needs_re_render(&self) -> bool {
        self.modified
    }

    pub(crate) fn notify_rendered(&mut self) {
        self.modified = false;
    }

    pub(crate) fn zoom_in(&mut self) {
        self.min_x /= self.zoom_factor;
        self.max_x /= self.zoom_factor;
        self.min_y /= self.zoom_factor;
        self.max_y /= self.zoom_factor;
        self.modified = true;
    }

    pub(crate) fn zoom_out(&mut self) {
        self.min_x *= self.zoom_factor;
        self.max_x *= self.zoom_factor;
        self.min_y *= self.zoom_factor;
        self.max_y *= self.zoom_factor;
        self.modified = true;
    }

    pub(crate) fn zoom_reset(&mut self) {
        self.zoom_factor = MandelbrotSettings::default().zoom_factor;
        self.modified = true;
    }

    pub(crate) fn pan_reset(&mut self) {
        self.min_x = MandelbrotSettings::default().min_x;
        self.max_x = MandelbrotSettings::default().max_x;
        self.min_y = MandelbrotSettings::default().min_y;
        self.max_y = MandelbrotSettings::default().max_y;
        self.modified = true;
    }

    pub(crate) fn pan_left(&mut self) {
        let delta = self.calculate_pan_delta(self.min_x, self.max_x);
        self.min_x -= delta;
        self.max_x -= delta;
        self.modified = true;
    }

    pub(crate) fn pan_right(&mut self) {
        let delta = self.calculate_pan_delta(self.min_x, self.max_x);
        self.min_x += delta;
        self.max_x += delta;
        self.modified = true;
    }

    pub(crate) fn pan_up(&mut self) {
        let delta = self.calculate_pan_delta(self.min_y, self.max_y);
        self.min_y -= delta;
        self.max_y -= delta;
        self.modified = true;
    }

    pub(crate) fn pan_down(&mut self) {
        let delta = self.calculate_pan_delta(self.min_y, self.max_y);
        self.min_y += delta;
        self.max_y += delta;
        self.modified = true;
    }

    fn calculate_pan_delta(&self, boundary_start: f64, boundary_end: f64) -> f64 {
        self.pan_factor * num::abs(boundary_start - boundary_end)
    }
}
