use std::path::Path;

use super::prelude::BindgenArgsBuilder;

pub fn use_sdk_libcxx(builder: &mut BindgenArgsBuilder, sdk: &Path) {
    // Bindgen may load a non-Apple libclang, whose default C++ headers take precedence over the
    // Apple SDK selected by -isysroot. Mixing those headers with the SDK can fail when Xcode and
    // libclang ship incompatible libc++ versions, so make Bindgen use the SDK's libc++ explicitly.
    builder.bindgen_only_arg("-nostdinc++");
    builder.bindgen_only_arg(format!("-isystem{}/usr/include/c++/v1", sdk.display()));
}
