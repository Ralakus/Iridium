#[allow(unused_imports)]
use super::core as core;
use core::na as na;

#[allow(dead_code)]
pub mod integration {

    use super::na as na;

    pub fn fr_coefficient_64() -> f64 {
        1f64 / (2f64 - 2f64.powf(1f64 / 3f64))
    }

    pub fn fr_complement_64() -> f64 {
        1f64 - 2f64 * fr_coefficient_64()
    }

    pub fn verlet(pos: &mut na::Vector3<f32>, vel: &mut na::Vector3<f32>, accel: &na::Vector3<f32>, delta: f32) {
        let half_delta: f32 = delta * 0.5f32;
        *pos += *vel * half_delta;
        *vel += *accel * delta;
        *pos += *vel * half_delta;
    }

    pub fn forest_ruth(pos: &mut na::Vector3<f32>, vel: &mut na::Vector3<f32>, accel: &na::Vector3<f32>, delta: f32) {
        verlet(pos, vel, accel, delta * fr_coefficient_64() as f32);
        verlet(pos, vel, accel, delta * fr_complement_64()  as f32);
        verlet(pos, vel, accel, delta * fr_coefficient_64() as f32);
    }
}