pub struct Controller {
    stepping_pin: u8,
    direction_pin: u8
}

trait Moving {
    pub fn move_motor(&self, distance: f32) -> void;
    fn convert_to_step(&self, distance: f32) -> u32;
}
