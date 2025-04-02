{
  description = "A neat CLI program to manage file (tem)plates.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import inputs.systems;

      perSystem = {pkgs, ...}: {
        packages.default = pkgs.callPackage ./default.nix {};
        devShells.default = pkgs.callPackage ./shell.nix {};
      };
    };
}
