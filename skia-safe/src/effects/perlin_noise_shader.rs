use crate::prelude::*;
use crate::{scalar, ISize, Shader};
use skia_bindings as sb;
use skia_bindings::SkShader;

impl RCHandle<SkShader> {
    pub fn fractal_perlin_noise(
        base_frequency: (scalar, scalar),
        num_octaves: usize,
        seed: scalar,
        tile_size: impl Into<Option<ISize>>,
    ) -> Option<Self> {
        fractal_noise(base_frequency, num_octaves, seed, tile_size)
    }

    pub fn turbulence_perlin_noise(
        base_frequency: (scalar, scalar),
        num_octaves: usize,
        seed: scalar,
        tile_size: impl Into<Option<ISize>>,
    ) -> Option<Self> {
        turbulence(base_frequency, num_octaves, seed, tile_size)
    }
}

pub fn fractal_noise(
    base_frequency: (scalar, scalar),
    num_octaves: usize,
    seed: scalar,
    tile_size: impl Into<Option<ISize>>,
) -> Option<Shader> {
    Shader::from_ptr(unsafe {
        sb::C_SkPerlinNoiseShader_MakeFractalNoise(
            base_frequency.0,
            base_frequency.1,
            num_octaves.try_into().unwrap(),
            seed,
            tile_size.into().native().as_ptr_or_null(),
        )
    })
}

pub fn turbulence(
    base_frequency: (scalar, scalar),
    num_octaves: usize,
    seed: scalar,
    tile_size: impl Into<Option<ISize>>,
) -> Option<Shader> {
    Shader::from_ptr(unsafe {
        sb::C_SkPerlinNoiseShader_MakeTurbulence(
            base_frequency.0,
            base_frequency.1,
            num_octaves.try_into().unwrap(),
            seed,
            tile_size.into().native().as_ptr_or_null(),
        )
    })
}
