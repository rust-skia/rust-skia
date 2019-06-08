use crate::prelude::*;
use skia_bindings::{
    SkYUVAIndex,
    SkColorChannel
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ColorChannel {
    R = SkColorChannel::kR as _,
    G = SkColorChannel::kG as _,
    B = SkColorChannel::kB as _,
    A = SkColorChannel::kA as _
}

impl NativeTransmutable<SkColorChannel> for ColorChannel {}
#[test] fn test_color_channel_layout() { ColorChannel::test_layout() }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct YUVAIndex {
    index: i32,
    channel: ColorChannel
}

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
                Self {
                    index: index.try_into().unwrap(),
                    channel
                }
            },
            None => Self {
                index: -1,
                channel: ColorChannel::A
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
