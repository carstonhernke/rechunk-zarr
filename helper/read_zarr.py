import numpy as np
import zarr 

z = zarr.open('../data/example_input.zarr')

print(z[:])
