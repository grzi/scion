/// Convenience struct used in all `Scion` to specify any 2D position.
#[derive(Default, Debug, Copy, Clone)]
pub struct Coordinates {
    x: f32,
    y: f32,
    layer: usize,
}

impl Coordinates {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, layer: 0 }
    }

    pub fn new_with_layer(x: f32, y: f32, layer: usize) -> Self {
        Self { x, y, layer }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn layer(&self) -> usize {
        self.layer
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.layer = layer;
    }
}

/// Component used by the renderer to know where and how to represent an object.
/// Default is position 0;0 with a scale of 1.0 and no angle.
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub(crate) local_translation: Coordinates,
    pub(crate) global_translation: Coordinates,
    pub(crate) scale: f32,
    pub(crate) angle: f32,
    pub(crate) dirty: bool,
    pub(crate) dirty_child: bool,
    pub(crate) min_x: Option<f32>,
    pub(crate) max_x: Option<f32>,
    pub(crate) min_y: Option<f32>,
    pub(crate) max_y: Option<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            local_translation: Default::default(),
            global_translation: Default::default(),
            scale: 1.0,
            angle: 0.0,
            dirty: false,
            dirty_child: true,
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }
}

impl Transform {
    /// Creates a new transform using provided values.
    pub fn new(translation: Coordinates, scale: f32, angle: f32) -> Self {
        Self {
            local_translation: translation,
            global_translation: translation,
            scale,
            angle,
            dirty: false,
            dirty_child: true,
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }

    /// Append a translation to this transform's position
    pub fn append_translation(&mut self, x: f32, y: f32) {
        self.local_translation.x += x;
        self.local_translation.y += y;
        self.global_translation.x += x;
        self.global_translation.y += y;
        self.dirty = true;
        self.handle_bounds();
    }

    /// Appends the x val to the translation's x value
    pub fn append_x(&mut self, x: f32) {
        self.local_translation.x += x;
        self.global_translation.x += x;
        self.dirty = true;
        self.handle_bounds();
    }

    /// Appends the y val to the translation's y value
    pub fn append_y(&mut self, y: f32) {
        self.local_translation.y += y;
        self.global_translation.y += y;
        self.dirty = true;
        self.handle_bounds();
    }

    /// Move this transform down
    pub fn move_down(&mut self, y: f32) {
        self.local_translation.y += y;
        self.global_translation.y += y;
        self.dirty = true;
        self.handle_bounds();
    }

    /// Append an angle to this transform's angle
    pub fn append_angle(&mut self, angle: f32) {
        self.angle += angle;
    }

    /// Get the transform's coordinates
    pub fn translation(&self) -> &Coordinates {
        &self.local_translation
    }

    /// Get the global transform's translation
    pub fn global_translation(&self) -> &Coordinates {
        &self.global_translation
    }

    /// Change the scale value to a new one.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }

    /// Change the layer value in the coordinates.
    pub fn set_layer(&mut self, layer: usize) {
        self.local_translation.layer = layer
    }

    /// Configure the minimum global x position for this transform to be min_x
    pub fn set_min_x(&mut self, min_x: Option<f32>) {
        self.min_x = min_x;
        self.handle_bounds();
    }

    /// Configure the maximum global x position for this transform to be max_x
    pub fn set_max_x(&mut self, max_x: Option<f32>) {
        self.max_x = max_x;
        self.handle_bounds();
    }

    /// Configure the minimum global y position for this transform to be min_x
    pub fn set_min_y(&mut self, min_y: Option<f32>) {
        self.min_y = min_y;
        self.handle_bounds();
    }

    /// Configure the maximum global y position for this transform to be max_x
    pub fn set_max_y(&mut self, max_y: Option<f32>) {
        self.max_y = max_y;
        self.handle_bounds();
    }

    /// Configure the minimum and maximum global values of x and y
    pub fn set_global_translation_bounds(
        &mut self,
        min_x: Option<f32>,
        max_x: Option<f32>,
        min_y: Option<f32>,
        max_y: Option<f32>,
    ) {
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;
        self.handle_bounds();
    }

    /// Computes the global_translation using the parent as origin
    pub(crate) fn compute_global_from_parent(&mut self, parent_translation: &Coordinates) {
        let mut new_global = parent_translation.clone();
        new_global.x += self.local_translation.x;
        new_global.y += self.local_translation.y;
        new_global.layer = self.local_translation.layer;
        self.global_translation = new_global;
        self.dirty = true;
        self.handle_bounds();
    }

    fn handle_bounds(&mut self) {
        if let Some(min_x) = self.min_x {
            if self.global_translation.x < min_x {
                self.global_translation.set_x(min_x);
            }
        }

        if let Some(max_x) = self.max_x {
            if self.global_translation.x > max_x {
                self.global_translation.set_x(max_x);
            }
        }

        if let Some(min_y) = self.min_y {
            if self.global_translation.y < min_y {
                self.global_translation.set_y(min_y);
            }
        }

        if let Some(max_y) = self.max_y {
            if self.global_translation.y > max_y {
                self.global_translation.set_y(max_y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::components::maths::transform::{Coordinates, Transform};

    #[test]
    fn compute_global_from_parent_test() {
        let parent_translation = Coordinates::new(1., 2.);
        let mut child_transform = Transform::new(Coordinates::new(5., 3.), 1., 1.);

        assert_eq!(5., child_transform.global_translation.x);
        assert_eq!(3., child_transform.global_translation.y);

        child_transform.compute_global_from_parent(&parent_translation);

        assert_eq!(6., child_transform.global_translation.x);
        assert_eq!(5., child_transform.global_translation.y);
    }

    #[test]
    fn modify_transform_should_set_dirty_test() {
        let mut transform = Transform::new(Coordinates::new(5., 3.), 1., 1.);
        assert_eq!(false, transform.dirty);

        transform.append_translation(1., 1.);
        assert_eq!(true, transform.dirty);

        transform.dirty = false;

        transform.move_down(1.);
        assert_eq!(true, transform.dirty);

        transform.dirty = false;

        transform.append_x(1.);
        assert_eq!(true, transform.dirty);

        transform.dirty = false;

        transform.append_y(1.);
        assert_eq!(true, transform.dirty);

        transform.dirty = false;

        transform.compute_global_from_parent(&Coordinates::new(1., 2.));
        assert_eq!(true, transform.dirty);
    }

    #[test]
    fn handle_bounds_test() {
        let mut t = Transform::default();
        t.set_min_x(Some(1.));
        assert_eq!(1., t.global_translation.x);
        t.append_x(-6.);
        assert_eq!(1., t.global_translation.x);
    }
}
