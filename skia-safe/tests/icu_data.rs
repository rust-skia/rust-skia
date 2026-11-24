//! Tests that the ICU data embedded in skia-bindings holds every item that Skia's ICU-based
//! `SkUnicode` loads.
//!
//! The build embeds only a subset of ICU's `icudtl.dat`, which the Skia fork's
//! `third_party/icu/filter_icu_data.py` selects. ICU reports a missing break rule file as an
//! error, but it does not report a missing dictionary or locale bundle. A break iterator then
//! silently loses its dictionary, or a locale silently falls back to the root rules. Each case
//! below therefore pins the boundaries that the complete ICU data produces, and names the data
//! items that these boundaries depend on.
//!
//! The line break cases select line rules through the `-u-lb-` and `-u-lw-` keywords of a
//! language tag and through the break bundles `root.res`, `ja.res`, `ko.res`, `zh.res`, and
//! `zh_Hant.res`. Together they load every line rule file in the ICU data.
//!
//! The filter also keeps `keyTypeData.res` and `timezoneTypes.res`, which no case covers. ICU uses
//! them to convert the `-u-` keywords of a language tag. Without them, ICU passes the keywords
//! through unchanged, and the break iterators read only `lb` and `lw`, so no boundary changes.
//!
//! Skia caches break iterators by break type and by the locale that ICU actually loaded. That
//! locale has no `-u-` keywords, and languages without a break bundle of their own share the
//! root locale. In one process, the first line break request therefore decides the rules of all
//! later requests that resolve to the same locale. So every case runs in a process of its own.
//!
//! The expected boundaries hold for the desktop ICU data, which the filter selects from. Android,
//! iOS, and WebAssembly builds embed the ICU data that Skia ships for those platforms, so the
//! tests do not run there.
#![cfg(all(
    feature = "textlayout",
    not(any(target_os = "android", target_os = "ios", target_arch = "wasm32"))
))]

use std::{
    env,
    ffi::{CString, c_void},
    process::Command,
};

use skia_bindings::{self as sb, SkUnicode_BreakType as BreakType};

/// Runs each case in a child process of this test binary, because Skia's break iterator cache
/// lets the first case decide the line rules of later cases (see the module documentation).
#[test]
fn break_iterators_find_the_boundaries_of_the_complete_icu_data() {
    if let Ok(index) = env::var(CASE_INDEX_VAR) {
        run_case_in_this_process(&CASES[index.parse::<usize>().unwrap()]);
        return;
    }

    let test_binary = env::current_exe().unwrap();
    let mut failures = Vec::new();
    for (index, case) in CASES.iter().enumerate() {
        let output = Command::new(&test_binary)
            .args([TEST_NAME, "--exact", "--nocapture"])
            .env(CASE_INDEX_VAR, index.to_string())
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        let ran = stderr.lines().any(|line| line == RAN_LINE);
        let failure = stderr
            .lines()
            .find_map(|line| line.strip_prefix(FAILURE_PREFIX));
        match (output.status.success(), ran, failure) {
            (true, true, None) => {}
            (_, _, Some(failure)) => failures.push(failure.to_owned()),
            (_, ran, None) => failures.push(format!(
                "{} ({}): the child process {}: {}{}",
                case.data,
                case.locale,
                if ran {
                    "failed after it ran the case"
                } else {
                    "did not run the case"
                },
                String::from_utf8_lossy(&output.stdout),
                stderr,
            )),
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

/// `uemoji.icu` holds the emoji properties. Without it, ICU reports no code point as an emoji.
#[test]
fn emoji_properties_are_available() {
    skia_safe::icu::init();
    let is_emoji = |c: char| unsafe { sb::C_SkUnicodes_ICU_isEmoji(c as i32) };
    assert!(is_emoji('😀'));
    assert!(is_emoji('\u{1FAE8}'));
    assert!(!is_emoji('a'));
}

/// The name of the test that runs the cases. It must match the test function's name, because a
/// child process selects the test by this name with `--exact`.
const TEST_NAME: &str = "break_iterators_find_the_boundaries_of_the_complete_icu_data";
/// The environment variable that tells a child process which case of [`CASES`] to run.
const CASE_INDEX_VAR: &str = "SKIA_SAFE_ICU_DATA_CASE";
/// The prefix of the line in which a child process reports a failing case.
const FAILURE_PREFIX: &str = "icu data case failed: ";
/// The line a child process prints after it ran its case.
const RAN_LINE: &str = "icu data case ran";

/// Runs one case in this process and reports its outcome on stderr, which the parent process
/// reads. The test harness writes its own output to stdout, without a line break before the test
/// output when it runs on one thread, so stdout cannot carry the report reliably.
fn run_case_in_this_process(case: &Case) {
    skia_safe::icu::init();
    let actual = breaks(case.locale, case.break_type, case.text);
    if actual.as_deref() != Some(case.expected) {
        eprintln!(
            "{FAILURE_PREFIX}{} ({}, {:?}): expected {:?} {:?}, got {:?} {:?}",
            case.data,
            case.locale,
            case.break_type,
            case.expected,
            segments(case.text, case.expected),
            actual,
            actual.as_deref().map(|a| segments(case.text, a)),
        );
    }
    eprintln!("{RAN_LINE}");
}

/// Returns the boundaries that Skia's ICU break iterator finds, or `None` if ICU cannot open the
/// break iterator.
fn breaks(locale: &str, break_type: BreakType, text: &str) -> Option<Vec<i32>> {
    unsafe extern "C" fn set_positions(ptr: *mut i32, len: usize, positions: *mut c_void) {
        let positions = unsafe { &mut *positions.cast::<Vec<i32>>() };
        if len > 0 {
            positions.extend_from_slice(unsafe { std::slice::from_raw_parts(ptr, len) });
        }
    }

    let locale = CString::new(locale).unwrap();
    let mut positions = Vec::new();
    let mut sink = sb::VecSink {
        data: (&mut positions as *mut Vec<i32>).cast(),
        set_fn: Some(set_positions),
        _phantom_0: Default::default(),
    };
    let opened = unsafe {
        sb::C_SkUnicodes_ICU_breaks(
            locale.as_ptr(),
            break_type,
            text.as_ptr() as _,
            text.len(),
            &mut sink,
        )
    };
    opened.then_some(positions)
}

/// Splits `text` at `positions`, so that a failure shows the segments instead of offsets.
fn segments<'a>(text: &'a str, positions: &[i32]) -> Vec<&'a str> {
    positions
        .windows(2)
        .map(|w| {
            text.get(w[0] as usize..w[1] as usize)
                .unwrap_or("<not a char boundary>")
        })
        .collect()
}

/// One break iterator query and the boundaries the complete ICU data produces for it.
struct Case {
    /// The ICU data items the expected boundaries depend on. A break bundle `brkitr/*.res`
    /// stands for the rule file it selects for the given keywords.
    data: &'static str,
    /// A BCP 47 language tag.
    locale: &'static str,
    break_type: BreakType,
    text: &'static str,
    /// The boundaries as UTF-8 offsets into `text`.
    expected: &'static [i32],
}

/// Latin text with CJK characters, whose line breaks differ between the root line rules.
const LATIN_LINE: &str = "Hello world—again (really)… e.g. 3/4 x-y ぁー々";
/// Japanese text with small kana and iteration marks, whose line breaks differ between the
/// Japanese line rules.
const JAPANESE_LINE: &str = "ジョーンズさん、ぁぃ々〜ゝです…‥。日本語のテキストを分割します";
/// Chinese text with small kana and iteration marks, whose line breaks differ between the
/// Chinese line rules and the root line rules.
const CHINESE_LINE: &str = "「中文」測試，標點符號。ぁー々〜…";

const CASES: &[Case] = &[
    Case {
        data: "brkitr/char.brk",
        locale: "en",
        break_type: BreakType::kGraphemes,
        text: "é👨‍👩‍👧🇩🇪!",
        expected: &[0, 2, 20, 28, 29],
    },
    Case {
        data: "brkitr/word.brk",
        locale: "en",
        break_type: BreakType::kWords,
        text: "Hello, world! It's 3.14.",
        expected: &[0, 5, 6, 7, 12, 13, 14, 18, 19, 23, 24],
    },
    Case {
        data: "brkitr/thaidict.dict",
        locale: "th",
        break_type: BreakType::kWords,
        text: "ภาษาไทยง่ายนิดเดียว",
        expected: &[0, 12, 21, 33, 42, 57],
    },
    Case {
        data: "brkitr/laodict.dict",
        locale: "lo",
        break_type: BreakType::kWords,
        text: "ສະບາຍດີປະເທດລາວ",
        expected: &[0, 21, 36, 45],
    },
    Case {
        data: "brkitr/khmerdict.dict",
        locale: "km",
        break_type: BreakType::kWords,
        text: "ភាសាខ្មែរគឺជាភាសាផ្លូវការ",
        expected: &[0, 27, 39, 51, 66, 75],
    },
    Case {
        data: "brkitr/burmesedict.dict",
        locale: "my",
        break_type: BreakType::kWords,
        text: "မြန်မာနိုင်ငံသည်အာရှတိုက်တွင်ရှိသည်",
        expected: &[0, 39, 48, 54, 60, 75, 87, 96, 105],
    },
    Case {
        data: "brkitr/cjdict.dict, nfkc.nrm",
        locale: "zh",
        break_type: BreakType::kWords,
        text: "我们都是中国人",
        expected: &[0, 6, 12, 21],
    },
    Case {
        data: "brkitr/cjdict.dict, nfkc.nrm",
        locale: "ja",
        break_type: BreakType::kWords,
        text: "日本語のテキストを分割します",
        expected: &[0, 9, 12, 24, 27, 33, 36, 42],
    },
    Case {
        data: "brkitr/root.res: the default line rules",
        locale: "en",
        break_type: BreakType::kLines,
        text: LATIN_LINE,
        expected: &[0, 6, 11, 14, 20, 32, 37, 41, 43, 45, 48, 54],
    },
    Case {
        data: "brkitr/root.res: lb=strict",
        locale: "en-u-lb-strict",
        break_type: BreakType::kLines,
        text: LATIN_LINE,
        expected: &[0, 6, 11, 14, 20, 32, 37, 41, 43, 45, 54],
    },
    Case {
        data: "brkitr/root.res: lb=loose",
        locale: "en-u-lb-loose",
        break_type: BreakType::kLines,
        text: LATIN_LINE,
        expected: &[0, 6, 11, 14, 20, 32, 37, 41, 43, 45, 48, 51, 54],
    },
    Case {
        data: "brkitr/ja.res: lb=normal",
        locale: "ja-u-lb-normal",
        break_type: BreakType::kLines,
        text: JAPANESE_LINE,
        expected: &[
            0, 3, 6, 9, 12, 15, 18, 24, 27, 33, 39, 42, 54, 57, 60, 63, 66, 69, 72, 75, 78, 81, 84,
            87, 90, 93, 96,
        ],
    },
    Case {
        data: "brkitr/ja.res: lb=loose",
        locale: "ja-u-lb-loose",
        break_type: BreakType::kLines,
        text: JAPANESE_LINE,
        expected: &[
            0, 3, 6, 9, 12, 15, 18, 24, 27, 30, 33, 36, 39, 42, 48, 54, 57, 60, 63, 66, 69, 72, 75,
            78, 81, 84, 87, 90, 93, 96,
        ],
    },
    Case {
        data: "brkitr/ja.res: lw=phrase, brkitr/cjdict.dict",
        locale: "ja-u-lw-phrase",
        break_type: BreakType::kLines,
        text: JAPANESE_LINE,
        expected: &[0, 15, 27, 30, 39, 54, 63, 66, 78, 81, 87, 90, 96],
    },
    Case {
        data: "brkitr/ja.res: lb=normal, lw=phrase, brkitr/cjdict.dict",
        locale: "ja-u-lb-normal-lw-phrase",
        break_type: BreakType::kLines,
        text: JAPANESE_LINE,
        expected: &[0, 15, 24, 27, 30, 33, 39, 54, 63, 66, 78, 81, 87, 90, 96],
    },
    Case {
        data: "brkitr/ja.res: lb=loose, lw=phrase, brkitr/cjdict.dict",
        locale: "ja-u-lb-loose-lw-phrase",
        break_type: BreakType::kLines,
        text: JAPANESE_LINE,
        expected: &[
            0, 15, 24, 27, 30, 33, 36, 39, 48, 54, 63, 66, 78, 81, 87, 90, 96,
        ],
    },
    Case {
        data: "brkitr/ko.res: lw=phrase",
        locale: "ko-u-lw-phrase",
        break_type: BreakType::kLines,
        text: "한국어 텍스트를 나눕니다.",
        expected: &[0, 10, 23, 36],
    },
    Case {
        data: "brkitr/zh.res: the default line rules",
        locale: "zh",
        break_type: BreakType::kLines,
        text: CHINESE_LINE,
        expected: &[0, 6, 12, 15, 21, 24, 27, 30, 36, 39, 45, 51],
    },
    Case {
        data: "brkitr/zh_Hant.res, reached from zh_TW: the default line rules",
        locale: "zh-TW",
        break_type: BreakType::kLines,
        text: CHINESE_LINE,
        expected: &[0, 6, 12, 15, 21, 24, 27, 30, 36, 39, 45, 51],
    },
];
