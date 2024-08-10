{ lib
, rustPlatform
, pkg-config
, udev
, libinput
}:
rustPlatform.buildRustPackage rec {
  pname = "verdi";
  version = "0.0.1";
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    udev     # required by `devil`
    libinput # required by `colpetto`
  ];
  cargoLock.lockFile = ./Cargo.lock;
  cargoLock.outputHashes = {
    "diretto-0.0.3" = "sha256-lcrJjJ5JRtr76gDHQPlHz/PRAXnw4uXVSM4cEyty6W8=";
    "linux-raw-sys-0.6.4" = "sha256-eg9LCBcrKHXKzpxLV9ClcI5b/7e/pKA3f2dB59lwzx4=";
    "naga-22.0.0" = "sha256-yuBW6MbhvGK7SayPj3kpFsgd2n/Geaxfse2b15AT5Y8=";
    "raw-window-handle-0.6.2" = "sha256-gAlhrvx/yILQSqaBCAqVB+eopiZqLDBQTGGyvM799KM=";
  };
  src = lib.cleanSource ./.;
}

