#![no_main]

use libfuzzer_sys::fuzz_target;

use upi_net::RepologyResponse;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _: std::result::Result<RepologyResponse, _> = serde_json::from_str(s);
    }
});
