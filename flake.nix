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
        rustPlatform = pkgs.makeRustPlatform.override { stdenv = pkgs.clangStdenv; }  {
          inherit (rustToolchain) cargo rustc;
        };
        llvm = pkgs.llvmPackages;
        # See https://discourse.nixos.org/t/develop-shell-environment-setup-for-macos/11399/6
        coreAudio = if pkgs.stdenv.isDarwin then
          pkgs.symlinkJoin {
            name = "sdk";
            paths = with pkgs.darwin.apple_sdk.frameworks; [
              AudioToolbox
              AudioUnit
              CoreAudio
              CoreFoundation
              CoreMIDI
              OpenAL
            ];
            postBuild = ''
              mkdir $out/System
              mv $out/Library $out/System
            '';
          }
        else
          "";
      in
      # See https://nixos.wiki/wiki/Flakes#Output_schema
      {
        # `nix develop` pulls all of this in to become your shell.
        devShells.default = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          nativeBuildInputs = [
            rustToolchain
            rustPlatform.bindgenHook

            # Common cargo tools we often use
            pkgs.cargo-deny
            pkgs.cargo-expand
            pkgs.cargo-binutils
          ];
          buildInputs = [
            # This is missing on mac m1 nix, for some reason.
            # see https://stackoverflow.com/a/69732679
            pkgs.libiconv
            coreAudio
          ];
          # NOTE: coreaudio-sys still doesn't build despite my best efforts! If
          # you know how to fix this please reach out.
          shellHook = ''
            export LIBCLANG_PATH="${llvm.libclang.lib}/lib"
            export COREAUDIO_SDK_PATH="${coreAudio}"
          '';
        };
        # This only formats the nix files.
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
