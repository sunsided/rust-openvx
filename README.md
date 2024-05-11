# OpenVX 1.3 meets Rust

![](https://img.shields.io/badge/OpenVX-1.3.1-blue)
![](https://img.shields.io/badge/OpenCV-4.6-blue?logo=opencv)
[![codecov](https://codecov.io/gh/sunsided/rust-openvx/graph/badge.svg?token=DURM37BW0T)](https://codecov.io/gh/sunsided/rust-openvx)

This repo consists of the following parts:

- [openvx-sys](openvx-sys) is the bindgen-generated Rust wrapper around OpenVX,
- [openvx](openvx) contains a Rust-y wrapper around the sys crate.

Test applications:

- [canny-edge-detector](canny-edge-detector/src/main.rs) contains a port of the Canny Edge Detector sample
  of the [OpenVX Samples](https://github.com/KhronosGroup/openvx-samples).

Here's an example input and output of the Canny Edge Detection sample.

![](.readme/canny.png)

```text
VX_TYPE_GRAPH: CANNY_GRAPH, 3 nodes, VX_GRAPH_STATE_COMPLETED, avg perf 0.044175636, 0 parameters, 1 refs
VX_TYPE_NODE: RGB_TO_YUV, 2 params, avg perf 0.002818527, VX_SUCCESS, 1 refs
VX_TYPE_NODE: EXTRACT_LUMA, 3 params, avg perf 0.002013746, VX_SUCCESS, 1 refs
VX_TYPE_NODE: CANNY_EDGE, 5 params, avg perf 0.039333154, VX_SUCCESS, 1 refs
```

Note that for some reason the graph release operation

```rust
vxReleaseGraph(&mut graph);
```

currently produces the following log messages with the associated status `VX_ERROR_INVALID_REFERENCE`:

```text
Failed to remove kernel[0]=org.khronos.extra.edge_trace
Failed to remove kernel[1]=org.khronos.extra.euclidean_nonmaxsuppression_harris
Failed to remove kernel[2]=org.khronos.extras.harris_score
Failed to remove kernel[3]=org.khronos.extras.laplacian3x3
Failed to remove kernel[4]=org.khronos.extras.image_to_list
Failed to remove kernel[5]=org.khronos.extra.nonmaximasuppression
Failed to remove kernel[6]=org.khronos.extra.elementwise_norm
Failed to remove kernel[7]=org.khronos.extras.scharr3x3
Failed to remove kernel[8]=org.khronos.extras.sobelMxN
```

The OpenVX 1.3 Quick Reference Guide was added [here](reference/openvx-13-reference-card.pdf)
for convenience.

## WIP

To install Intel OpenCL on Ubuntu 20.04, run

```bash
sudo apt-get install ocl-icd-opencl-dev intel-opencl-icd
```

Test using `clinfo`.

Install the [Khronos OpenVX Sample Implementation](https://github.com/KhronosGroup/OpenVX-sample-impl)

```bash
git clone --recursive https://github.com/KhronosGroup/OpenVX-sample-impl
```

I built from commit [`9caba36`](https://github.com/KhronosGroup/OpenVX-sample-impl/tree/9caba36a541fa6480f9d65d9a78dde208ebe96f9) by using

```bash
python Build.py \
    --os=Linux --arch=64 --conf=Release \
    --c=clang --cpp=clang \
    --conf_vision --conf_nn \
    --opencl_interop --enh_vision --ix \
    --streaming --pipelining
```

The commit was added as a git submodule at [vendor/openvx-sample-impl](vendor/openvx-sample-impl) for convenience.

Building using `--conf_nnef` (the Neural Network Exchange Format) didn't work for some
reason, and specifying `--opencl` always targeted NEON architecture, so didn't work.

To build the Rust projects, make sure the OpenVX library and headers are in scope:

```bash
export OPENVX_DIR=/path/to/OpenVX-sample-impl/install/Linux/x64/Release
export LIBRARY_PATH=$LIBRARY_PATH:$OPENVX_DIR/bin
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$OPENVX_DIR/bin
```

## Further reading

- [OpenVX 1.3 Sample Implementation: Sample Build Instructions](https://github.com/KhronosGroup/OpenVX-sample-impl#sample-build-instructions)
- [Installing Intel® oneAPI Toolkits via Linux* Package Managers](https://software.intel.com/content/www/us/en/develop/articles/oneapi-repo-instructions.html)
- [Install Intel® Distribution of OpenVINO™ toolkit for Linux* Using APT Repository](https://docs.openvinotoolkit.org/latest/openvino_docs_install_guides_installing_openvino_apt.html)
- [Intel® Distribution of OpenVX* Implementation Developer Guide](https://software.intel.com/content/www/us/en/develop/documentation/openvino-ovx-guide/top.html)
