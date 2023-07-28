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
            max_iterations: 50,
            min_x: -2.2,
            max_x: 0.75,
            min_y: -1.2,
            max_y: 1.2,
            zoom_factor: 1.01,
            pan_factor: 0.05,
            modified: false,
        }
    }
}

impl MandelbrotSettings {
    pub(crate) fn notify_rendered(&mut self) {
        self.modified = false;
    }

    pub(crate) fn notify_needs_re_render(&mut self) {
        self.modified = true;
    }

    pub(crate) fn needs_re_render(&self) -> bool {
        self.modified
    }

    pub(crate) fn add_iterations(&mut self, delta: i32) {
        self.max_iterations = cmp::max(1, self.max_iterations + delta);
        self.notify_needs_re_render();
    }

    pub(crate) fn iterations_reset(&mut self) {
        self.max_iterations = MandelbrotSettings::default().max_iterations;
        self.notify_needs_re_render();
    }

    pub(crate) fn zoom_in(&mut self) {
        self.zoom(&MandelbrotSettings::zoom_in_to_origin);
    }

    pub(crate) fn zoom_out(&mut self) {
        self.zoom(&MandelbrotSettings::zoom_out_from_origin);
    }

    pub(crate) fn pan_left(&mut self) {
        self.offset_by(-self.calculate_pan_delta(self.min_x, self.max_x), 0f64);
        self.notify_needs_re_render();
    }

    pub(crate) fn pan_right(&mut self) {
        self.offset_by(self.calculate_pan_delta(self.min_x, self.max_x), 0f64);
        self.notify_needs_re_render();
    }

    pub(crate) fn pan_up(&mut self) {
        self.offset_by(0f64, -self.calculate_pan_delta(self.min_y, self.max_y));
        self.notify_needs_re_render();
    }

    pub(crate) fn pan_down(&mut self) {
        self.offset_by(0f64, self.calculate_pan_delta(self.min_y, self.max_y));
        self.notify_needs_re_render();
    }

    pub(crate) fn pan_and_zoom_reset(&mut self) {
        let defaults = MandelbrotSettings::default();
        self.min_x = defaults.min_x;
        self.max_x = defaults.max_x;
        self.min_y = defaults.min_y;
        self.max_y = defaults.max_y;
        self.notify_needs_re_render();
    }

    fn offset_by(&mut self, x_offset: f64, y_offset: f64) {
        self.min_x += x_offset;
        self.max_x += x_offset;
        self.min_y += y_offset;
        self.max_y += y_offset;
    }

    fn calculate_pan_delta(&self, boundary_start: f64, boundary_end: f64) -> f64 {
        self.pan_factor * num::abs(boundary_start - boundary_end)
    }

    fn find_current_center(&self) -> (f64, f64) {
        (
            (self.min_x + self.max_x) / 2f64,
            (self.min_y + self.max_y) / 2f64,
        )
    }

    fn zoom(&mut self, zoom_function: &dyn Fn(&mut MandelbrotSettings)) {
        let (x_offset, y_offset) = self.find_current_center();
        self.offset_by(-x_offset, -y_offset);
        zoom_function(self);
        self.offset_by(x_offset, y_offset);
        self.notify_needs_re_render();
    }

    fn zoom_in_to_origin(&mut self) {
        self.min_x /= self.zoom_factor;
        self.max_x /= self.zoom_factor;
        self.min_y /= self.zoom_factor;
        self.max_y /= self.zoom_factor;
    }

    fn zoom_out_from_origin(&mut self) {
        self.min_x *= self.zoom_factor;
        self.max_x *= self.zoom_factor;
        self.min_y *= self.zoom_factor;
        self.max_y *= self.zoom_factor;
    }
}
