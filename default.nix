{ rustPlatform, pkgs }: rustPlatform.buildRustPackage rec {
  pname = "jam-seqs";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "0a86hmqdfn642vacriviyzhf21xfvlr6l6iwv8x8rqp62k2vd40f";
  verifyCargoDeps = true;
  buildType = "release";
  nativeBuildInputs = with pkgs; [ pkgconfig gcc glibc fuse fuse-common openal libsndfile ];
  RUST_BACKTRACE = 1;
}
