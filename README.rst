======
kalast
======

Structure
=========

- `kalast/`: Python code
- `src/`: Rust code
- `examples`: Examples of usage of Kalast
- `res/`: resources folder (if missing get it from cloud-as.oma.be)

Compilation
===========

Build the kalast rust dynamic library `kalast/_rs.cpython-314-darwin.so` (example for Mac)

.. code:: sh

    uv run maturin develop

Run an example.

.. code:: sh

    uv run python -i examples/two_spheres/illum.py

Import kalast from Python.

.. code:: python
   
   # uv run python
   import kalast;