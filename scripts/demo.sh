#!/usr/bin/env bash
set -euo pipefail

export PATH="/opt/homebrew/bin:/opt/homebrew/sbin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:$PATH"

UPI="./target/release/upi"

run() {
  echo "\$ $*"
  sleep 0.6
  "$@"
  local ec=$?
  sleep 1.5
  echo
  return $ec
}

clear
sleep 1

cat << "ART"

  ██╗   ██╗██████╗ ██╗
  ██║   ██║██╔══██╗██║
  ██║   ██║██████╔╝██║
  ██║   ██║██╔═══╝ ██║
  ╚██████╔╝██║     ██║
   ╚═════╝ ╚═╝     ╚═╝

  Universal Package Installer
  One CLI: every OS, every package manager.

ART

sleep 2

run $UPI --help
sleep 1

echo "--------------------------------------------"
echo "  SEARCH: find packages on your current OS"
echo "--------------------------------------------"
sleep 1
echo

run $UPI search python
sleep 1

echo "--------------------------------------------"
echo "  INSTALL (dry-run): see the command"
echo "--------------------------------------------"
sleep 1
echo

echo "--- macOS --- "
sleep 0.5
run $UPI --dry-run install python
sleep 1.5

echo "--- Debian/Ubuntu --- "
sleep 0.5
run $UPI --os debian --dry-run install python
sleep 1.5

echo "--- Fedora --- "
sleep 0.5
run $UPI --os fedora --dry-run install python
sleep 1.5

echo "--- Arch Linux --- "
sleep 0.5
run $UPI --os arch --dry-run install python
sleep 1.5

echo "--- Windows --- "
sleep 0.5
run $UPI --os windows --dry-run install python
sleep 2

echo "--------------------------------------------"
echo "  OFFLINE: zero-network mode using"
echo "  the built-in seed database"
echo "--------------------------------------------"
sleep 1
echo

run $UPI --offline --dry-run install python
sleep 2

echo "--------------------------------------------"
echo "  Different package: neovim"
echo "--------------------------------------------"
sleep 1
echo

run $UPI --os debian --dry-run install neovim
sleep 1.5
run $UPI --os arch --dry-run install neovim
sleep 2

sleep 1
echo

cat << "FOOTER"

  ╔══════════════════════════════════════════════╗
  ║  10,089 package-manager mappings             ║
  ║  531 cross-platform packages                 ║
  ║  5 OS families / 8 package managers          ║
  ║  Zero-setup built-in seed database           ║
  ╚══════════════════════════════════════════════╝
  Get it now:
    https://github.com/Nisoku/UPI

FOOTER
sleep 3
