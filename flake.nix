{
  description = "skilltree flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # eachDefaultSystem and other utility functions
    utils.url = "github:numtide/flake-utils";
    # Replacement for rustup
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, fenix, }:
    # This helper function abstracts over the host platform.
    # See https://github.com/numtide/flake-utils#eachdefaultsystem--system---attrs
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        # Brings in the rust toolchain from the standard file
        # that rustup/cargo uses.
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-Q9UgzzvxLi4x9aWUJTn+/5EXekC98ODRU1TwhUs9RnY=";
        };
        rustPlatform = pkgs.makeRustPlatform {
          inherit (rustToolchain) cargo rustc;
        };
      in
      # See https://nixos.wiki/wiki/Flakes#Output_schema
      {
        # `nix develop` pulls all of this in to become your shell.
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rustToolchain

            # Common cargo tools we often use
            pkgs.cargo-deny
            pkgs.cargo-expand
            pkgs.cargo-binutils
          ];

          # see https://github.com/NixOS/nixpkgs/blob/95b81c96f863ca8911dffcda45d1937efcd66a4b/pkgs/games/jumpy/default.nix#L60C5-L60C38
          buildInputs = [
            pkgs.zstd
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs; [
            alsa-lib
            libxkbcommon
            udev
            vulkan-loader
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
          xorg.libXrandr
          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Cocoa
            rustPlatform.bindgenHook
            # # This is missing on mac m1 nix, for some reason.
            # # see https://stackoverflow.com/a/69732679
            pkgs.libiconv
          ];
        };
        # This only formats the nix files.
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
