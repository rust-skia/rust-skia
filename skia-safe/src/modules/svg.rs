use std::{
    error::Error,
    fmt,
    io::{self},
};

use skia_bindings::{self as sb, SkRefCntBase};

use super::resources::NativeResourceProvider;
use crate::{
    interop::{MemoryStream, NativeStreamBase, RustStream},
    prelude::*,
    Canvas, FontMgr, Size,
};

pub type Dom = RCHandle<sb::SkSVGDOM>;
require_base_type!(sb::SkSVGDOM, sb::SkRefCnt);
unsafe_send_sync!(Dom);

impl NativeRefCountedBase for sb::SkSVGDOM {
    type Base = SkRefCntBase;
}

/// Error when something goes wrong when loading an SVG file. Sadly, Skia doesn't give further
/// details so we can't return a more expressive error type, but we still use this instead of
/// `Option` to express the intent and allow for `Try`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoadError;

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to load svg (reason unknown)")
    }
}

impl Error for LoadError {
    fn description(&self) -> &str {
        "Failed to load svg (reason unknown)"
    }
}

impl From<LoadError> for io::Error {
    fn from(other: LoadError) -> Self {
        io::Error::new(io::ErrorKind::Other, other)
    }
}

impl fmt::Debug for Dom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dom").finish()
    }
}

impl Dom {
    pub fn read<R: io::Read>(
        mut reader: R,
        resource_provider: impl Into<NativeResourceProvider>,
        font_mgr: impl Into<FontMgr>,
    ) -> Result<Self, LoadError> {
        let mut reader = RustStream::new(&mut reader);
        let stream = reader.stream_mut();
        let resource_provider = resource_provider.into();
        let font_mgr = font_mgr.into();

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(stream, resource_provider.into_ptr(), font_mgr.into_ptr())
        };

        Self::from_ptr(out).ok_or(LoadError)
    }

    pub fn from_str(
        svg: impl AsRef<str>,
        resource_provider: impl Into<NativeResourceProvider>,
        font_mgr: impl Into<FontMgr>,
    ) -> Result<Self, LoadError> {
        Self::from_bytes(svg.as_ref().as_bytes(), resource_provider, font_mgr)
    }

    pub fn from_bytes(
        svg: &[u8],
        resource_provider: impl Into<NativeResourceProvider>,
        font_mgr: impl Into<FontMgr>,
    ) -> Result<Self, LoadError> {
        let mut ms = MemoryStream::from_bytes(svg);
        let resource_provider = resource_provider.into();
        let font_mgr = font_mgr.into();

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(
                ms.native_mut().as_stream_mut(),
                resource_provider.into_ptr(),
                font_mgr.into_ptr(),
            )
        };
        Self::from_ptr(out).ok_or(LoadError)
    }

    pub fn render(&self, canvas: &Canvas) {
        // TODO: may be we should init ICU whenever we expose a Canvas?
        #[cfg(all(feature = "embed-icudtl", feature = "textlayout"))]
        crate::icu::init();

        unsafe { sb::SkSVGDOM::render(self.native() as &_, canvas.native_mut()) }
    }

    pub fn set_container_size(&mut self, size: impl Into<Size>) {
        let size = size.into();
        unsafe { sb::C_SkSVGDOM_setContainerSize(self.native_mut(), size.native()) }
    }
}

#[cfg(test)]
mod tests {

    use super::Dom;
    use crate::{
        prelude::{NativeAccess, NativeRefCounted},
        resources::NativeResourceProvider,
        surfaces, FontMgr, Surface,
    };

    #[test]
    fn render_simple_svg() {
        // https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/410.svg
        // Note: height and width must be set
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" height = "256" width = "256">
            <path d="M30,1h40l29,29v40l-29,29h-40l-29-29v-40z" stroke="#;000" fill="none"/>
            <path d="M31,3h38l28,28v38l-28,28h-38l-28-28v-38z" fill="#a23"/>
            <text x="50" y="68" font-size="48" fill="#FFF" text-anchor="middle"><![CDATA[410]]></text>
            </svg>"##;
        let mut surface = surfaces::raster_n32_premul((256, 256)).unwrap();
        let canvas = surface.canvas();
        let font_mgr = FontMgr::new();
        let dom = Dom::from_str(svg, font_mgr.clone(), font_mgr).unwrap();
        dom.render(canvas);
        save_to_tmp(&mut surface, "simple");
    }

    #[test]
    fn resource_provider_and_font_mgr_get_dropped_after_drop_of_dom() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" height = "256" width = "256">
            </svg>"##;
        let mut surface = surfaces::raster_n32_premul((256, 256)).unwrap();
        let canvas = surface.canvas();
        let font_mgr = FontMgr::new();
        let provider: NativeResourceProvider = font_mgr.clone().into();
        let dom = Dom::from_str(svg, provider.clone(), font_mgr.clone()).unwrap();
        dom.render(canvas);
        // Dom keeps the resource provider even after rendering.
        assert!(provider.native()._ref_cnt() >= 2);
        // And at least two of the font managers are referred to (one in the resource provider, and the other in the Dom)
        assert!(font_mgr.native()._ref_cnt() >= 3);
        drop(dom);
        // now it's free.
        assert_eq!(1, provider.native()._ref_cnt());
        drop(provider);
        // and so is the font mgr
        assert_eq!(1, font_mgr.native()._ref_cnt());
    }

    // Run this manually (needs network connectivity)
    #[cfg(feature = "ureq")]
    #[test]
    #[ignore]
    fn render_svg_with_ureq_resource_provider() {
        use crate::resources::UReqResourceProvider;

        let svg = r##"
            <svg version="1.1"
            xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
            width="128" height="128">
            <image width="128" height="128" transform="rotate(45)" transform-origin="64 64"
                xlink:href="https://www.rust-lang.org/logos/rust-logo-128x128.png"/>
            </svg>"##;
        let mut surface = surfaces::raster_n32_premul((256, 256)).unwrap();
        let canvas = surface.canvas();
        let font_mgr = FontMgr::new();
        let resource_provider = UReqResourceProvider::new(font_mgr.clone());
        let dom = Dom::from_str(svg, resource_provider, font_mgr).unwrap();
        dom.render(canvas);
        save_to_tmp(&mut surface, "svg-with-ureq");
    }

    // A test case to see if a download error is handled.
    #[cfg(feature = "ureq")]
    #[test]
    fn render_svg_with_ureq_resource_provider_with_missing_resource() {
        use crate::resources::UReqResourceProvider;

        let svg = r##"
            <svg version="1.1"
            xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
            width="128" height="128">
            <image width="128" height="128" transform="rotate(45)" transform-origin="64 64"
                xlink:href="https://www.not-existing.org/logos/rust-logo-128x128.png"/>
            </svg>"##;
        let mut surface = surfaces::raster_n32_premul((256, 256)).unwrap();
        let canvas = surface.canvas();
        let font_mgr = FontMgr::new();
        let resource_provider = UReqResourceProvider::new(font_mgr.clone());
        let dom = Dom::from_str(svg, resource_provider, font_mgr).unwrap();
        dom.render(canvas);
        save_to_tmp(&mut surface, "svg-with-ureq-missing-image");
    }

    // data: (png image taken from <https://stackoverflow.com/questions/5242319/what-does-this-mean-image-pngbase64>)
    #[test]
    fn svg_with_base64_image_with_escaped_encoding() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="256" height="256">
            <image width="256" height="256" xlink:href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAKsAAADVCAMAAAAfHvCaAAAAGFBMVEVYn%2BH%2F%2F%2F%2Bex%2B3U5vd7s%2Bfq8%2Fs0itq72PMLUPvtAAASvklEQVR4AbXBC0JqCQxEwT5Jd7L%2FHc8FdR4g%2BEGtEr8u%2FBHxu7otdzd%2FQPyqlmRp1Pw%2B8aukDfRa1fw28ZtWy4sa89vEb7LCi0zx28RvqgkvouW3id%2FU8pbtWmv5beJXRWNrRmp%2BnfhlHXZm%2BQPi95Vk%2FoD4fZbMHxC%2FryTzB8Tva435A%2BL3rcb8AfH7VjJ%2FQPy%2BHYk%2FIH5facwfEL8iaZcrnKyn%2BAPi57K2VL2WF1hJ%2FAHxQ2tJrg6HteXVjPkD4ge6V3J1%2BF97zhx%2BnXhWb8nacKXlnYPErxNPyfqw4ZYKVuUZdfhd4hmxunY73NICgfWMOvwm8ZQ1pMvlDZdaCic98kjV4beIp8ScpLvsSvhflzqQmqVLB281v0E8pc2bdNne8EayNTPNSbt02PBj4intcKltb%2FNibY%2BLf9aSO%2FyMeMo6XMva3g0vwrWsxvyMeEoc3knZ2g53ZaXa8DzxlHa4J23Jae5aycXTxFPa4WRdXAtdsivckZXG4TniKWtOSlre6y7LG651Wxq5OzxDPGUVIKNwX6ekCv%2B0ddglVPMM8ZQ10FJ4LGVvOEuXRl7OqnmGeEor4Ck%2BtnI1ZEvjDa%2FcPEM8ZQVY4RO9VqUlN%2F84PEM8JQ50cUgXH2mrKlyq5RniOQ4vVjPLHdu86OKGi2eIr%2BgNV6JwljmYO6zlbJsbWp4hPtVrjYpLLV7UHIp7rOVkixtaniE%2BU5I2Nc2FKJytZhTuiac5rLnh4hniEzUbDjXhn3g5W0nNA1aAKm7YPEN8bMecrZYLWl70hkcyBay5YfMM8aHI4aR7xAUVHyirOdhAmRsqniE%2BtOKsRjIXtDzmmRGHVmDFDRfPEB%2BJzMmO01xScdYnVRs6vPHMFG9W4ZrMM8RHouWw43DNhlDWiSVZY3nDoWYc3qzDNZlniPe6w4uoOFjcKhPXuJNWyG6VqjSuhm7%2BiZorUfEM8U5J8nKyMw0tcZLwPxdRtTlUcUgVdGlml0uZ4pqKZ4hr5VUnpSXdUgVa4hA5vHERV1Tp9XhdJTWHksYd%2Ftdarql4hrjQiaPiYLclNSeebVYz5o0W7Ghsa9blmlFtx01rxP8yy5XIPEP8L1W7bjWHlbzhRTwjzXrCK1f3qqSEyBysLVtayKp40yqurcITxJtUgavVHNob%2FinZTWt5VVvWVKvJSttQCkRjb%2FA4vLK5thOeIN6sm9ai5cTFhYRDy%2FyTGpdU0hxkaZvWUrZluTmLims14QniVbywClqgeouT9IZXNWoupGzNqHa3y5LGVYBnipbCSVxcq1meIN54oRXsbEk26S3NmBcZ807K3gon2ZLcxF5tPMVJprlWE54g3nihtbRHm7WjkbxTHSCWwj1r2U4HSMmdQEmWwonNtah4gnhjA9ZSaohmpnpDjWRptDwS25LcQGsc2Bla5sTFtZV4gnixpWmIVWpgRuVwsiV5q7kv0JJcNVIFapydUrHTQKa5IfMEcRKrurSQ0qhsmVR4kea%2B7pIr9NqSrRltWlaxomUgVVyLxBPEYeUGygtszew2KfOBclVpVN2ctCXNidZaaKWmONhc6rKaJwi6xuGkRmWpAkRa7outF9XN%2F7LlmbJmpiCyvBxk%2FtnSqHmGWGk5i2ZcaWBLau5KKHt3Ce%2FsaLMz46VG4cTFm%2FaMOzxFUYWztjzhkNI43JPyYvPAegPxzFRpOYmWF1WywrPUag5xjRapqqxxubijvYFVaC%2Fv7YSDpzxjzlbhpKXxhqcpWshqtECk0Yys6m5utZdD1LCuCifhfyVOapqsxhyiQMmSm58QNdZheZGV5FqwueXiZBUga28DvRte1NQCpQVSUkFqPbIr%2FIxg7arwJqqEg6e5Vuas1Zytyw1ka5uT9ajKI87WbksaLT8mbkXFyWqaa2rOVuFVStUNpGrDoSTPmDfWdlby8kPiHQtoa0vLpXU4WzX%2FS5W2gWxtOHQ24U3CSUmu8BPinR2XVSFyuNAOZ9Fyae1qDu2qcF8suRKeJt7pcW1zaE9xwcVZq7nWtpeTrQ0PrEeq8CTxnsWrlbThELra5ixqbsXWNoeq6nBft6TlOeK9VnG2lfb4TKOOlpOouKPsWg4pb3Nf1uMGusP3iDtKDaTcgMuWvL1FmZOouCtlbwJs1Yb7SuN2Nd8k7mgvXV4OKWALiGkVJ14eyPqQQG9Vc0dWGnn5LnFPTW1z1gW0OdSyag5aHsvaroVs1YZL2dKMt1nzXeKulas52QLanGy3xq4a87Eu2yHZ2uZNWzPjDbDmu8R9a8m7iQNscbKyy%2BWS%2BUzWtqp7qzpA1jPj8KKK7xIPZG2NVWTTSbpKbs5cfEF6y64qV6ctqcKbdvgm8VhSlnWwJbuaV3LzRb11onFt%2BKcVvkl8one7u3bD%2FzJuXnRt%2BFTXVHOWqubQ4rvEEyI1L1Z2h8%2B0eRHLKiBqvkk8IePmxZq1lk%2B0w0nJUHKIlm8ST8ioeVEFtFwbPhA3h8gcdpZV803iCRkVL7Y42bK2w0NlDqXlpJRV803iGZYrnFRxlqwO3eEuN4dSOGlVme8Sz7C37QZqeZPekl0b3nMBreKsp1bNN4lnWIEtF1Vc6i1bVZtwxQX0NC9UrfBN4hk7zaHLNrey1kgVLljATnO2rmj5JvEMqzlrF%2B%2BFXitcsAArnFkdLd8knrFqPmFzyQq0xUm0tJZvEs8oAR0eix0u1ARSqg70NNHyTeIZUqgZ85gdLlgcMjOSRlBqvkk8wwOSp3moJlyoCYfeKkmBVvgm8YyaUJJ5zOJSTXMWSgus%2BC7xjJpA%2BMiquVATXiUcSuGbxDNqmk%2BUxtW82WmurMI3iWd4wifaHo1rNxx2miul8E3iGTXhc4nH0lQ1O80VK3yTeEYNX5SspbEnXFmFbxLPqGm%2BrsvWFFdK4ZvEM2rCt6RmzCWL7xLP2Anfs2M3Fyy%2BSzyjpvmqDoed5YrFd4ln7DRftHI19BRXSuGbxDN6wtdEqjF4lisS3yWeEYUvWlkDNeZKTfgm8ZFu7mqFr%2FKMYae4lFH4JvGBVLgraghf09uQMZdabr5JfKC2q1zV3IgarOLLPMWllptvEo%2B1e7dkq5ZrLkip%2BKqa4lLk5ZvEY15INay9XIqXVGS%2BqsdcirzclYVa7hAPbQFVnJSaC9HCapavqjGXIjXvbNmSxi7eE4%2BsA21OumwuSQUJX1ZjLsVabqR6t7tUlrThhnjEC%2FFy6AKbCy45zdftmEutKm5UcSgHspY7XBEPVAFVHLoCUXPFkr3hi2wutba44QDr5iyeqQ3%2FiAccqOLQDhAV17pG0jZfUuZS5OJaGYiWF%2B2ypOV%2F4q5UQZtDu4G2xK10aeTlC1bhUslciQpYh7PSQtau8ErcVYZ4gXYDcUXLe1lrvBU%2B0VoutFRcWQWo4qwdTlYSr8Q9caDMwc3BDgl3xZpRb%2FORnuVCJHNlla2oOYmLQ8q7Ll6Ie6pgDaQKSCl8IF3WqAgPrbgU2VxpV1kje2EdoOWGlsOJuKMd1g14OdjNp1YjNY%2B0m0s15kYgJVlaFxBVOETuAOK9eEELrDmUli%2Fo8oy94S4Xl2LzQGukEFU46RptQLy3BWWgHSBTvEp32eGRtjTjSriQBKLlShUPrSRcnK2qtIB4Zw3tQNRAbF5FB0vhoS57JFXzZmUtuLiy5gNlTTixlkgB8Y4byhAX0HJ4Y%2FcmWkjz0NrSaMNJ5EiNi3%2FSpPlIayqA3UBcIG5tQTuwBcQOJx3AsrSzxHJ4bKs9U5xoqWnK4U17%2BUzPFLQ4iQ3iRtxQC3gBK5xZJjOutcaSpeYjsUZqKFmGOLxIaflU1jI2ZzuLuLGuLe2yBlrLC1tdWg7ZmWal8KHeGtXG0gLLSdZyha%2BoKYdDl7WIGxpbI7lSicyLqFkH2rVZF%2BwUnymNXNu8WUkVLqSaB6IpIGWXF3Ft1UC6rRq3mhc7TRXgLS2lrKb5VEoz6nCSrtE2V6p4aMeQ8tJaxLU4nGU9o%2BXVTrMF%2BLBgjYqvSNkjL%2BDxhmut5tDb3CF1uwJoEdday6vMTHjVs7GA3g3QU8tXxZJc6Q23yhxWckPCtZW1nLgQ12KFF5Ed3pQ0U7yKp%2Fi6YM%2FI4dZOA3FRRdvhSmaWMxtxI3JzVlP8k9qsVFWdbVvTfENCjcytUoBW46XscE3DizLi1o6KQ4%2FDlZRsWSfBCt%2BSdHGrzGHFOtjFtUgNNJQR78Qjr%2BVwzV4I65SazPJzrQbKq6bl5kapU7bbRryXLo3c3LATYIfMEs3yc1bA44bScqumvJ21jLgrhHdSktNWkONR%2BLmULMnbpQm3pOWkZxHf0R7NKKykDr9iq3ptuexOuJQRZ5lCfE96K5Ct5iNpe118WQKxVeGCxnYDmUL8iUjb2%2BXmexIu9Di9XtgpxJ9wcehuOzwt1gJx4ynEM9K9tS5X7fLempP2dmnDczwjTlLYi%2FiCnHXSe9LWic9k3qvlRTltLU%2Bp2lE1sKUG8bm2DiNpNBpJu5vwwuEdLa%2FWy6p4JL27Dg%2B0pUBsQHxu67C1Vb2dpLlU5h3bG87aS0vNXWtJtip0bbjDhqgB8TkvH1g115qttnfDoW0oNe%2B1Rs0hlqVRc8cSmYP4XBUfUXHNlQ5tqzkpNaXmHVV4lVpq1NxjhYP43JqP2FwracOh7OZQDuXmRmu5sjMO75SWE%2FE5F4%2F09s5wI5abQ0rFoVxZNTes7e7wvy053NpwJj7n4kVCDt29teWypJHFOy0VJ6sN0CrK4dpakmv5pxQeEZ8rQ%2B9alnU2knyo2k64Ix4vh5I5sVNarqW3u8z%2F4mkeEZ8LrCxXtbfWu9t8qqQK0DKHVtEubrWm%2BZ9VPCS%2BJN1828oB4gqwalrFtUjNP3bzkPg7sdXAyhyssF4upWb5Z8c8Jv5QWmpgVRxUsGoulMw%2FPQqPiZ%2Fp8JGVOLQWKAW6%2BCcyF2qGD4gfibe2ead5lXEDpQAu0rv8r2WgtZxl1Twm%2Ftls1HxHK7HDjZV51VIgWmBlSeMKr%2BxseZYXq%2BUx8aY0MxrvVnUC4XNxgYtrJY15taMmNlAztd0lhxfW6MChC1rFY%2BLVjlwzKutVdfhE7xjKXEiX3CuHFzWG0lLycogUXnTtxuaws6DiMfFK09kZQ9K1VSvJ3oRHslIFWuGftdzQUoWzlYONinBILRdaC8TTYPO%2F3nBFnKxLG2um%2BKfXOrg6vBdLrvJSCm9SJpy0RtucrMRq1Zy1woUy0B4HbN60ex0uiEN0KLk1xZXs2paKW9FIqrJrzP%2Fs5k17tJz0GE%2FxohwulGElOUTmTRWl5oI4lKRRsTPhVpIdc6sl10IsFW9WXNpROPH0TkGAVnFpx5a63WSKN5HVXBKwc1btEffsNO8kvBObS5lZTnaUMXFYqbnUltwg75h%2FusMVATXleWW7qk1Xb8KLVfiKlsIlj9Sc1FhFtjITboSTlSp8QMCO5JU11bb1ZlQdIHL4iprmktWROclqGlaWmvsSPiRAIy3lcAhk05vsWgfbU3xFVFyRWTUvSqqa2S7zHEFmRikt7yS18kxxFj6yY67UbNu86U6qIApPEUSasZb7Ek0DqXh5LHa4lDFWc6kd4uUpgsiaKR6pKQ61uHmsZrmyk1ZxpQ1oeYoAzaG4ry1zsuXisVJxpeQdc60N2DxFgGckc1ePixdpHkjVjrnS0kpc6u5SwMtTBKxkybUJN3bUfCaulsMVTVvNP%2BmyNQVe7tjlE%2BJFb1mSLVfV9jaHHS2fiao15sqOd4pL29ArbxXvldV8TPwv6XVV6YXtGTefiiqaMRei2TFXKpzUONxKFWo%2BJt5J0ltlzQxfsCqimSpv86KmrHApBbXA2s2NuKPwMfFQvOELWgvsnEjVQMYtc2UXqjm0xI0yq%2FAx8T0JtyJz8DiekWpjxWoupRqqOamp5VJPsXJt9256wz3iW8oOt1xNaWah3NZJZK7UAg6HLo%2B5tFPgke2SreUe8R1rO9xayTpALFaa2Z3mUhyo4qQ6I67MbLlsyyfFPeI71m7ey0orw2pL256WuFILVHOI41mu1IyK3u0q28094nvCXQHLtqyF9Gq5tA7E4bAViRsrNW%2FCXeK3lDTVVoBI4ZIDVHFYpbTcyIbPiF%2FTSbPT3SUtl6qAuDl4W8UzxC%2Fz6CRciALUcijT4inil%2FV2p4pLtUDcwCol8xTxF8KlKg5VQGtb4jniz7UbWAcox%2BJJ4s%2B5OLiAVnuKJ4m%2FtuawBURbszxL%2FLF4OXgh9s7yNPHHqjisgVLLPE%2F8rXYD7UCrVsXzxN%2Bq4uAGrFj8gPhTXRzKwGprmh8Qf2rlot2AvSp%2BQvyl1nikAlprh58Qf0lqolGBarX8iPhLZWBVqnVsfkb8pTaHcru61PyM%2BEtrDq2UW8sPib%2FUChBvbIcfEn%2FKxWGrpeWnxJ9qVYDyVPgp8bfa2qRmmh8Tf21lq5qfE38uveE3%2FAdr385%2FSVd%2FMAAAAABJRU5ErkJggg%3D%3D"/>
            </svg>"##;
        let mut surface = surfaces::raster_n32_premul((256, 256)).unwrap();
        let canvas = surface.canvas();
        let font_mgr = FontMgr::new();
        let dom = Dom::from_str(svg, font_mgr.clone(), font_mgr).unwrap();
        dom.render(canvas);
        save_to_tmp(&mut surface, "svg-with-base64-image-escaped-encoding");
    }

    // data: (gif image from <https://www.rfc-editor.org/rfc/rfc2397>)
    #[test]
    fn svg_with_base64_image2() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="256" height="256">
            <image width="256" height="256" xlink:href="data:image/gif;base64,R0lGODdhMAAwAPAAAAAAAP///ywAAAAAMAAw
                AAAC8IyPqcvt3wCcDkiLc7C0qwyGHhSWpjQu5yqmCYsapyuvUUlvONmOZtfzgFz
                ByTB10QgxOR0TqBQejhRNzOfkVJ+5YiUqrXF5Y5lKh/DeuNcP5yLWGsEbtLiOSp
                a/TPg7JpJHxyendzWTBfX0cxOnKPjgBzi4diinWGdkF8kjdfnycQZXZeYGejmJl
                ZeGl9i2icVqaNVailT6F5iJ90m6mvuTS4OK05M0vDk0Q4XUtwvKOzrcd3iq9uis
                F81M1OIcR7lEewwcLp7tuNNkM3uNna3F2JQFo97Vriy/Xl4/f1cf5VWzXyym7PH
                hhx4dbgYKAAA7"/>
            </svg>"##;
        let mut surface = surfaces::raster_n32_premul((256, 256)).unwrap();
        let canvas = surface.canvas();
        let font_mgr = FontMgr::new();
        let dom = Dom::from_str(svg, font_mgr.clone(), font_mgr).unwrap();
        dom.render(canvas);
        save_to_tmp(&mut surface, "svg-with-base64-image2");
    }

    #[cfg(feature = "save-svg-images")]
    fn save_to_tmp(surface: &mut Surface, name: &str) {
        use crate::EncodedImageFormat;
        use std::{fs::File, io::Write, path::Path};

        let image = surface.image_snapshot();
        let data = image.encode(None, EncodedImageFormat::PNG, None).unwrap();
        write_file(data.as_bytes(), Path::new(&format!("/tmp/svg-{name}.png")));

        pub fn write_file(bytes: &[u8], path: &Path) {
            let mut file = File::create(path).expect("failed to create file");
            file.write_all(bytes).expect("failed to write to file");
        }
    }

    #[cfg(not(feature = "save-svg-images"))]
    fn save_to_tmp(_surface: &mut Surface, _name: &str) {}
}
