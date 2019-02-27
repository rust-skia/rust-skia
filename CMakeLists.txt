# This file is used only for IDE support of srd/bindings.cpp.

cmake_minimum_required (VERSION 2.6)
project (rust-skia)

include_directories("skia/include/core")
include_directories("skia/include/gpu")

add_library(skiabindings src/bindings.cpp)