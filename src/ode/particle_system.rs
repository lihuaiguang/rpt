use super::ParticleState;

/// A trait that represents a system formulating some physical laws
pub trait ParticleSystem {
    /// Compute time-derivative of a state
    fn time_derivative(&self, state: &ParticleState) -> ParticleState;

    /// Integrate the system with RK4 for a given time, with given time step
    fn rk4_integrate(&self, state: &mut ParticleState, mut time: f64, step: f64) {
        // Do one integration step, a helper function for RK4
        let mut integrate_step = |step: f64| {
            let k1 = self.time_derivative(state);
            let k2 = self.time_derivative(&(&*state + &k1 * (step / 2.0)));
            let k3 = self.time_derivative(&(&*state + &k2 * (step / 2.0)));
            let k4 = self.time_derivative(&(&*state + &k3 * step));
            *state = &*state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (step / 6.0);
        };
        while time > step {
            integrate_step(step);
            time -= step;
        }
        integrate_step(time);
    }
}

pub struct SimpleCircleSystem;

impl ParticleSystem for SimpleCircleSystem {
    fn time_derivative(&self, state: &ParticleState) -> ParticleState {
        ParticleState {
            pos: state
                .pos
                .iter()
                .map(|p| glm::vec3(-p.y, p.x, 0.0))
                .collect(),
            vel: vec![glm::vec3(0.0, 0.0, 0.0); state.vel.len()],
        }
    }
}

/// System that represents solid gravity objects in space
pub struct SolidGravitySystem;

impl ParticleSystem for SolidGravitySystem {
    fn time_derivative(&self, state: &ParticleState) -> ParticleState {
        let mut acc = vec![glm::vec3(0.0, 0.0, 0.0); state.pos.len()];
        for i in 0..state.pos.len() {
            for j in 0..i {
                let dir = glm::normalize(&(state.pos[i] - state.pos[j]));
                let len = glm::length(&(state.pos[i] - state.pos[j]));
                let force = dir * (len.powi(-2) - 0.0001 * len.powi(-5));
                acc[j] += force;
                acc[i] -= force;
            }
        }

        ParticleState {
            pos: state.vel.clone(),
            vel: acc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rk4_works() {
        let mut state = ParticleState {
            pos: vec![glm::vec3(1.0, 0.0, 0.0)],
            vel: vec![glm::vec3(0.0, 0.0, 0.0)],
        };
        SimpleCircleSystem.rk4_integrate(&mut state, std::f64::consts::TAU, 0.005);
        println!("{}", state.pos[0].x);
        assert!(glm::distance(&state.pos[0], &glm::vec3(1.0, 0.0, 0.0)) < 1e-3);
    }
}
