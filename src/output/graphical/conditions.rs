use super::Interface;
use super::objects::Position;

use std::time::{SystemTime};

const SLIDE_DELTA : f64 = 0.005;
const SLIDE_VELOCITY : f64 = 0.05;

/* this one is for test purposes */

pub fn n_nano_seconds_from(&(n, now) : &(u32, &SystemTime), _ : & mut Interface) -> bool {
    match now.elapsed() {
        Ok(elapsed) => {
            match elapsed.as_secs() {
                s if s >= 1 => true,
                _           => match elapsed.subsec_nanos() {
                    s if s >= n => true,
                    _           => false
                }
            }
        }
        Err(_) => {
            true
        }
    }
}

pub fn object_slide_to(&(external_id, pos) : & (usize, &Position), interface : & mut Interface) -> bool {

    let object = & mut interface.objects[external_id];

    if object.get_position().distance_from(pos) < SLIDE_DELTA {
        object.set_position(pos.clone());
        return true
    }

    let x = -(object.get_position().x - pos.x) * (SLIDE_VELOCITY * interface.speed).min(1.0);
    let y = -(object.get_position().y - pos.y) * (SLIDE_VELOCITY * interface.speed).min(1.0);

    object.translate(x, y);

    false
}

//pub fn path_is_complete((path : &Vec<Move>))