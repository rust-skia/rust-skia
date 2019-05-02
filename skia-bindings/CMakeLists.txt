# This file is used only for IDE support of src/bindings.cpp.

cmake_minimum_required (VERSION 2.6)
project (skia-bindings)

include_directories("skia/include/core")
include_directories("skia/include/docs")
include_directories("skia/include/effects")
include_directories("skia/include/gpu")
include_directories("skia/include/pathops")
include_directories("skia/include/svg")
include_directories("skia/src/xml")

add_library(skiabindings src/bindings.cpp)
