import numpy

# import kalast

class Time:
    dt: float
    duration_total: float
    duration_record: float

    def __init__(self) -> None: ...

# class ProgressDebug:
#     frequency: float
#     digits_full: int
#     digits_decimal: int
#
#     def __init__(self) -> None: ...
#
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
    # thermal_properties: list[kalast.tpm.properties.Properties]
    # bodies: list[Body]
    # bodies_data_map: list[BodyDataMap]
    # progress_debug: ProgressDebug
    time: Time

    def __init__(self) -> None: ...
    def prepare(self) -> None: ...
