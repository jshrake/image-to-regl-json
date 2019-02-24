# image-to-regl-texture-format

A utility to transform an image file into it's raw representation and output it as JSON for use with [regl](https://github.com/regl-project/regl). I built this to work around frustrations related to [https://github.com/regl-project/regl/issues/435](https://github.com/regl-project/regl/issues/435), and because it simply matches my workflow better. One downside is that the resulting JSON file can be fairly large, so consider compression.

See the flat array example here for the JSON schema [https://github.com/regl-project/regl/blob/7378e54ee54554a98f30e7c92d2946b7c8e192b1/API.md#texture-constructor](https://github.com/regl-project/regl/blob/7378e54ee54554a98f30e7c92d2946b7c8e192b1/API.md#texture-constructor).

# Run

```console
$ cargo run -- path/to/image.png > image.json
```
