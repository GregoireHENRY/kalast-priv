import numpy

from kalast._rs.entity import (  # noqa
    EARTH,
    MOON,
    MARS,
    PHOBOS,
    DEIMOS,
    DIDYMOS,
    DIMORPHOS,
    DIMORPHOS_PRE,
    TIRI,
    AFC,
    HERA,
    HALCA,
    MEX,
    TGO,
    Body,
    Camera,
    Spacecraft,
)

# Can define a new body like that.
DIDYMOS2 = Body()
DIDYMOS2.id = 65803
DIDYMOS2.name = "DIDYMOS"
DIDYMOS2.frame = "DIDYMOS_FIXED"
DIDYMOS2.radii = numpy.array([409.5, 400.5, 302.5])
DIDYMOS2.orbit_period = 700 * 24 * 3600
DIDYMOS2.spin_period = 2.26 * 3600
