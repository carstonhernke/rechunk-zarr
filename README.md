# rechunk-zarr
## A small demonstration of rechunking a Zarr file in-place using Rust

### Objective
To take a [Zarr file](https://zarr.readthedocs.io/en/stable/getting_started.html) containing a one-dimensional variable with N data points and N chunks, and rechunk it in-place so all data points are contained in one chunk.

### Usage
With Rust and Cargo installed, run `cargo run data/example_input.zarr` to rechunk the file in-place. `data/example_input.zarr` may be substituted with another file path.

### Assumptions
As this is a small demonstration project only, there are some significant assumptions and the code is likely not generalizable to most Zarr files in its current state.
- All values are 32-bit signed integers
- The input Zarr file contains only one one-dimensional variable
- No compression is applied
- The Zarr file follows [version 2 of the Zarr storage specification](https://zarr.readthedocs.io/en/stable/spec/v2.html)

### Helper scripts
The files `helper/create_zarr.py` and `helper/read_zarr.py` utilize the common [Zarr Python](https://zarr.readthedocs.io/en/stable/getting_started.html) implementation to create a sample .zarr file and to read the re-chunked .zarr file to ensure its integrity.