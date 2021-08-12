# camera-image-processing-template

- This is camera image processing template
  - Get frame from camera
  - Convert camera frame to image::RGBImage
  - image process
    - as example
    - rgb image to gray image calculated by each pixel
    - gray image to binarized image calculated by imageproc::contrast::otsu_level and imageproc::contrast::threshold
- In detail, see src/main.rs

## Result

- see [Output data](data)
- Benchmark output

```
Camera /dev/video0: 1920 * 1080, 50 FPS
capture
Process 5.379 msec
from_raw
Process 0.591 msec
from_vec
Process 1.562 msec
rgb to gray
Process 12.432 msec
otsu binarize
Process 4.265 msec
```

```
Camera /dev/video0: 640 * 360, 330 FPS
capture
Process 1.002 msec
from_raw
Process 0.130 msec
from_vec
Process 0.058 msec
rgb to gray
Process 0.775 msec
otsu binarize
Process 0.194 msec
```
