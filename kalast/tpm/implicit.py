import numpy

from kalast.util import STEFAN_BOLTZMANN


def tridiag_implicit(
    nx: int, k: numpy.array, dx: numpy.array, rcoef: numpy.array
) -> numpy.array:
    """
    Update the coefficients of the tridiagonal matrix.
    Necessary if some of the thermal properties, the grid or the timestep are changed between iterations.
    Based on multiheats.
    """
    matrix = numpy.zeros((3, nx))
    dkn = (k[2:] - k[1:-1]) / (2 * dx[1:]) + (k[1:-1] - k[:-2]) / (2 * dx[:-1])
    # an
    matrix[2, :-2] = (
        -rcoef[1:-1] / dx[:-1] * (-dkn / 2 + 2 * k[1:-1] / (dx[1:] + dx[:-1]))
    )
    # bn
    matrix[1, 1:-1] = 1 - rcoef[1:-1] / (dx[1:] * dx[:-1]) * (
        dkn / 2 * (dx[1:] - dx[:-1]) - 2 * k[1:-1]
    )
    # cn
    matrix[0, 2:] = -rcoef[1:-1] / dx[1:] * (dkn / 2 + 2 * k[1:-1] / (dx[1:] + dx[:-1]))
    return matrix


def flux_bc_implicit(self, rcoef, solar_flux, method: str = "leyrat"):
    """
    Set boundary conditions for implicit Euler Scheme
    Imposed flux or imposed temperature possible.
    Based on multiheats.
    """
    # bc_bottom = 0
    s0_corr, b0_corr = bc_up_implicit(method, rcoef)  # Correction terms

    s0 = self.temp[0] + rcoef[0] * (
        (self.cond[1] - 3 * self.cond[0])
        / self.dx[0]
        * (-solar_flux / self.cond[0] + s0_corr)
        + self.qheat[0]
    )

    sN = self.temp[-1] + rcoef[-1] * (
        (3 * self.cond[-1] - self.cond[-2]) * self.bc_bottom / self.dx[-1]
        + self.qheat[-1]
    )

    coef_top = rcoef[0] * self.cond[0] / self.dx[0] ** 2
    coef_bot = rcoef[-1] * self.cond[-1] / self.dx[-1] ** 2

    b0 = 1 + 2 * coef_top + b0_corr  # b0
    c0 = -2 * coef_top  # c0
    aN = -2 * coef_bot  # aN
    bN = 1 + 2 * coef_bot  # bN
    return s0, sN, b0, c0, aN, bN


def bc_up_implicit(
    self,
    method: str,
    rcoef: numpy.array,
    e: float,
    t: numpy.array,
    k: numpy.array,
    dx: numpy.array,
):
    """
    Add corrected terms for the upper boundary conditions.
    Following C. Leyrat's solver (Probably same as Schorghofer).
    If not uses our standard BC (Mergny et al 2023), may cause instabilities.
    Based on multiheats.
    """
    if method == "leyrat":
        s0_corr = (
            -3 * STEFAN_BOLTZMANN * e * t[0] ** 4 / k[0]
        )  # Correction from Leyrat BC
        b0_corr = (
            -4
            * rcoef[0]
            * (self.cond[1] - 3 * k[0])
            * STEFAN_BOLTZMANN
            * e
            * t[0] ** 3
            / (dx[0] * k[0])
        )  # Correction with Leyrat BC
    else:
        s0_corr = e * STEFAN_BOLTZMANN * t[0] ** 4 / k[0]
        b0_corr = 0
    return s0_corr, b0_corr
