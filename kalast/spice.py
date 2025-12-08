import numpy
import spiceypy as spice

from kalast._rs.spice import (  # noqa
    BODIES,
    CAMERAS,
    SPACECRAFTS,
    Body,
    Camera,
    Spacecraft,
)

# Can define a new body like that.
# DIDYMOS = Body()
# DIDYMOS.id = 65803
# DIDYMOS.name = "DIDYMOS"
# DIDYMOS.frame = "DIDYMOS_FIXED"
# DIDYMOS.radii = numpy.array([409.5, 400.5, 302.5])
# DIDYMOS.orbit_period = 700 * 24 * 3600
# DIDYMOS.spin_period = 2.26 * 3600


def cam_cpt_surf(ray: numpy.array, cam: Camera, bod: Body, et: float):
    """
    ray in camera frame
    """
    sp, _, sv = spice.sincpt(
        "ellipsoid", bod.name, et, bod.frame, "none", cam.name, cam.frame, ray
    )
    h = numpy.linalg.norm(sv)

    _, lo, la, pha, inc, emi = lolapha_planeto(sp, cam, bod, et)
    return sp, h, lo, la, pha, inc, emi


def cam_cpt_surf_2(ray: numpy.array, cam: Camera, bod: Body, et: float):
    """
    ray in camera frame
    """
    # bsight ray start pos and vector in body frame
    (p, lt_) = spice.spkpos(cam.name, et, bod.frame, "none", bod.name)
    m = spice.pxform(cam.frame, bod.frame, et)
    v = m @ ray * numpy.linalg.norm(p)
    sp = spice.surfpt(p, v, bod.radii[0], bod.radii[1], bod.radii[2])

    h, lo, la, pha, inc, emi = lolapha_planeto(sp, cam, bod, et)
    return sp, h, lo, la, pha, inc, emi


def subobs(obs: Body | Spacecraft, bod: Body, et: float):
    sp, _, sv = spice.subpnt(
        "intercept/ellipsoid", bod.name, et, bod.frame, "none", obs.name
    )
    h = numpy.linalg.norm(sv)

    _, lo, la, pha, _, _ = lolapha_planeto(sp, obs, bod, et)
    return sp, h, lo, la, pha


def lolapha_planeto(sp: numpy.array, obs: Body | Spacecraft, bod: Body, et: float):
    # planetographic
    (lo, la, h) = spice.recpgr(bod.name, sp, bod.radii[0], bod.flattening)
    _, _, pha, inc, emi = spice.ilumin(
        "ellipsoid", bod.name, et, bod.frame, "none", obs.name, sp
    )
    return h, lo, la, pha, inc, emi


def fovcov(d: float, cam: Camera, bod: Body) -> tuple[float, float, float]:
    proj = d * numpy.atan(cam.fov)
    res = proj / cam.px
    area_px = res[0] * res[1]
    visible_area_targ = numpy.pi * bod.radius**2
    covpx = numpy.clip(numpy.floor(visible_area_targ / area_px), 0, cam.npx)
    cov = (covpx / cam.npx) * 100.0
    return res, covpx, cov
