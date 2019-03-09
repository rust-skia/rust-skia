use crate::prelude::*;
use skia_bindings::{
    SkYUVAIndex,
    SkColorChannel
};

#[derive(Copy, Clone)]
pub struct ColorChannel(pub(crate) SkColorChannel);

impl ColorChannel {
    pub const R: Self = Self(SkColorChannel::kR);
    pub const G: Self = Self(SkColorChannel::kG);
    pub const B: Self = Self(SkColorChannel::kB);
    pub const A: Self = Self(SkColorChannel::kA);
}

#[derive(Copy, Clone)]
pub struct YUVAIndex(pub(crate) SkYUVAIndex);

impl NativeTransmutable<SkYUVAIndex> for YUVAIndex {}

#[test]
fn test_yuva_index_layout() {
    YUVAIndex::test_layout()
}

impl YUVAIndex {

    pub fn new(index: Option<(usize, ColorChannel)>) -> YUVAIndex {
        match index {
            Some((index, channel)) => {
                assert!(index < 4);
                YUVAIndex::from_native(SkYUVAIndex {
                    fIndex: index.try_into().unwrap(),
                    fChannel: channel.0
                })
            },
            None => {
                YUVAIndex::from_native(SkYUVAIndex {
                    fIndex: -1,
                    fChannel: ColorChannel::A.0
                })
            }
        }
    }

    pub fn are_valid_indices(indices: &[YUVAIndex; 4]) -> Option<usize> {
        let mut num_planes = 0;
        unsafe {
            SkYUVAIndex::AreValidIndices(indices.native().as_ptr(), &mut num_planes)
        }.if_true_some(num_planes.try_into().unwrap())
    }
}


