from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib.font_manager as fm

def use_sf_fonts():
    """
    Configure matplotlib to use Apple SF fonts if available.
    Falls back gracefully if fonts are not found.
    """

    # Common SF font names macOS exposes
    preferred_fonts = [
        "SF Pro Text",
        "SF Pro Display",
        "SF Mono"
    ]

    available_fonts = {f.name: f.fname for f in fm.fontManager.ttflist}

    for font in preferred_fonts:
        if font in available_fonts:
            plt.rcParams.update({
                "font.family": "sans-serif",
                "font.sans-serif": [font],
                "axes.titlesize": 12,
                "axes.labelsize": 10,
                "xtick.labelsize": 9,
                "ytick.labelsize": 9,
                "figure.titlesize": 13,
            })
            print(f"Using font: {font}")
            return

    print("SF fonts not found — using default matplotlib font.")