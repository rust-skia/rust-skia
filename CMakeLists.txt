# This file is used only for IDE support of bindings.cpp.

cmake_minimum_required (VERSION 2.6)
project (SkiaBindings)

include_directories("skia/include/core")
include_directories("skia/include/gpu")

add_library(skiabindings src/bindings.cpp)