# img2header
img2header converts image files to C headers

## Install
```sh
cd img2header
cargo install --path .
```

## Usage
To generate a header containing image data call img2header with a img file.
```sh
img2header -o image.h image.jpg
```

## Supported image types
Img2header supports the following file types: `.bmp .jpg .png .gif .tiff`

## Specifying output format
The generated output can be adjusted with additional arguments.
### Data type
By default, img2header writes output as `uint8_t`. The data type can be changed using the `--data-type` argument.
```sh
img2header -o image.h --data-type int32_t image.jpg
```
### Grayscale
Img2header will retain the color channels present in the input image. However, a grayscale output can be generated with
```sh
img2header -o image.h --grayscale image.jpg
```
