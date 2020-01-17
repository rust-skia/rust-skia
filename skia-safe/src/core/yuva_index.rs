use crate::prelude::*;
use crate::ColorChannel;
use skia_bindings as sb;
use skia_bindings::SkYUVAIndex;

pub use skia_bindings::SkYUVAIndex_Index as Index;
#[test]
pub fn test_yuva_index_naming() {
    let _ = Index::Y;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct YUVAIndex {
    pub(crate) index: i32,
    pub(crate) channel: ColorChannel,
}

impl Default for YUVAIndex {
    fn default() -> Self {
        YUVAIndex::new(None)
    }
}

impl NativeTransmutable<SkYUVAIndex> for YUVAIndex {}

#[test]
fn test_yuva_index_layout() {
    YUVAIndex::test_layout()
}

impl YUVAIndex {
    pub const INDEX_COUNT: usize = 4;

    pub fn new(index: Option<(usize, ColorChannel)>) -> YUVAIndex {
        match index {
            Some((index, channel)) => {
                assert!(index < Self::INDEX_COUNT);
                Self {
                    index: index.try_into().unwrap(),
                    channel,
                }
            }
            None => Self {
                index: -1,
                channel: ColorChannel::A,
            },
        }
    }

    pub fn are_valid_indices(indices: &[YUVAIndex; Self::INDEX_COUNT]) -> Option<usize> {
        let mut num_planes = 0;
        unsafe { sb::C_SkYUVAIndex_AreValidIndices(indices.native().as_ptr(), &mut num_planes) }
            .if_true_then_some(|| num_planes.try_into().unwrap())
    }

    pub(crate) fn is_valid(self) -> bool {
        self.index >= 0 && self.index < Self::INDEX_COUNT as i32
    }
}
