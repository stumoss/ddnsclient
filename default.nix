with import <nixpkgs> {};

rustPlatform.buildRustPackage rec {
  name = "ddnsclient-0.0.1";
  src = ./.;
  buildInputs = [ gcc openssl pkgconfig ];
  depsSha256 = "0s50ij2qm08q08y44nvk8dpi3rzxvfs4q50531ypsd6hg5l1na9j";
}
