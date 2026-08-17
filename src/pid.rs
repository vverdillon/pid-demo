#[derive(Debug)]
pub struct PIDCfg {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

impl Default for PIDCfg {
    fn default() -> Self {
        Self {
            kp: 0.,
            ki: 0.,
            kd: 0.,
        }
    }
}

/// Trapezoidal rule integration with:
/// measures: measures to integrate,
/// dt: time step between two measures,
/// t0: integration begining
/// t1: integration ending
pub fn integrate(measures: &[f32], t0: f32, t1: f32, dt: f32) -> f32 {
    if measures.len() < 2 || dt <= 0.0 {
        return 0.0;
    }

    // Convert time to index: i = t / dt
    let mut i0 = (t0 / dt).round() as usize;
    let mut i1 = (t1 / dt).round() as usize;

    // Bounds safety checks
    let max_idx = measures.len() - 1;
    i0 = i0.min(max_idx);
    i1 = i1.min(max_idx);

    if i0 >= i1 {
        return 0.0;
    }

    (i0..i1).fold(0.0, |sum, i| {
        sum + 0.5 * dt * (measures[i] + measures[i + 1])
    })
}

/// Derivative evaluation
/// measures: measures to differentiate,
/// dt: time step between two measures,
/// t: time at which the derivative is evaluated
pub fn differentiate(measures: &[f32], dt: f32, t: f32) -> f32 {
    let n = measures.len();
    if n < 2 || dt <= 0.0 {
        return 0.0;
    }

    let mut i = (t / dt).round() as usize;
    if i >= n {
        i = n - 1;
    }

    if i == 0 {
        (measures[1] - measures[0]) / dt
    } else if i == n - 1 {
        (measures[i] - measures[i - 1]) / dt
    } else {
        (measures[i + 1] - measures[i - 1]) / (2.0 * dt)
    }
}

/// Calculate the PID reponse with:
/// cfg: PID configuration,
/// setpoint: goal of the PID,
/// measures: previous measures
/// dt: time step
fn pid(cfg: PIDCfg, setpoint: f32, measures: Vec<f32>, dt: f32) -> f32 {
    let error: Vec<f32> = measures.into_iter().map(|y| setpoint - y).collect();
    let t_end = (error.len() as f32) * dt;

    cfg.kp * error[error.len() - 1]
        + cfg.ki * integrate(&error, 0., t_end, dt)
        + cfg.kd * differentiate(&error, dt, t_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let result1 = integrate(&[0., 0., 0., 0.], 0., 100., 0.1);
        let result2 = integrate(&[1., 1., 1., 1., 1., 1.], 0., 100., 1.);
        let result3 = integrate(&[1., 0., 1., 0., 1., 0.], 0., 100., 1.);
        let result4 = integrate(&[0., 1., 2., 0.], 0., 2., 1.);
        let result5 = integrate(&[0., 1., 2., 0.], 0., 2.1, 0.5);

        assert_eq!(result1, 0.);
        assert_eq!(result2, 5.);
        assert_eq!(result3, 2.5);
        assert_eq!(result4, 2.);
        assert_eq!(result5, 1.5);
    }
}
