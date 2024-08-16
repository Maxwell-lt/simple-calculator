{
  description = "simple-calculator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default.override { extensions = [ "llvm-tools-preview" ]; };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = rust;
          cargo = rust;
        };
        libPath =
          with pkgs;
          lib.makeLibraryPath [
            vulkan-loader
            libxkbcommon
            wayland
            xorg.libX11
          ];
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            rust
            rust-analyzer
            cargo-nextest
            cargo-llvm-cov
          ];

          shellHook = ''
                        export RUST_BACKTRACE=1
                        export LD_LIBRARY_PATH=${libPath}:$LD_LIBRARY_PATH
            	  '';
        };

        defaultPackage = rustPlatform.buildRustPackage rec {
          pname = "simple-calculator";
          version = "0.1.0";

          postFixup = ''
            patchelf $out/bin/${pname} \
              --add-rpath ${libPath}
          '';

          src = builtins.filterSource (path: type: type != "symlink" && baseNameOf path != "target") ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      }
    );
}
