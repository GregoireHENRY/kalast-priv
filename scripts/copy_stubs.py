#!/usr/bin/env python

import stubs_common as common


if __name__ == "__main__":
    for pyi in common.STUBS.rglob("*.pyi"):
        pyi_rel = pyi.relative_to(common.STUBS)
        dest = common.KALAST / pyi_rel

        pyi_rel_root = pyi.relative_to(common.ROOT)
        dest_rel_root = dest.relative_to(common.ROOT)

        # check a .py exist for this .pyi
        py_rel_root = dest_rel_root.parent / f"{dest_rel_root.stem}.py"
        if not py_rel_root.exists():
            print(
                f"Cannot copy {pyi_rel_root} -> {dest_rel_root}, because no corresponding {py_rel_root}"
            )

        print(f"Copying {pyi_rel_root} -> {dest_rel_root}")
        pyi_rel_root.copy(dest_rel_root)
