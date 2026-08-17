"""
Simulation d'un pendule simple soumis a un couple exterieur J et a un
frottement de l'air visqueux (proportionnel a la vitesse angulaire).

Equation differentielle :
    d2(alpha)/dt2 + (lambda/(m*l^2)) * d(alpha)/dt + (g/l) * sin(alpha) - J/(m*l^2) = 0

Integration par la methode d'Euler semi-implicite (Euler-Cromer),
et affichage : trajectoire angulaire + animation du pendule.
"""

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import animation

# ------------------------- Parametres physiques -------------------------
G = 9.81  # gravite (m/s^2)
MASS = 1.0  # masse (kg)
LENGTH = 1.0  # longueur du pendule (m)
TORQUE = 9.811  # couple applique (N.m) ; mettre != 0 pour un couple moteur
FRICTION = 0.5  # coefficient de frottement visqueux de l'air (N.m.s/rad)

# ------------------------- Parametres de simulation -------------------------
DT = 0.01  # pas de temps (s)
T_MAX = 30.0  # duree totale de la simulation (s)

ALPHA0 = np.pi / 2  # angle initial (rad)
D_ALPHA0 = 0.0  # vitesse angulaire initiale (rad/s)


def simulate(alpha0, d_alpha0, mass, length, torque, friction, dt, t_max):
    """Integre l'equation du pendule (avec frottement) par Euler semi-implicite."""
    n_steps = int(t_max / dt)
    t = np.zeros(n_steps)
    alpha = np.zeros(n_steps)
    d_alpha = np.zeros(n_steps)

    alpha[0] = alpha0
    d_alpha[0] = d_alpha0

    inertia = mass * length**2

    for i in range(1, n_steps):
        d2_alpha = (
            torque / inertia
            - (friction / inertia) * d_alpha[i - 1]
            - (G / length) * np.sin(alpha[i - 1])
        )
        d_alpha[i] = d_alpha[i - 1] + d2_alpha * dt
        alpha[i] = alpha[i - 1] + d_alpha[i] * dt
        t[i] = t[i - 1] + dt

    return t, alpha, d_alpha


def main():
    t, alpha, d_alpha = simulate(
        ALPHA0, D_ALPHA0, MASS, LENGTH, TORQUE, FRICTION, DT, T_MAX
    )

    # Coordonnees cartesiennes de la masse (origine = point d'attache)
    x = LENGTH * np.sin(alpha)
    y = -LENGTH * np.cos(alpha)

    fig, (ax_pendulum, ax_curve) = plt.subplots(1, 2, figsize=(11, 5))

    # --- Sous-graphe gauche : animation du pendule ---
    ax_pendulum.set_xlim(-1.2 * LENGTH, 1.2 * LENGTH)
    ax_pendulum.set_ylim(-1.2 * LENGTH, 1.2 * LENGTH)
    ax_pendulum.set_aspect("equal")
    ax_pendulum.set_title("Pendule (avec frottement de l'air)")
    ax_pendulum.grid(True)

    (line,) = ax_pendulum.plot([], [], "o-", lw=2, markersize=10, color="tab:blue")
    (trace,) = ax_pendulum.plot([], [], "-", lw=1, color="tab:orange", alpha=0.5)
    time_text = ax_pendulum.text(0.05, 0.9, "", transform=ax_pendulum.transAxes)

    # --- Sous-graphe droite : alpha(t) ---
    ax_curve.set_xlim(0, T_MAX)
    margin = 0.2 * (max(alpha) - min(alpha) + 1e-3)
    ax_curve.set_ylim(min(alpha) - margin, max(alpha) + margin)
    ax_curve.set_xlabel("temps (s)")
    ax_curve.set_ylabel("alpha (rad)")
    ax_curve.set_title("Angle en fonction du temps")
    ax_curve.grid(True)
    (curve_line,) = ax_curve.plot([], [], color="tab:blue")

    trace_x, trace_y = [], []

    def init():
        line.set_data([], [])
        trace.set_data([], [])
        curve_line.set_data([], [])
        time_text.set_text("")
        return line, trace, curve_line, time_text

    def update(frame):
        # on saute des frames pour accelerer l'animation
        i = frame * SKIP
        if i >= len(t):
            i = len(t) - 1

        line.set_data([0, x[i]], [0, y[i]])

        trace_x.append(x[i])
        trace_y.append(y[i])
        trace.set_data(trace_x[-200:], trace_y[-200:])  # garde une trainee limitee

        curve_line.set_data(t[:i], alpha[:i])
        time_text.set_text(f"t = {t[i]:.2f} s")
        return line, trace, curve_line, time_text

    SKIP = 3  # nombre de pas de simulation par frame d'animation
    n_frames = len(t) // SKIP

    ani = animation.FuncAnimation(
        fig,
        update,
        frames=n_frames,
        init_func=init,
        interval=DT * 1000 * SKIP,
        blit=True,
    )

    plt.tight_layout()
    plt.show()

    return (
        ani  # garder une reference pour eviter que l'animation soit garbage-collected
    )


if __name__ == "__main__":
    ani = main()
