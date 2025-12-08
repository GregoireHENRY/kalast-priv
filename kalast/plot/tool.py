#!/usr/bin/env python

import itertools
from dataclasses import dataclass, field
from pathlib import Path

import matplotlib
import numpy
import scipy
from matplotlib import pyplot, ticker
from pyarrow import csv
import pyarrow

# import itertools

import kalast


@dataclass
class Legend:
    frame: bool = True
    show: bool = True


@dataclass
class Axis:
    label: str = None
    lim: tuple[float, float] = None
    loc: float = None
    scale: str = None


@dataclass
class Map:
    cbar_bottom: bool = True
    proj: str = "cyl"
    cmin: float = None
    ax: Axis = field(default_factory=Axis)
    vmin: float = None
    vmax: float = None
    cmap: matplotlib.colors.Colormap = None
    nx: int = None
    ny: int = None


@dataclass
class Data:
    x: numpy.array = None
    y: numpy.array = None
    z: numpy.array = None
    label: str = None
    color: str = "k"
    lw: float = 1
    ls: str | tuple = "solid"
    map: Map = None


@dataclass
class Config:
    name: str = None
    data: list[Data] = None
    xax: Axis = field(default_factory=Axis)
    yax: Axis = field(default_factory=Axis)
    map: Map = None
    grid: bool = False
    legend: Legend = field(default_factory=Legend)
    show: bool = False
    write: bool = True


def plot(cfg: Config):
    path = Path(__file__).parent.resolve()
    matplotlib.style.use(path / "main.mplstyle")
    fig, ax = pyplot.subplots(figsize=(15, 7.3))

    if cfg.map is not None:
        # if cfg.map.proj == "cyl":
        #     map = Basemap(
        #         ax=ax,
        #         projection="cyl",
        #         llcrnrlat=-90,
        #         urcrnrlat=90,
        #         llcrnrlon=-180,
        #         urcrnrlon=180,
        #         resolution="c",
        #     )
        # elif cfg.map.proj == "moll":
        #     map = Basemap(
        #         ax=ax,
        #         projection="moll",
        #         lon_0=0,
        #         resolution="c",
        #     )
        # elif cfg.map.proj == "ortho":
        #     map = Basemap(ax=ax, projection="ortho", lon_0=40, lat_0=20, resolution="l")
        # elif cfg.map.proj == "nsper":
        #     map = Basemap(
        #         ax=ax,
        #         projection="nsper",
        #         lon_0=40,
        #         lat_0=40,
        #         satellite_height=3000 * 1000.0,
        #         resolution="l",
        #     )

        # if cfg.map.proj is not None and cfg.map.proj != "cyl":
        #     map.drawparallels(numpy.arange(-90.0, 120.0, 30.0))
        #     map.drawmeridians(numpy.arange(0.0, 420.0, 60.0))

        norm = matplotlib.colors.Normalize(
            vmin=cfg.map.ax.lim[0], vmax=cfg.map.ax.lim[1]
        )

        lvl = 1
        lvls = numpy.arange(
            cfg.map.ax.lim[0], cfg.map.ax.lim[1] + lvl / 10, lvl, dtype=int
        )

        xy = numpy.column_stack((cfg.data[0].x, cfg.data[0].y))

        xgrid = numpy.linspace(-numpy.pi, numpy.pi, cfg.map.nx) * kalast.DPR
        ygrid = numpy.linspace(-numpy.pi / 2, numpy.pi / 2, cfg.map.ny) * kalast.DPR
        grid = numpy.array(numpy.meshgrid(xgrid, ygrid))

        fgrid = grid.reshape(2, -1).T
        zflat = scipy.interpolate.RBFInterpolator(xy, cfg.data[0].z)(fgrid)
        zgrid = zflat.reshape(cfg.map.nx, cfg.map.ny)

    if cfg.xax.label is not None:
        ax.set_xlabel(cfg.xax.label)

    if cfg.yax.label is not None:
        ax.set_ylabel(cfg.yax.label)

    if cfg.data is not None:
        for data in cfg.data:
            if cfg.map is None:
                ax.plot(
                    data.x,
                    data.y,
                    color=data.color,
                    lw=data.lw,
                    ls=data.ls,
                    label=data.label,
                )
            else:
                # tmap = map.contourf(*grid, zgrid, levels=lvls, cmap=cmap, norm=norm, latlon=True)
                # pcolormesh
                # contourf
                # shading="gouraud",
                # tmap = map.contourf(
                tmap = ax.contourf(
                    *grid,
                    zgrid,
                    cmap=cfg.map.cmap,
                    norm=norm,
                    latlon=True,
                    levels=lvls,
                )

                if cfg.map.cbar_bottom:
                    orientation = "horizontal"
                    pad = 0.05
                    shrink = 0.4
                    aspect = 30
                else:
                    orientation = "vertical"
                    pad = 0.02
                    shrink = 0.855
                    aspect = 23

                _ = fig.colorbar(
                    tmap,
                    label=cfg.map.ax.label,
                    orientation=orientation,
                    shrink=shrink,
                    pad=pad,
                    aspect=aspect,
                )

                cax = fig.axes[-1]

                loc = ticker.MultipleLocator(base=cfg.map.ax.loc)
                if not cfg.map.cbar_bottom:
                    if cfg.map.vmin:
                        cax.plot(
                            [0, 0.15],
                            [cfg.map.vmin, cfg.map.vmin],
                            color="k",
                            linewidth=0.9,
                        )
                        cax.text(
                            -0.08,
                            cfg.map.vmin,
                            f"{cfg.map.vmin:.0f}",
                            ha="right",
                            va="center_baseline",
                        )
                    if cfg.map.vmax:
                        cax.plot(
                            [0, 0.15],
                            [cfg.map.vmax, cfg.map.vmax],
                            color="k",
                            linewidth=0.9,
                        )
                        cax.text(
                            -0.08,
                            cfg.map.vmax,
                            f"{cfg.map.vmax:.0f}",
                            ha="right",
                            va="center_baseline",
                        )

                    if cfg.map.ax.loc is not None:
                        cax.yaxis.set_major_locator(loc)
                else:
                    if cfg.map.vmin:
                        cax.plot(
                            [cfg.map.vmin, cfg.map.vmin],
                            [0.75, 1.0],
                            color="k",
                            linewidth=0.9,
                        )
                        cax.text(
                            cfg.map.vmin,
                            1.1,
                            f"{cfg.map.vmin:.0f}",
                            ha="center",
                            va="bottom",
                        )
                    if cfg.map.vmax:
                        cax.plot(
                            [cfg.map.vmax, cfg.map.vmax],
                            [0.75, 1.0],
                            color="k",
                            linewidth=0.9,
                        )
                        cax.text(
                            cfg.map.vmax,
                            1.1,
                            f"{cfg.map.vmax:.0f}",
                            ha="center",
                            va="bottom",
                        )
                    if cfg.map.ax.loc is not None:
                        cax.xaxis.set_major_locator(loc)

    if cfg.grid:
        ax.grid(True)

    if cfg.xax.lim is not None:
        ax.set_xlim(cfg.xax.lim)

    if cfg.yax.lim is not None:
        ax.set_ylim(cfg.yax.lim)

    if cfg.xax.scale is not None:
        ax.set(xscale=cfg.xax.scale)

    if cfg.yax.scale is not None:
        ax.set(yscale=cfg.yax.scale)

    if cfg.xax.loc is not None:
        loc = ticker.MultipleLocator(base=cfg.xax.loc)
        ax.xaxis.set_major_locator(loc)

    if cfg.yax.loc is not None:
        loc = ticker.MultipleLocator(base=cfg.yax.loc)
        ax.yaxis.set_major_locator(loc)

    if cfg.legend.show:
        ax.legend(frameon=cfg.legend.frame)

    if cfg.name is not None:
        out = Path("out/surface")
        if not out.exists():
            out.mkdir(parents=True)

        fig.savefig(out / f"{cfg.name}.png", bbox_inches="tight", dpi=300)
        # fig.savefig(out / f"{cfg.name}.pdf", bbox_inches="tight")

    if cfg.write:
        write(cfg)

    if cfg.show:
        pyplot.show()


def write(cfg: Config):
    columns = []
    data = []
    for ii, data_ in enumerate(cfg.data):
        columns.append(f"x{ii}")
        columns.append(f"y{ii}")
        data.append(data_.x)
        data.append(data_.y)

        if cfg.map is not None:
            columns.append(f"z{ii}")
            data.append(data_.z)

    tab = pyarrow.table(data, names=columns)

    if cfg.name is not None:
        out = Path("out/surface")
        if not out.exists():
            out.mkdir(parents=True)
    csv.write_csv(tab, out / f"{cfg.name}.csv")


def plot_sphere(
    ax,
    c,
    r,
    texture: Path | None = None,
    color: str | None = None,
    count: int | tuple[int, int] | None = None,
    median_filter: int | None = None,
    **kw,
):
    if isinstance(count, int):
        count = (count, count)

    if texture is None:
        u = numpy.linspace(0, numpy.pi, count[0])
        v = numpy.linspace(0, 2 * numpy.pi, count[1])
        u, v = numpy.meshgrid(u, v)
        kw["fc"] = color
    else:
        img = pyplot.imread(texture)

        u = numpy.linspace(0, numpy.pi, count[0])
        v = numpy.linspace(0, 2 * numpy.pi, count[1])

        u_inds = numpy.arange(0, count[0], 1).round().astype(int)
        v_inds = numpy.arange(0, count[1], 1).round().astype(int)
        u_inds_img = numpy.linspace(0, img.shape[0] - 1, count[0]).round().astype(int)
        v_inds_img = numpy.linspace(0, img.shape[1] - 1, count[1]).round().astype(int)

        if median_filter == "auto":
            median_filter = (img.shape[0] // count[0], img.shape[1] // count[1])
        if median_filter is not None:
            mfd = (median_filter[0] // 2, median_filter[1] // 2)
        else:
            mfd = None

        new_img = numpy.zeros((count[0], count[1], 3))

        for xyz in range(3):
            for (x, y), (ii, jj) in zip(
                itertools.product(u_inds, v_inds),
                itertools.product(u_inds_img, v_inds_img),
            ):
                ii0 = max(0, ii - mfd[0])
                ii1 = min(ii + mfd[0] + 1, img.shape[1] - 1)
                jj0 = max(0, jj - mfd[1])
                jj1 = min(jj + mfd[1] + 1, img.shape[1] - 1)

                if mfd[0] > 1 | mfd[1] > 1:
                    new_img[x, y, xyz] = img[ii0:ii1, jj0:jj1, xyz].mean() / 255
                else:
                    new_img[x, y, xyz] = img[ii, jj, xyz] / 255
        img = new_img

        u, v = numpy.meshgrid(u, v)
        kw["facecolors"] = img
        kw["cstride"] = 1
        kw["rstride"] = 1

    x = r[0] * numpy.sin(u) * numpy.cos(v)
    y = r[1] * numpy.sin(u) * numpy.sin(v)
    z = r[2] * numpy.cos(u)

    ax.plot_surface((x - c[0]).T, (y - c[1]).T, (z - c[2]).T, **kw)
