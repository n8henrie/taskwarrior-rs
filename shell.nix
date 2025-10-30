with import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-24.05.tar.gz") { };
mkShell {
  packages = [
    cmake
    gnutls
    iconv
    libtasn1
    pkg-config
  ];
}
