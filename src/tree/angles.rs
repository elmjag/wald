use std::f32::consts::TAU;

use rand::Rng;
use rand::rngs::SmallRng;

const RANGE_WIDHT: f32 = 2.0;
const BELL_WIDTH: f32 = 0.6;

fn overflow_below_0(angle: f32) -> Option<f32> {
    let overflow = (RANGE_WIDHT / 2.0) - angle;
    if overflow <= 0.0 {
        return None;
    }

    Some(TAU - overflow)
}

fn overflow_above_tau(angle: f32) -> Option<f32> {
    let overflow = (angle + (RANGE_WIDHT / 2.0)) - TAU;
    if overflow <= 0.0 {
        return None;
    }

    Some(overflow)
}

fn wraped_values(angle: f32, x: f32) -> (f32, f32) {
    let diff = (x - angle).abs();

    if diff < (RANGE_WIDHT / 2.0) {
        /* no wrapping needed */
        return (angle, x);
    }

    if angle < x {
        /* wrap around angle full-circle */
        (angle + TAU, x)
    } else {
        assert!(x < angle);
        /* wrap around x full-circle */
        (angle, x + TAU)
    }
}

///
/// Angle Probability Density Function (PDF)
///
fn angle_pdf(angle: f32, x: f32) -> f32 {
    //
    // implements guassian distribution
    // with the top at 'angle' and hard-coded 'width'
    //

    // take care of cases where angle 'bell' wraps around 0 or TAU
    let (wraped_x, wraped_angle) = wraped_values(angle, x);

    ((-(wraped_x - wraped_angle).powi(2)) / BELL_WIDTH).exp()
}

pub fn find_in_range_angle(angles: &Vec<f32>, new_angle: f32) -> Option<f32> {
    let mut in_range_angle: Option<(f32, f32)> = None;

    let mut set_in_range_angle = |diff: f32, angle: f32| {
        in_range_angle = match in_range_angle {
            // no in range angle yet, save this one
            None => Some((diff, angle)),
            // update with new angle _if_
            // new one is closer then the old one
            Some((curr_diff, _)) => {
                if curr_diff < diff {
                    in_range_angle
                } else {
                    Some((diff, angle))
                }
            }
        }
    };

    for angle in angles.iter() {
        let diff = (new_angle - angle).abs();

        if diff <= (RANGE_WIDHT / 2.0) {
            set_in_range_angle(diff, *angle);
        }

        // deal with angles that wrap around 0.0
        match overflow_below_0(new_angle) {
            Some(overflow) => {
                if overflow < *angle {
                    set_in_range_angle(diff, *angle);
                    return Some(*angle);
                }
            }
            None => { /* nop */ }
        }

        // deal with angles that wrap around tau
        match overflow_above_tau(new_angle) {
            Some(overflow) => {
                if *angle < overflow {
                    set_in_range_angle(diff, *angle);
                    return Some(*angle);
                }
            }
            None => { /* nop */ }
        }
    }

    match in_range_angle {
        None => None,
        Some((_, angle)) => Some(angle),
    }
}

fn accept(branch_angles: &Vec<f32>, new_angle: f32, y: f32) -> bool {
    let in_range_angle = find_in_range_angle(branch_angles, new_angle);

    if in_range_angle.is_none() {
        // nothing in range, accept
        return true;
    }

    let probability = angle_pdf(in_range_angle.unwrap(), new_angle);

    // note that our PDF is 'inverted'
    y > probability
}

pub fn new_branch_angle(branch_angles: &Vec<f32>, rng: &mut SmallRng) -> f32 {
    loop {
        let angle = rng.random_range(0.0..TAU);
        let y = rng.random_range(0.0..1.0);

        if accept(branch_angles, angle, y) {
            return angle;
        }
    }
}
