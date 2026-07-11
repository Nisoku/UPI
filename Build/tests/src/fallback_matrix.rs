use upi_core::parse_search_output;

#[test]
fn brew_exact_match() {
    let output = "\
==> Formulae
ffmpeg
ffmpegthumbnailer

==> Casks
ffmpegx
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn brew_substring_match() {
    let output = "\
==> Formulae
ffmpegthumbnailer
ffmpeg
";
    let result = parse_search_output(output, "thumbnailer");
    assert_eq!(result, Some("ffmpegthumbnailer".into()));
}

#[test]
fn apt_matches_first_token_before_slash() {
    let output = "\
Sorting... Done
Full Text Search... Done
ffmpeg/focal-updates,now 7:4.2.7-0ubuntu0.1 amd64 [installed]
  Multimedia player...
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn apt_skips_noise_lines() {
    let output = "\
Sorting... Done
Full Text Search... Done
libpng-dev/focal 1.6.37-2 amd64
";
    let result = parse_search_output(output, "libpng");
    assert_eq!(result, Some("libpng-dev".into()));
}

#[test]
fn dnf_strips_arch_suffix() {
    let output = "\
Last metadata expiration check: 0:00:01 ago
ffmpeg.x86_64 : The ffmpeg...
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn dnf_matches_name_before_dot() {
    let output = "\
ffmpeg.x86_64 : ...
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn pacman_strips_repo_prefix() {
    let output = "\
extra/ffmpeg 7:7.1-1 [installed]
    ...
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn pacman_multilib() {
    let output = "\
extra/ffmpeg 7:7.1-1
multilib/lib32-ffmpeg 7:7.0-1
";
    let result = parse_search_output(output, "lib32-ffmpeg");
    assert_eq!(result, Some("lib32-ffmpeg".into()));
}

#[test]
fn winget_extracts_name_column() {
    let output = "\
Name                  Id                   Version
--------------------------------------------------
FFmpeg (prerelease)   Gyan.FFmpeg          7.1
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("FFmpeg (prerelease)".into()));
}

#[test]
fn choco_first_token() {
    let output = "\
chocolatey 0.10.15
ffmpeg 4.4.1 [Approved]
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn short_query_does_not_pick_unrelated_prefix_match() {
    let output = "\
chocolatey 0.10.15
rgsupervision 1.2.3 [Approved]
";
    let result = parse_search_output(output, "rg");
    assert_eq!(result, None);
}

#[test]
fn node_prefers_nodejs_over_iisnode() {
    let output = "\
Chocolatey v2.7.3
iisnode 0.2.26 [Approved]
node-webkit 0.6.2 [Approved]
nodejs 26.5.0 [Approved]
";
    let result = parse_search_output(output, "node");
    assert_eq!(result, Some("nodejs".into()));
}

#[test]
fn scoop_after_arrow() {
    let output = "\
'ffmpeg' suggests 'ffmpeg-shared'
Results from local buckets...
ffmpeg (main) --> ffmpeg [latest]
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn macports_strips_version_prefix() {
    let output = "\
ffmpeg @7.1 (multimedia, video)
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn empty_output_returns_none() {
    let result = parse_search_output("", "ffmpeg");
    assert_eq!(result, None);
}

#[test]
fn no_match_returns_none() {
    let output = "\
Sorting... Done
Full Text Search... Done
someotherpkg/focal 1.0 amd64
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, None);
}

#[test]
fn generic_fallback_works() {
    let output = "\
ffmpeg is the best
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}

#[test]
fn generic_with_repo_prefix() {
    let output = "\
main/ffmpeg 1.0
";
    let result = parse_search_output(output, "ffmpeg");
    assert_eq!(result, Some("ffmpeg".into()));
}
