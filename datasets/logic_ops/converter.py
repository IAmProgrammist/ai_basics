from PIL import Image
import numpy as np
import os

for filename in os.listdir(os.getcwd()):
    if filename.endswith(".py"):
        continue

    # Load your image (can be any format)
    img = Image.open(filename).convert('L')  # Convert to grayscale

    # Convert to binary (1-bit)
    threshold = 128
    binary_img = img.point(lambda p: 255 if p > threshold else 0)

    # Convert to numpy array of 1s and 0s
    binary_array = (np.array(binary_img) > 128).astype(int)

    [directory, index] = filename.split("_")

    try:
        os.mkdir(directory)
    except:
        pass

    path = f"{os.getcwd()}\\{directory}\\{index.split('.')[0]}.bin"
    with open(path, "w") as file:
        pass

    # Save as text file
    np.savetxt(path, binary_array, fmt='%d', delimiter='')
