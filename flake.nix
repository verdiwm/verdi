{
  description = "The elegant Wayland compositor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, systems, crane, ... }: let
    name = "verdi";
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = forEachSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        package = pkgs.callPackage ./package.nix { inherit craneLib; };
      in {
        ${name} = package;
        default = package;
      }
    );
    devShells = forEachSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        shell = craneLib.devShell {
          inherit name;

          packages = with pkgs; [
            rust-analyzer
            clippy
            gdb
          ];

          inputsFrom = [ self.packages.${system}.${name} ];
        };
      in {
        ${name} = shell;
        default = shell;
      }
    );
  };
}
