# An attempt at using OpenVX in Rust

- [openvx-sys](openvx-sys) is the bindgen-generated Rust wrapper around OpenVX,
- [openvx](openvx) contains some Rusty helpers
- [canny-edge-detector](canny-edge-detector) contains a port of the Canny Edge Detector sample
  of the [OpenVX Samples](https://github.com/KhronosGroup/openvx-samples).

## WIP

To install Intel OpenCL on Ubuntu 20.04, run

```bash
sudo apt-get install \
    ocl-icd-opencl-dev \
    intel-opencl-icd
```

Test using `clinfo`.

Install the [Khronos OpenVX Sample Implementation](https://github.com/KhronosGroup/OpenVX-sample-impl)

```bash
git clone --recursive https://github.com/KhronosGroup/OpenVX-sample-impl#sample-build-instructions
```

I built from commit [`63ced2f`](https://github.com/KhronosGroup/OpenVX-sample-impl/tree/63ced2ff8d359a74937dead88aa687103244177e) by using

```bash
python Build.py \
    --os=Linux --arch=64 --conf=Release \
    --c=clang --cpp=clang \
    --conf_vision --conf_nn \
    --opencl_interop --enh_vision --ix \
    --streaming --pipelining
```

Building using `--conf_nnef` (the Neural Network Exchange Format) didn't work for some
reason, and specifying `--opencl` always targeted NEON architecture, so didn't work. 

## Further reading

- [OpenVX 1.3 Sample Implementation: Sample Build Instructions](https://github.com/KhronosGroup/OpenVX-sample-impl#sample-build-instructions)
- [Installing Intel® oneAPI Toolkits via Linux* Package Managers](https://software.intel.com/content/www/us/en/develop/articles/oneapi-repo-instructions.html)
- [Install Intel® Distribution of OpenVINO™ toolkit for Linux* Using APT Repository](https://docs.openvinotoolkit.org/latest/openvino_docs_install_guides_installing_openvino_apt.html)
- [Intel® Distribution of OpenVX* Implementation Developer Guide](https://software.intel.com/content/www/us/en/develop/documentation/openvino-ovx-guide/top.html)
