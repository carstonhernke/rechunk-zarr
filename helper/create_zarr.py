import numpy as np
import zarr 

zarr.storage.default_compressor = None
numpy_array = np.random.randint(100, size=10000)

z = zarr.array(numpy_array, chunks=(1), dtype='i4', compressor=None)

print(z[:])
zarr.save('../data/example_input.zarr', z)