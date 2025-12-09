#!/usr/bin/env python

# Remove stubs in kalast source to clean up.

import stubs_common as common


if __name__ == "__main__":
    for pyi in common.KALAST.rglob("*.pyi"):
        pyi_rel_root = pyi.relative_to(common.ROOT)

        print(f"Deleting {pyi_rel_root}")
        pyi_rel_root.unlink()
