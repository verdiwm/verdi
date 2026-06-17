{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      eachSystem = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      devShells = eachSystem (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo-nextest
              cargo-expand
              cargo-bloat
              cargo-edit
              pkg-config
              vulkan-validation-layers
              udev
              clang
              libinput
              vulkan-loader
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                ];
              })
            ];

            LD_LIBRARY_PATH = with pkgs; "./target:${vulkan-loader}/lib:${vulkan-validation-layers}/lib";
            VULKAN_SDK = with pkgs; "${vulkan-headers}";
            VK_LAYER_PATH = with pkgs; "${vulkan-validation-layers}/share/vulkan/explicit_layer.d";
          };
        }
      );
    };
}
