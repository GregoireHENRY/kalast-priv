#!/usr/bin/env python

from pathlib import Path

import matplotlib
import numpy
import scipy
from matplotlib import pyplot, ticker, tri
# import pywavefront

import kalast


class Colorbar:
    min = None
    max = None
    step = None

    ticks = None

    formatter = None

    show_vmin_vmax = False
    vmin = None
    vmax = None

    extend = "neither"
    # neither
    # both
    # min
    # max

    label = None

    orientation = "horizontal"
    # horizontal
    # vertical

    cmap = matplotlib.cm.cividis
    norm = None
    norm_method = "linear"
    # linear
    # log

    mappable = None

    def get_mappable(self) -> matplotlib.cm.ScalarMappable:
        if self.mappable is not None:
            self.mappable
        return matplotlib.cm.ScalarMappable(cmap=self.get_cmap(), norm=self.get_norm())

    def get_cmap(self) -> matplotlib.colors.Colormap:
        if self.mappable is not None:
            return self.mappable.cmap
        return self.cmap

    def get_norm(self) -> matplotlib.colors.Normalize | None:
        if self.mappable is not None:
            return self.mappable.norm
        if self.norm is not None:
            return self.norm
        if self.norm_method == "linear":
            return matplotlib.colors.Normalize(vmin=self.min, vmax=self.max)
        elif self.norm_method == "log":
            return matplotlib.colors.LogNorm(vmin=self.min, vmax=self.max)

    def get_ticks(self) -> numpy.ndarray | None:
        if self.ticks is not None:
            return self.ticks
        if self.min is not None and self.max is not None and self.step is not None:
            return numpy.arange(self.min, self.max + self.step, self.step)

    def get_formatter(
        self,
    ) -> None | matplotlib.ticker.LogFormatter | matplotlib.ticker.Formatter:
        if self.formatter is not None:
            return self.formatter


class Contour:
    xy = None
    z = None

    nx = 40
    ny = 40

    method = "tricontour"
    # RBFinterp
    # triinterp
    # griddata
    # tricontour
    # mesh

    griddata_opt = "contour"
    # contour
    # pcolor
    # imshow

    mesh_path = None
    mesh_threshold_check = None

    def grid(self) -> (numpy.ndarray, numpy.ndarray):
        xx = numpy.linspace(-numpy.pi, numpy.pi, self.nx) * kalast.math.DPR
        yy = numpy.linspace(-numpy.pi / 2, numpy.pi / 2, self.ny) * kalast.math.DPR
        xgrid, ygrid = numpy.meshgrid(xx, yy)
        return xx, yy, xgrid, ygrid

    def create_map(
        self, ax: matplotlib.axes.Axes, cbar: Colorbar
    ) -> (
        matplotlib.tri.TriContourSet
        | matplotlib.contour.QuadContourSet
        | matplotlib.collections.PolyQuadMesh
        | matplotlib.image.AxesImage
        | list[matplotlib.patches.Polygon]
    ):
        if self.method == "tricontour":
            return self.tricontour(ax, cbar)
        elif self.method == "triinterp":
            return self.triinterp(ax, cbar)
        elif self.method == "RBFinterp":
            return self.RBFinterp(ax, cbar)
        elif self.method == "griddata":
            return self.griddata(ax, cbar)
        elif self.method == "mesh":
            return self.mesh(ax, cbar)

    def RBFinterp(
        self, ax: matplotlib.axes.Axes, cbar: Colorbar
    ) -> matplotlib.contour.QuadContourSet:
        xx, yy, xgrid, ygrid = self.grid()
        grid = numpy.array()
        fgrid = grid.reshape(2, -1).T
        zflat = scipy.interpolate.RBFInterpolator(self.xy, self.z)(fgrid)
        zgrid = zflat.reshape(self.nx, self.ny)
        return ax.contourf(
            xgrid,
            ygrid,
            zgrid,
            levels=cbar.levels(),
            cmap=cbar.get_cmap(),
            norm=cbar.get_norm(),
        )

    def triinterp(
        self, ax: matplotlib.axes.Axes, cbar: Colorbar
    ) -> matplotlib.contour.QuadContourSet:
        xx, yy, xgrid, ygrid = self.grid()
        triang = tri.Triangulation(self.xy[:, 0], self.xy[:, 1])
        interpolator = tri.LinearTriInterpolator(triang, self.z)
        xxgrid, yygrid = numpy.meshgrid(xgrid, ygrid)
        zz = interpolator(xxgrid, yygrid)
        return ax.contourf(
            xxgrid,
            yygrid,
            zz,
            levels=cbar.levels(),
            cmap=cbar.get_cmap(),
            norm=cbar.get_norm(),
        )

    def griddata(
        self, ax: matplotlib.axes.Axes, cbar: Colorbar
    ) -> (
        matplotlib.contour.QuadContourSet
        | matplotlib.collections.PolyQuadMesh
        | matplotlib.image.AxesImage
    ):
        xx, yy, xgrid, ygrid = self.grid()
        zz = scipy.interpolate.griddata(
            (self.xy[:, 0], self.xy[:, 1]),
            self.z,
            (xx[None, :], yy[:, None]),
            method="linear",
        )
        if self.method_opt == "contour":
            return ax.contourf(
                xx,
                yy,
                zz,
                cmap=cbar.get_cmap(),
                norm=cbar.get_norm(),
                levels=cbar.levels(),
            )

        elif self.method_opt == "pcolor":
            return ax.pcolor(xx, yy, zz, cmap=cbar.get_cmap(), norm=cbar.get_norm())

        elif self.method_opt == "imshow":
            return ax.imshow(
                zz,
                extent=(xx.min(), xx.max(), yy.min(), yy.max()),
                cmap=cbar.get_cmap(),
                norm=cbar.get_norm(),
            )

    def tricontour(
        self, ax: matplotlib.axes.Axes, cbar: Colorbar
    ) -> matplotlib.tri.TriContourSet:
        return ax.tricontourf(
            self.xy[:, 0],
            self.xy[:, 1],
            self.z,
            cmap=cbar.get_cmap(),
            norm=cbar.get_norm(),
            levels=cbar.levels(),
        )

    # def mesh(
    #     self, ax: matplotlib.axes.Axes, cbar: Colorbar
    # ) -> list[matplotlib.patches.Polygon]:
    #     scene = pywavefront.Wavefront(self.mesh_path, collect_faces=True)
    #     faces = scene.mesh_list[0].faces
    #     vertices = scene.vertices

    #     for ii in range(len(faces)):
    #         a = kalast.util.cart2sph(vertices[faces[ii][0]])[:2] * DPR
    #         b = kalast.util.cart2sph(vertices[faces[ii][1]])[:2] * DPR
    #         c = kalast.util.cart2sph(vertices[faces[ii][2]])[:2] * DPR
    #         s1 = b - a
    #         s2 = c - b
    #         s3 = a - c
    #         d1 = numpy.linalg.norm(s1)
    #         d2 = numpy.linalg.norm(s2)
    #         d3 = numpy.linalg.norm(s3)
    #         sph = numpy.array([a, b, c])

    #         cond = numpy.array([d1, d2, d3]) > self.mesh_threshold_check
    #         if cond.sum() >= 1:
    #             cond2 = sph[:, 0] == 180
    #             if cond2.sum() == 1:
    #                 sph[cond2, 0] = -180
    #             elif cond2.sum() == 2:
    #                 sph[cond2, 0] = -180

    #         lon = sph[:, 0]
    #         lat = sph[:, 1]
    #         value = self.z[ii]

    #         # color = kalast.util.cmapv_to_rbg(value, d["cmap"])
    #         # color = d["cmap"](value)

    #         value = cbar.get_norm()(value)
    #         color = cbar.get_cmap()(value)
    #         print(lon)
    #         print(lat)
    #         print(value)
    #         print(color)
    #         ax.fill(lon, lat, color=color, edgecolor="k", lw=1, joinstyle="bevel")


class Params:
    xlim = (-180, 180)
    ylim = (-90, 90)
    use_lims = True

    xloc = 30
    yloc = 30

    show_axes_label = False

    path = Path("out/smap_new.png")
    show = True

    contour = Contour()
    cbar = Colorbar()

    def create_map(
        self, ax: matplotlib.axes.Axes
    ) -> (
        matplotlib.tri.TriContourSet
        | matplotlib.contour.QuadContourSet
        | matplotlib.collections.PolyQuadMesh
        | matplotlib.image.AxesImage
        | list[matplotlib.patches.Polygon]
    ):
        return self.contour.create_map(ax, self.cbar)

    def plot(self):
        return plot(self)


def plot(params: Params):
    path = Path(__file__).parent.resolve()
    matplotlib.style.use(path / "main.mplstyle")

    # fig, ax = pyplot.subplots(figsize=(15, 7.3))

    fig, axs = pyplot.subplots(2, 1, figsize=(15, 7.3), height_ratios=[9.5, 0.5])
    ax = axs[0]

    if params.show_axes_label:
        ax.set_xlabel("Longitude (°)")
        ax.set_ylabel("Latitude (°)")

    if params.use_lims:
        ax.set_xlim(params.xlim)
        ax.set_ylim(params.ylim)

    loc = ticker.MultipleLocator(base=params.xloc)
    ax.xaxis.set_major_locator(loc)

    loc = ticker.MultipleLocator(base=params.xloc)
    ax.yaxis.set_major_locator(loc)

    _smap = params.create_map(ax)
    mappable = params.cbar.get_mappable()

    ax = axs[1]
    ax.set_visible(False)
    cax = fig.add_axes([0.25, 0.05, 0.5, 0.03])

    _cb = fig.colorbar(
        mappable,
        label=params.cbar.label,
        orientation=params.cbar.orientation,
        cax=cax,
        ticks=params.cbar.get_ticks(),
        format=params.cbar.get_formatter(),
    )

    params.path.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(params.path, bbox_inches="tight", dpi=300)

    if params.show:
        pyplot.show()
