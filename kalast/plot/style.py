import matplotlib
from pathlib import Path

from matplotlib.ticker import FuncFormatter


def formatter_decimal_round(x, pos):
    if x.is_integer():
        return str(int(x))
    else:
        return str(x)


def use_formatter_decimal_round(ax: matplotlib.axes.Axes):
    formatter = FuncFormatter(formatter_decimal_round)
    ax.xaxis.set_major_formatter(formatter)
    ax.yaxis.set_major_formatter(formatter)


def load(style: str = "style1"):
    path = Path(__file__).parent.resolve()
    matplotlib.style.use(path / f"{style}.mplstyle")
