#!/usr/bin/env python

import numpy

import kalast
from kalast.util import AU, HOUR, DAY

setup = kalast.routines.setup.Setup()
# setup.sun_position = numpy.array([1.0, 0.0, 0.0]) * AU

# props = kalast.tpm.properties.Properties()
# props.albedo = 0.1
# props.emissivity = 0.9
# props.density = 2000.0
# props.heat_capacity = 600.0
# props.thermal_inertia = 400.0
# setup.thermal_properties = [props]

setup.time.dt = 1.0

# time = kalast.routines.setup.Time()
# time.dt = 300.0
# time.duration_total = 20.0 * DAY
# time.duration_record = 12.0 * HOUR
# setup.time = time

#

temperature_init = 290.0

p0 = numpy.array([100.0, 0.0, 0.0])
n0 = numpy.array([1.0, 0.0, 0.0])

spin_period = 6.0 * HOUR
spin_axis = numpy.array([0.0, 0.0, 1.0])

delta_depth = 1e-2
