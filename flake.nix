{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
        overlays.default = final: prev: {
          yaru = self.packages.yaru."${system}";
          don = self.packages.don."${system}";
          recently_use = self.packages.recently_use."${system}";
        };
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
