with import <nixpkgs> {};
mkShell {
  nativeBuildInputs = [
    cargo
    rustc
    rustfmt
    pkg-config
  ];
}
