#!/usr/bin/env python

from pathlib import Path

import matplotlib
from matplotlib import pyplot, ticker
import numpy


class Params:
    vmin = 220
    vmax = 360
    dv = 20

    szx = 0.3
    szy = 5

    label = "Temperature (K)"
    # Δ gravity norm (%)
    # Δ slope (°)
    # slope (°)

    orientation = "horizontal"
    # vertical

    show_min_max = False
    # aspect = 10
    # shrink = 0.1

    extend = "neither"
    # both min max

    bg = None
    # dark

    no_axes = False

    path = Path("cbar.png")
    dpi = "figure"  # 100, 400

    # cmap = matplotlib.cm.inferno
    # matplotlib.cm.Blues(numpy.linspace(0.1, 1.0, 200))
    # matplotlib.colors.LinearSegmentedColormap.from_list("map_blues_custom", cmap)

    mappable = None

    # def norm(self):
    #     # norm = matplotlib.colors.Normalize(vmin=vmin, vmax=vmax)
    #     return matplotlib.colors.Normalize(vmin=self.vmin, vmax=self.vmax)


def create(params: Params):
    # mapp = matplotlib.cm.ScalarMappable(cmap=params.cmap, norm=params.norm())

    # szz = params.szz
    szy = params.szy
    szx = params.szx

    if params.orientation == "horizontal":
        # szz = params.szy
        szy = params.szx
        szx = params.szy

    if params.bg == "dark":
        matplotlib.style.use("dark_background")

    # fig, ax = pyplot.subplots(figsize=(szx, szy))
    fig, ax = pyplot.subplots(figsize=(szx, szy))

    if params.no_axes:
        ax = pyplot.Axes(fig, [0.0, 0.0, 1.0, 1.0])
        ax.set_axis_off()
        fig.add_axes(ax)

    # pad=pad,
    # shrink=shrink,
    # aspect=aspect,
    _cb = fig.colorbar(
        params.mappable,
        label=params.label,
        orientation=params.orientation,
        cax=ax,
        extend=params.extend,
    )

    loc = ticker.MultipleLocator(base=params.dv)

    if params.orientation == "vertical":
        ax.yaxis.set_major_locator(loc)
        if params.show_min_max:
            ax.plot([0, 0.15], [params.vmin, params.vmin], color="k", linewidth=0.9)
            ax.plot([0, 0.15], [params.vmax, params.vmax], color="k", linewidth=0.9)
            ax.text(
                -0.08,
                params.vmin,
                f"{params.vmin:.0f}",
                ha="right",
                va="center_baseline",
            )
            ax.text(
                -0.08,
                params.vmax,
                f"{params.vmax:.0f}",
                ha="right",
                va="center_baseline",
            )
    elif params.orientation == "horizontal":
        ax.xaxis.set_major_locator(loc)
        if params.show_min_max:
            # ax.plot([params.vmin, params.vmin], [0.75, 1.0], color="k", linewidth=0.9)
            # ax.plot([params.vmax, params.vmax], [0.75, 1.0], color="k", linewidth=0.9)
            ax.plot([params.vmin, params.vmin], [1.0, 1.0], color="k", linewidth=1.0)
            ax.plot([params.vmax, params.vmax], [1.0, 1.0], color="k", linewidth=1.0)
            ax.text(params.vmin, 1.1, f"{params.vmin:.0f}", ha="center", va="bottom")
            ax.text(params.vmax, 1.1, f"{params.vmax:.0f}", ha="center", va="bottom")

    # pyplot.axis("off")

    fig.savefig(params.path, bbox_inches="tight", dpi=params.dpi)
    # fig.savefig(params.path.parent / "new.pdf", bbox_inches="tight")
    # fig.savefig(params.path.parent / "new.svg", bbox_inches="tight")

    # pyplot.show()


def custom_split_map(
    cmap1, cmap2, vmin: float, vmax: float, mid: float = None, name: str = "newmap"
) -> matplotlib.cm.ScalarMappable:
    colors1 = cmap1(numpy.linspace(0.0, 1.0, cmap1.N))
    colors2 = cmap2(numpy.linspace(0.0, 1.0, cmap2.N))
    colors_stacked = numpy.vstack((colors1, colors2))
    cmap3 = matplotlib.colors.LinearSegmentedColormap.from_list(name, colors_stacked)
    if mid is None:
        mid = (vmax - vmin) / 2
    norm = matplotlib.colors.TwoSlopeNorm(mid, vmin, vmax)
    return matplotlib.cm.ScalarMappable(cmap=cmap3, norm=norm)
