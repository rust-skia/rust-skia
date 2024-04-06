use super::ScriptRunIterator;
use crate::{Borrows, FourByteTag, Shaper};

pub fn script_run_iterator(
    utf8: &str,
    script: impl Into<Option<FourByteTag>>,
) -> Borrows<ScriptRunIterator> {
    let script = script.into();
    if let Some(tag) = script {
        Shaper::new_script_run_iterator(utf8, tag)
    } else {
        Shaper::new_hb_icu_script_run_iterator(utf8)
    }
}
