with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "sequence-rs";
  nativeBuildInputs = with pkgs; [
    rustc cargo pkgconfig
  ];
  buildInputs = with pkgs; [
    openal libsndfile libjack2
  ];
  RUST_BACKTRACE = 1;
}
