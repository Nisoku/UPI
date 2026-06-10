#![no_main]

use libfuzzer_sys::fuzz_target;

use upi_core::parse_search_output;

fuzz_target!(|data: &[u8]| {
    let s = if let Ok(s) = std::str::from_utf8(data) {
        s
    } else {
        return;
    };

    // split on null byte; if no null, treat whole input as output
    let (output, query) = if let Some(idx) = s.find('\0') {
        (&s[..idx], &s[idx + 1..])
    } else {
        (s, "")
    };

    let _ = parse_search_output(output, query);
});
