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

/// Trapezoidal rule integration with variable time steps:
/// measures: values to integrate (length N),
/// t0: integration start time,
/// t1: integration end time,
/// dts: time steps between consecutive measures (length N - 1)
pub fn integrate(measures: &[f32], t0: f32, t1: f32, dts: &[f32]) -> f32 {
    if measures.len() < 2 || dts.is_empty() || measures.len() != dts.len() + 1 || t1 <= t0 {
        return 0.0;
    }

    let mut current_time = 0.0;
    let mut sum = 0.0;

    for i in 0..dts.len() {
        let dt = dts[i];
        if dt <= 0.0 {
            current_time += dt;
            continue;
        }

        let segment_start = current_time;
        let segment_end = current_time + dt;

        // Check if the segment overlaps with the [t0, t1] interval
        if segment_end > t0 && segment_start < t1 {
            // Clamp the interval to the current segment bounds
            let sub_t0 = segment_start.max(t0);
            let sub_t1 = segment_end.min(t1);

            // Linear interpolation of values at exact sub_t0 and sub_t1 bounds
            let frac0 = (sub_t0 - segment_start) / dt;
            let frac1 = (sub_t1 - segment_start) / dt;

            let y0 = measures[i] + frac0 * (measures[i + 1] - measures[i]);
            let y1 = measures[i] + frac1 * (measures[i + 1] - measures[i]);

            // Area of the sub-trapezoid
            sum += 0.5 * (sub_t1 - sub_t0) * (y0 + y1);
        }

        current_time = segment_end;
        if current_time >= t1 {
            break;
        }
    }

    sum
}

/// Numerical differentiation with variable time steps:
/// measures: values to differentiate (length N),
/// dts: time steps between consecutive measures (length N - 1),
/// t: target time at which the derivative is evaluated
pub fn differentiate(measures: &[f32], dts: &[f32], t: f32) -> f32 {
    let n = measures.len();
    if n < 2 || dts.is_empty() || n != dts.len() + 1 {
        return 0.0;
    }

    // Accumulate time steps to locate target time `t`
    let mut current_time = 0.0;
    let mut idx = 0;

    for (i, &dt) in dts.iter().enumerate() {
        if current_time + dt >= t {
            idx = i;
            break;
        }
        current_time += dt;
        idx = i;
    }

    // Boundary conditions and central difference scheme for non-uniform grid
    if idx == 0 {
        let dt0 = dts[0];
        if dt0 <= 0.0 {
            0.0
        } else {
            (measures[1] - measures[0]) / dt0
        }
    } else if idx >= dts.len() {
        let last_idx = dts.len() - 1;
        let dt_last = dts[last_idx];
        if dt_last <= 0.0 {
            0.0
        } else {
            (measures[n - 1] - measures[n - 2]) / dt_last
        }
    } else {
        // Non-uniform central difference around sample `idx`:
        // h0 = dt_{idx-1}, h1 = dt_{idx}
        let h0 = dts[idx - 1];
        let h1 = dts[idx];

        if h0 <= 0.0 || h1 <= 0.0 {
            return 0.0;
        }

        let y_prev = measures[idx - 1];
        let y_curr = measures[idx];
        let y_next = measures[idx + 1];

        // Formula for second-order accurate first derivative on non-uniform grid
        (y_next * h0 * h0 - y_prev * h1 * h1 + y_curr * (h1 * h1 - h0 * h0)) / (h0 * h1 * (h0 + h1))
    }
}

/// Calculates the PID response with:
/// cfg: PID configuration parameters,
/// setpoint: target value for the controller,
/// measures: vector of previous process measurements,
/// dts: vector of time steps between measurements
pub fn pid(cfg: &PIDCfg, setpoint: f32, measures: &[f32], dts: &[f32]) -> f32 {
    if measures.is_empty() || dts.len() + 1 != measures.len() {
        return 0.0;
    }

    let error: Vec<f32> = measures.iter().map(|&y| setpoint - y).collect();
    let t_end: f32 = dts.iter().sum();

    let p_term = cfg.kp * error.last().unwrap();

    // println!("coucou {}", p_term);

    let i_term = cfg.ki * integrate(&error, 0.0, t_end, dts);
    let d_term = cfg.kd * differentiate(&error, dts, t_end);

    p_term + i_term + d_term
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        // Constant time steps represented as arrays (1 element fewer than measures)
        let result1 = integrate(&[0., 0., 0., 0.], 0., 100., &[0.1, 0.1, 0.1]);
        let result2 = integrate(&[1., 1., 1., 1., 1., 1.], 0., 100., &[1., 1., 1., 1., 1.]);
        let result3 = integrate(&[1., 0., 1., 0., 1., 0.], 0., 100., &[1., 1., 1., 1., 1.]);
        let result4 = integrate(&[0., 1., 2., 0.], 0., 2., &[1., 1., 1.]);

        // Example with variable steps: dt = 0.5 between each point
        let result5 = integrate(&[0., 1., 2., 0.], 0., 2.1, &[0.5, 0.5, 0.5]);

        assert_eq!(result1, 0.);
        assert_eq!(result2, 5.);
        assert_eq!(result3, 2.5);
        assert_eq!(result4, 2.);
        assert_eq!(result5, 1.5);
    }

    #[test]
    fn int_variable_dt() {
        // Specific test with non-uniform time steps
        // x = [0, 1, 3], y = [1, 2, 4] -> dts = [1.0, 2.0]
        // Area 1 = 0.5 * 1.0 * (1 + 2) = 1.5
        // Area 2 = 0.5 * 2.0 * (2 + 4) = 6.0
        // Total = 7.5
        let measures = [1.0, 2.0, 4.0];
        let dts = [1.0, 2.0];
        let result = integrate(&measures, 0.0, 3.0, &dts);
        assert_eq!(result, 7.5);
    }

    #[test]
    fn diff_variable_dt() {
        // Linear function y = 2x over x = [0, 1, 3] -> dts = [1.0, 2.0]
        let measures = [0.0, 2.0, 6.0];
        let dts = [1.0, 2.0];

        let deriv_start = differentiate(&measures, &dts, 0.0);
        let deriv_end = differentiate(&measures, &dts, 3.0);

        assert_eq!(deriv_start, 2.0);
        assert_eq!(deriv_end, 2.0);
    }
}
