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
### Grayscale
Img2header will retain the color channels present in the input image. However, a grayscale output can be generated with
```sh
img2header -o image.h --grayscale image.jpg
```
### Grayscale
To maintain alpha channel information use `alpha-channel` flag. This will also add opaque alpha channel if no alpha channel information is present in the input. Not compatible with `grayscale` flag.
```sh
img2header -o image.h --alpha-channel image.jpg
```
