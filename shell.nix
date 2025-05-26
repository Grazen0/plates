{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];

  env = {
    RUST_BACKTRACE = "1";
  };
}
