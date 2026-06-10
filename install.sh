#!/bin/sh
set -eu

REPO="Nisoku/UPI"

# Platform detection
detect_platform() {
  local os arch
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"

  case "$os" in
    linux)   os="linux" ;;
    darwin)  os="macos" ;;
    mingw*|msys*|cygwin*) os="windows" ;;
    *)       echo "unsupported OS: $os" >&2; exit 1 ;;
  esac

  case "$arch" in
    x86_64|amd64) arch="amd64" ;;
    aarch64|arm64) arch="arm64" ;;
    *) echo "unsupported arch: $arch" >&2; exit 1 ;;
  esac

  if [ "$os" = "linux" ] && [ "$arch" = "arm64" ]; then
    echo "unsupported arch: $arch on $os" >&2; exit 1
  fi

  echo "${os}-${arch}"
}

# Get latest release tag
fetch_latest_tag() {
  if command -v curl >/dev/null 2>&1; then
    curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" \
      | awk -F'"' '/"tag_name"/ {print $4}'
  elif command -v wget >/dev/null 2>&1; then
    wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" \
      | awk -F'"' '/"tag_name"/ {print $4}'
  else
    echo "need curl or wget" >&2; exit 1
  fi
}

# Determine install dir (fall back if /usr/local/bin not writable)
pick_install_dir() {
  if [ -w "/usr/local/bin" ]; then
    echo "/usr/local/bin"
  elif [ -w "${HOME}/.local/bin" ]; then
    echo "${HOME}/.local/bin"
  else
    mkdir -p "${HOME}/.local/bin"
    echo "${HOME}/.local/bin"
  fi
}

# Main
main() {
  local platform tag url tmpdir binary_name dest install_dir bin_suffix

  platform="$(detect_platform)"
  echo "detected platform: ${platform}"

  tag="${UPI_VERSION:-$(fetch_latest_tag)}"
  if [ -z "$tag" ]; then
    echo "could not determine latest version" >&2
    exit 1
  fi
  echo "latest release: ${tag}"

  install_dir="${UPI_INSTALL_DIR:-$(pick_install_dir)}"
  bin_suffix=""
  case "$platform" in
    windows-*) bin_suffix=".exe" ;;
  esac
  dest="${install_dir}/upi${bin_suffix}"

  url="https://github.com/${REPO}/releases/download/${tag}/upi-${tag}.tar.gz"

  echo "downloading ${url}"
  tmpdir="$(mktemp -d)"
  if command -v curl >/dev/null 2>&1; then
    curl -sSfL "$url" -o "${tmpdir}/upi.tar.gz"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "${tmpdir}/upi.tar.gz" "$url"
  else
    echo "need curl or wget" >&2; exit 1
  fi

  tar -xzf "${tmpdir}/upi.tar.gz" -C "$tmpdir"

  binary_name="upi-${platform}${bin_suffix}"
  if [ ! -f "${tmpdir}/${binary_name}" ]; then
    echo "binary not found in archive: ${binary_name}" >&2
    ls "${tmpdir}" >&2
    exit 1
  fi

  mkdir -p "$install_dir"
  cp "${tmpdir}/${binary_name}" "$dest"
  chmod +x "$dest"
  rm -rf "$tmpdir"

  echo "installed upi to ${dest}"

  if ! echo "$PATH" | grep -q "$install_dir"; then
    echo "warning: ${install_dir} is not in PATH" >&2
  fi
}

main "$@"
