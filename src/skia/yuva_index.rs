use crate::prelude::*;
use rust_skia::{
    SkYUVAIndex,
    SkColorChannel
};

#[derive(Copy, Clone)]
pub struct ColorChannel(pub(crate) SkColorChannel);

impl ColorChannel {
    pub const R: ColorChannel = ColorChannel(SkColorChannel::kR);
    pub const G: ColorChannel = ColorChannel(SkColorChannel::kG);
    pub const B: ColorChannel = ColorChannel(SkColorChannel::kB);
    pub const A: ColorChannel = ColorChannel(SkColorChannel::kA);
}

#[derive(Copy, Clone)]
pub struct YUVAIndex(pub(crate) SkYUVAIndex);

impl YUVAIndex {
    pub fn new(index: Option<(usize, ColorChannel)>) -> YUVAIndex {
        match index {
            Some((index, channel)) => {
                assert!(index < 4);
                YUVAIndex(SkYUVAIndex {
                    fIndex: index.try_into().unwrap(),
                    fChannel: channel.0
                })
            },
            None => {
                YUVAIndex(SkYUVAIndex {
                    fIndex: -1,
                    fChannel: ColorChannel::A.0
                })
            }
        }
    }

    pub fn are_valid_indices(indices: &[YUVAIndex; 4]) -> Option<usize> {
        let index_slice : Vec<SkYUVAIndex> = indices.iter().map(|i| i.0).collect();

        let mut num_planes = 0;
        unsafe { SkYUVAIndex::AreValidIndices(index_slice.as_ptr(), &mut num_planes) }
            .if_true_some(num_planes.try_into().unwrap())
    }
}


