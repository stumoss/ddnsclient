{ pkgs ? (import <nixpkgs> {}) }:

with pkgs;

rustPlatform.buildRustPackage rec {
  name = "ddnsclient-1.0.2";
  src = ./.;
  buildInputs = [ gcc openssl pkgconfig ];
  cargoSha256 = "10x28289k0mclzj42vf51l5qxildpyqi5l5s6bj532abw83cf0am";
}
