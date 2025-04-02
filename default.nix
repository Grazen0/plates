{
  lib,
  rustPlatform,
  installShellFiles,
  ...
}: let
  manifest = (lib.importTOML ./Cargo.toml).package;
  inherit (manifest) name;
in
  rustPlatform.buildRustPackage rec {
    pname = name;
    version = manifest.version;
    src = lib.cleanSource ./.;
    cargoLock.lockFile = "${src}/Cargo.lock";

    env = {
      RUST_BACKTRACE = "1";
      PLATES_GEN_MAN_PAGES = true;
      PLATES_GEN_COMPLETIONS = true;
    };

    nativeBuildInputs = [
      installShellFiles
    ];

    postInstall = ''
      install -Dm644 "out/man/${name}.1" -t "$out/share/man/man1"

      installShellCompletion --cmd "${name}" \
        --bash "out/completions/${name}.bash" \
        --fish "out/completions/${name}.fish" \
        --zsh  "out/completions/_${name}"
    '';

    meta = with lib; {
      description = "A neat CLI program to manage file (tem)plates.";
      homepage = manifest.homepage;
      license = licenses.mit;
      mainProgram = "plates";
    };
  }
