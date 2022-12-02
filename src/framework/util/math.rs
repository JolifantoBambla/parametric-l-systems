pub const PI_2: f32 = std::f32::consts::PI * 2.0;

#[inline]
pub fn is_close_to_zero(val: f32) -> bool {
    val.abs() < f32::EPSILON
}
