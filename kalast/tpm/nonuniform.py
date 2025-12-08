import numpy

from kalast.util import STEFAN_BOLTZMANN


def R350(chi: float = 2.7) -> float:
    """
    Based on Mitchell and de Pater (1994) and heat1d.

    Args:
        Radiative conductivity parameter
    """
    return chi / 350**3


R350_DEFAULT = R350()


def column(d: float, m: int = 10, n: float = 5, b: int = 20) -> numpy.array:
    """Non-uniform depth column, layer thickness increasing.
    From heat1d.

    Args:
        depth of the first layer (m)
        number elements first layer
        factor of depth increase as in: dz[i]=dz[i-1]*(1+1/n)
        number of skin depths to bottom layer
    """
    dz = numpy.zeros(1) + d / m  # thickness of uppermost model layer
    z = numpy.zeros(1)  # initialize depth array at zero
    zmax = d * b  # depth of deepest model layer

    i = 0
    while z[i] < zmax:
        i += 1
        h = dz[i - 1] * (1 + 1 / n)  # geometrically increasing thickness
        dz = numpy.append(dz, h)  # thickness of layer i
        z = numpy.append(z, z[i - 1] + dz[i])  # depth of layer i

    return z


def transition_layers(p1: float, p2: float, H: float, z: numpy.array) -> numpy.array:
    """Change in properties over depth between two different layers.
    Using H-parameter of heat1d.

    Args:
        property of 1st layer (upper)
        property of 2nd layer (deeper)
        H-parameter (e-folding scale of property)
        depth array (m) [list]

    H=0.07 for the Moon
    """
    return p2 - (p2 - p1) * numpy.exp(-z / H)


def conductivity(k: float, t: float, r: float = R350_DEFAULT) -> float:
    """Calculate a temperature-dependent thermal conductivity.
    Based on Mitchell and de Pater (1994), McNoleg (1996), Vasavada et al. (2012) and heat1d.

    Args:
        conductivity (..)
        temperature (K) [list]
        grain-size related parameter
    """
    return k * (1 + r * t**3)


def heat_capacity(c: numpy.array, t: numpy.array) -> numpy.array:
    """Calculate a temperature-dependent heat capacity.
    Based on Ledlow et al. (1992), Hemingway et al. (1981) and heat1d.

    This is valid for T > ~10 K, yields negative (non-physical) values for T < 1.3 K.

    Args:
        polynomial coefficients of heat capacity (..) [list]
        temperature (K) [list]

    Moon: [8.9093e-9, -1.234e-5, 2.3616e-3, 2.7431, -3.6125]
    """
    return numpy.polyval(c, t)


def conduction_1d(
    t: numpy.ndarray,
    p: numpy.ndarray,
    c: numpy.array,
    a: numpy.array,
    b: numpy.array,
    dt: float,
) -> numpy.ndarray:
    """One-step conduction of temperatures inside of a 1d column.
    Based on heat1d.

    Args:
        temperature (K) [list of N]
        density and heat capacity (...) [list of N]
        alpha and beta (...) [list of N-2]
        delta time (s)

    Out:
        temperature (K) [list of N-2]

    Size of dx and diffu matches size of t[1:-1]
    t[:-2] is "before" / above / in the past
    t[2:] is "after" / deeper / the future
    """
    return t[1:-1] + dt / (p[1:-1] * c[1:-1]) * (
        a * t[:-2] - (a + b) * t[1:-1] + b * t[2:]
    )


def newton_method_fn(
    t: float, f: float, e: float, k: float, subt: numpy.ndarray, r350: float, dx: float
) -> float:
    return (
        f
        - STEFAN_BOLTZMANN * e * t**4
        + conductivity(k, t, r350) * (-subt[1] + 4 * subt[0] - 3 * t) / (2 * dx)
    )


def newton_method_dfn(
    t: float, e: float, k: float, subt: numpy.array, r350: float, dx: float
) -> float:
    return (
        -4 * STEFAN_BOLTZMANN * e * t**3
        + 3 * k * r350 * t**2 * (-subt[1] + 4 * subt[0] - 3 * t) / (2 * dx)
        - 3 * conductivity(k, t, r350) / (2 * dx)
    )
