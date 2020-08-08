# This file is used only for IDE support of src/bindings.cpp.

cmake_minimum_required (VERSION 3.0)
project (skia-bindings)

include_directories(skia-bindings/skia)

add_compile_definitions(SK_SHAPER_HARFBUZZ_AVAILABLE, SK_VULKAN, SK_XML, SK_METAL, SK_DIRECT3D)

add_library(skiabindings
        skia-bindings/src/bindings.cpp
        skia-bindings/src/gl.cpp
        skia-bindings/src/gpu.cpp
        skia-bindings/src/paragraph.cpp
        skia-bindings/src/shaper.cpp 
        skia-bindings/src/svg.cpp
        skia-bindings/src/vulkan.cpp
        skia-bindings/src/metal.cpp
        skia-bindings/src/d3d.cpp
        )
