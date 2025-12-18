#!/usr/bin/env python

import numpy

import kalast
from kalast.util import AU, HOUR, DAY

setup = kalast.routines.setup.Setup()
setup.sun_position = numpy.array([1.0, 0.0, 0.0]) * AU

setup.thermal_properties = [
    kalast.tpm.properties.Properties(
        albedo=0.1,
        emissivity=0.9,
        density=2000.0,
        heat_capacity=600.0,
        thermal_inertia=400.0,
    )
]

setup.bodies = [
    kalast.routines.setup.Body(
        spin_period=6.0 * HOUR,
        # spin_axis=numpy.array([0.0, 0.0, 1.0]),
    )
]

setup.time.dt = 1.0
setup.time.duration_total = 20.0 * DAY
setup.time.duration_record = 12.0 * HOUR

#

temperature_init = 290.0

p0 = numpy.array([100.0, 0.0, 0.0])
n0 = numpy.array([1.0, 0.0, 0.0])

delta_depth = 1e-2
