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
Process 8.255 msec
from_raw
Process 0.652 msec
save ppm by image
Process 21.945 msec
from_vec
Process 0.754 msec
save png by image
Process 94.467 msec
rgb to gray
Process 11.745 msec
otsu binarize
Process 1.518 msec```

```
Camera /dev/video0: 640 * 360, 330 FPS
capture
Process 0.987 msec
from_raw
Process 0.168 msec
save ppm by image
Process 3.509 msec
from_vec
Process 0.116 msec
save png by image
Process 8.609 msec
rgb to gray
Process 0.958 msec
otsu binarize
Process 0.236 msec```
