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
Process 5.367 msec
from_raw
Process 0.552 msec
from_vec
Process 2.097 msec
```
