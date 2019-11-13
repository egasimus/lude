{ rustPlatform, pkgs }: rustPlatform.buildRustPackage rec {
  pname = "sequence-rs";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "1nh8kaanhan87z2nf38mnqh51yd7ff0316f2yabg0bpihmp11nyl";
  verifyCargoDeps = true;
  buildType = "release";
  nativeBuildInputs = with pkgs; [ pkgconfig gcc glibc fuse fuse-common openal libsndfile ];
  RUST_BACKTRACE = 1;
}
