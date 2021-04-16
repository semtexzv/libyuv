#!/usr/bin/env bash

git clone https://chromium.googlesource.com/libyuv/libyuv

bindgen --default-enum-style moduleconsts \
  ./libyuv/include/libyuv.h \
  -- -I./libyuv/include/ > ./src/ffi.rs