// This file simulate a simple pendulum with a certain masse and a certain couple.

const G: f32 = 9.81;

#[derive(Debug, Clone, Copy)]
pub struct PendulumState {
    pub dt: f32,       // time step, dt
    pub alpha: f32,    // pendulum angle, alpha
    d_alpha: f32,      // derivative pendulum angle
    d2_alpha: f32,     // double derivative pendulum angle
    pub masse: f32,    // pendulum masse, m
    pub length: f32,   // pendulum length, l
    torque: f32,       // torque applied on the pendulum, J
    pub friction: f32, // friction applied on the pendulum, lambda
}

impl PendulumState {
    fn new(
        dt: f32,
        alpha: f32,
        d_alpha: f32,
        d2_alpha: f32,
        masse: f32,
        length: f32,
        torque: f32,
        friction: f32,
    ) -> Self {
        Self {
            dt,
            alpha,
            d_alpha,
            d2_alpha,
            masse,
            length,
            torque,
            friction,
        }
    }
}

impl Default for PendulumState {
    fn default() -> Self {
        Self {
            dt: 0.1,
            alpha: 0.,
            d_alpha: 0.,
            d2_alpha: 0.,
            masse: 1.,
            length: 1.,
            torque: 0.,
            friction: 0.,
        }
    }
}

impl Iterator for PendulumState {
    type Item = PendulumState;

    /// This iterator gives us the next iteration of the pendulum simulation with the euler method.
    /// The pendulum verify the differential equation:
    ///
    /// d2/dt2(alpha) + (lambda/(m*l^2)) * d/dt(alpha) + (g/l)sin(alpha) - J/(m*l^2) = 0
    ///
    fn next(&mut self) -> Option<Self::Item> {
        // Compute angular acceleration from the differential equation:
        // d2_alpha = J/(m*l^2) - (lambda/(m*l^2))*d_alpha - (g/l)*sin(alpha)
        let inertia = self.masse * self.length * self.length;

        self.d2_alpha = self.torque / inertia
            - (self.friction / inertia) * self.d_alpha
            - (G / self.length) * self.alpha.sin();

        // Explicit Euler integration (semi-implicit / Euler-Cromer)
        self.d_alpha += self.d2_alpha * self.dt;
        self.alpha += self.d_alpha * self.dt;

        Some(*self)
    }
}
