import math
from collections.abc import Iterable

import symfit
# import glm
import matplotlib
import numpy
from numba import jit, njit  # noqa

# import kalast  # noqa

TIMOUT1 = "YYYY-MM-DD HR:MN ::RND"
TIMOUT2 = "YYYYMMDD ::RND"
TIMOUT3 = "YYYYMMDDTHRMNSC ::RND"

# incident spectral solar flux at 1 AU at 545 nm
SFLUX_545 = 1896.0


def numpy_float(func):
    def wrapper_numpy_float(*args):
        istuple = True
        for ii in range(0, len(args)):
            if isinstance(args[ii], Iterable):
                if args[ii].dtype != numpy.float64:
                    if istuple:
                        args = list(args)
                        istuple = False
                    args[ii] = args[ii].astype(float)
        return func(*args)

    return wrapper_numpy_float


# def mataxisang(angle: float, axis: glm.dvec3) -> glm.dmat4:
#     return glm.rotate(angle, axis)


# def diag3(m: glm.dmat4) -> numpy.array:
#     return numpy.diag(m)[:-1]
# 
# 
# def trace3(m: glm.dmat4) -> float:
#     return diag3(m).sum()
# 
# 
# def find_rotang(m: glm.dmat4) -> float:
#     return numpy.acos((trace3(m) - 1) / 2)
# 
# 
# def find_rotaxis(m: glm.dmat4) -> glm.dvec3:
#     a = glm.dvec3(m[1, 2], m[2, 0], m[0, 1])
#     b = glm.dvec3(m[2, 1], m[0, 2], m[1, 0])
#     return (a - b) / glm.distance(a, b)
# 
# 
# def matpow(m: glm.dmat4, n: int) -> glm.dmat4:
#     return glm.dmat4(numpy.linalg.matrix_power(m, n))


def newton_method(
    v: float,
    fn: callable,
    args_fn: dict,
    dfn: callable,
    args_dfn: dict,
    itmax: int = 1e5,
    threshold: int = 0.1,
) -> float:
    ii = 0

    while True:
        f = fn(v, **args_fn)
        df = dfn(v, **args_dfn)
        delta = -f / df
        v += delta

        if ii > itmax:
            raise ValueError(
                f"Newton method never converged after reaching {itmax} (threshold={threshold}, current value={v} and delta={delta})."
            )
        if abs(delta) < threshold:
            break
        ii += 1

    return v


def cmapv_to_rbg(value: float, cmap: matplotlib.colors.Colormap):
    """
    value between 0 and 1
    """
    index = int(value * 255)
    return cmap.colors[index]


def cart2sph(v: numpy.array) -> numpy.array:
    # az: counterclockwise angle in the x-y plane measured in radians from the positive x-axis [-pi, pi]
    # el: from x-y plane [-pi/2, pi/2]
    hxy = numpy.hypot(v[0], v[1])
    r = numpy.hypot(hxy, v[2])
    el = numpy.arctan2(v[2], hxy)
    az = numpy.arctan2(v[1], v[0])
    return numpy.array([az, el, r])


def sph2cart(v: numpy.array) -> numpy.array:
    # az el r
    rcos_theta = v[2] * numpy.cos(v[1])
    x = rcos_theta * numpy.cos(v[0])
    y = rcos_theta * numpy.sin(v[0])
    z = v[2] * numpy.sin(v[1])
    return numpy.array([x, y, z])


# def glm_cart2sph(v: glm.dvec3) -> glm.dvec3:
#     hxy = glm.length(v.xy)
#     r = glm.length(glm.dvec3(hxy, v.z))
#     el = glm.atan2(v.z, hxy)
#     az = glm.atan2(v.y, v.x)
#     return glm.dvec3(az, el, r)
# 
# 
# def glm_sph2cart(v: glm.dvec3) -> glm.dvec3:
#     # az el r
#     rcos_theta = v.z * glm.cos(v.y)
#     x = rcos_theta * glm.cos(v.x)
#     y = rcos_theta * glm.sin(v.x)
#     z = v.z * numpy.sin(v[1])
#     return glm.dvec3(x, y, z)


def flattening(r: numpy.array) -> float:
    return (r[0] - r[2]) / r[0]


def find_closest(
    m: numpy.ndarray,
    refv1: float,
    i1: int,
    threshold: int,
    refv2: float = None,
    i2: int = None,
    N: int = 1,
) -> list[tuple[int, numpy.ndarray]]:
    ii = numpy.where(numpy.abs(refv1 - m[:, i1]) < threshold)[0]

    if refv2 is None or i2 is None:
        return numpy.argsort(ii)[:N]

    jj = numpy.argsort(numpy.abs(refv2 - m[ii, i2]))[:N]
    return ii[jj]


def distance_haversine(D: float, lo1: float, la1: float, lo2: float, la2: float):
    """
    D: diameter of sphere
    """
    hav = (
        0.5
        - math.cos(la2 - la1) / 2
        + math.cos(la1) * math.cos(la2) * (1 - math.cos(lo2 - lo1)) / 2
    )
    return D * math.asin(math.sqrt(hav))


def fourier_series(x, f, n=0, ss: str | None = None):
    """
    Returns a symbolic fourier series of order `n`.

    n: Order of the fourier series.
    x: Independent variable
    f: Frequency of the fourier series
    """
    if ss is not None:
        ss = f"_{ss}"
    else:
        ss = ""

    # Make the parameter objects for all the terms
    a0, *cos_a = symfit.parameters(
        ",".join(["a{}{}".format(i, ss) for i in range(0, n + 1)])
    )

    if n > 0:
        sin_b = symfit.parameters(
            ",".join(["b{}{}".format(i, ss) for i in range(1, n + 1)])
        )
    else:
        sin_b = []

    # Construct the series
    series = a0 + sum(
        ai * symfit.cos(i * f * x) + bi * symfit.sin(i * f * x)
        for i, (ai, bi) in enumerate(zip(cos_a, sin_b), start=1)
    )
    return series