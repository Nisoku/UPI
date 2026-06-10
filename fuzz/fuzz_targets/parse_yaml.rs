#![no_main]

use libfuzzer_sys::fuzz_target;

use upi_core::PlatformConfig;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = noyalib::from_str::<PlatformConfig>(s);
    }
});
