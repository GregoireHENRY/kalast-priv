import numpy

import kalast

class ProgressDebug:
    frequency: str
    digits_full: int
    digits_decimal: int

    def __init__(
        self, frequency: str = "10", digits_full: int = 0, digits_decimal: int = 3
    ) -> None: ...

class Time:
    dt: float
    duration_total: float
    duration_record: float

    def __init__(
        self, dt: float = 0.0, duration_total: float = 0.0, duration_record: float = 0.0
    ) -> None: ...

class SkinDepthParams:
    diffusivity: float
    period: float

    def __init__(self, diffusivity: float = 0.0, period: float = 0.0) -> None: ...

class BodyDataMap:
    temperatures: list[numpy.array]
    thermal_properties_all: int
    thermal_properties_map: list[tuple[int, int]]

    def __init__(
        self,
        temperatures: list[numpy.array] = [],
        thermal_properties_all: int = 0,
        thermal_properties_map: list[tuple[int, int]] = [],
    ) -> None: ...

class Body:
    spin_period: float
    orbit_period: float

    def __init__(
        self,
        spin_period: float = 0.0,
        orbit_period: float = 0.0,
    ) -> None: ...

# class Record:
#     temperature_surface: bool
#     flux_surface: bool
#     surface_facets: list[int] | str
#     temperature_interior: bool
#     interior_time_indices: list[int]
#
#     def __init__(self) -> None: ...
#     def set_surface_facets_all(self) -> None: ...

class Setup:
    sun_position: numpy.ndarray
    thermal_properties: list[kalast.tpm.properties.Properties]
    bodies: list[Body]
    bodies_data_map: list[BodyDataMap]
    progress_debug: ProgressDebug
    time: Time

    def __init__(self) -> None: ...
    def prepare(self) -> None: ...
