# `openxr-6dof`

A hello world for bevy XR.

To run the code:
```bash
cargo run -p openxr-6dof
```

## Android

Download the [oculus sdk](https://developer.oculus.com/downloads/package/oculus-openxr-mobile-sdk/) and place `OpenXR/Libs/Android/arm64-v8a/Release/libopenxr_loader.so` into the `rumtime_libs/arm64-v8a/` folder.

Install `xbuild`. **It is very important to pass --git**: 
```sh
cargo install xbuild --git https://github.com/rust-mobile/xbuild
```

Plug in headset to PC, allow usb debugging, and then:
```sh
adb connect <device_ip> # Optional, allows wireless debugging
x devices # ensure your device is listed
x run --device <device_id_from_above> -p openxr-6dof --release
```
