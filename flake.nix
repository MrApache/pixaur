{
  description = "Rust dev environment with xkbcommon";

  inputs = {
    #nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        libPath = with pkgs; lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
            wayland.dev
        ];
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-analyzer
            rustc
            cargo
            clippy
            mold
            clang
            pkg-config
          ];

          LD_LIBRARY_PATH = libPath;
          PKG_CONFIG_PATH = "${pkgs.wayland.dev}/lib/pkgconfig:${libPath}:${pkgs.libxkbcommon.dev}/lib/pkgconfig";
          shellHook = ''
            echo "Welcome to the Rust dev shell"
          '';
        };
      });
}


