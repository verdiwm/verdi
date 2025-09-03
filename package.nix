{
  clang,
  craneLib,
  lib,
  libinput,
  llvmPackages_20,
  pkg-config,
  systemdLibs,
  ...
}: let
  inherit (builtins) fromTOML readFile path;

  cargoToml = fromTOML (readFile ./Cargo.toml);
  inherit (cargoToml) package;

  pname = package.name;
in craneLib.buildPackage {
  inherit pname;
  inherit (package) version;

  src = path {
    path = craneLib.cleanCargoSource ./.;
    name = "${pname}-source";
  };

  buildInputs = [
    libinput
    systemdLibs
  ];

  nativeBuildInputs = [
    clang
    llvmPackages_20.bintools
    pkg-config
  ];

  buildPhase = ''
    runHook preBuild

    cargo xtask build

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    cargo xtask install --prefix . --destdir $out

    runHook postInstall
  '';

  meta = with lib; {
    inherit (package) homepage description;
    downloadPage = package.repository;
    changelog = "${package.repository}/blob/main/CHANGELOG.md";
    license = licenses.eupl12;
    maintainers = with maintainers; [ poz ];
    platforms = platforms.linux;
    mainProgram = pname;
  };
}
