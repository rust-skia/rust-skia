# This file is used only for IDE support of src/bindings.cpp.

cmake_minimum_required (VERSION 3.0)
project (skia-bindings)

include_directories(skia-bindings/skia)

add_library(skiabindings skia-bindings/src/bindings.cpp skia-bindings/src/shaper.cpp)
