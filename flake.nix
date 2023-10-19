{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rust_toolchain = pkgs.rust-bin.stable.latest;
        naersk' = pkgs.callPackage naersk {
          rustc = rust_toolchain.minimal;
          cargo = rust_toolchain.minimal;
        };
      in
      rec {
        packages = {
          recently_use = naersk'.buildPackage {
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.gtk3 ];
            src = ./recently_use;
          };
          yaru = naersk'.buildPackage {
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.dbus ];
            src = ./yaru;
          };
          don = naersk'.buildPackage {
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ ];
            src = ./don;
          };
        };
        overlays.default = final: prev: {
          yaru = packages.yaru;
          don = packages.don;
          recently_use = packages.recently_use;
        };

        # TODO: pull in dependencies from the packages
        devShell = pkgs.mkShell {
          nativeBuildInputs = [
            (rust_toolchain.default.override {
              extensions = [ "rust-src" "rustfmt" "rls" "clippy" ];
            })
          ];
        };
      }
    );
}
