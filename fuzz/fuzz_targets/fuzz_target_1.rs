#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let _ = fst::parse_stl(data);
});

// #![no_main]
// #[macro_use] extern crate libfuzzer_sys;
// extern crate url;
// 
// fuzz_target!(|data: &[u8]| {
//     if let Ok(s) = std::str::from_utf8(data) {
//         let _ = url::Url::parse(s);
//     }
// });
