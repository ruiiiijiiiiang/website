{
  description = "My Personal Website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        nativeBuildInputs = with pkgs; [
          binaryen
          pkg-config
          openssl
          dioxus-cli
          wasm-bindgen-cli_0_2_104
          rustToolchain
        ];

      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "website";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs;

          buildPhase = ''
            echo "Building Frontend..."
            dx build --release --features web --platform web

            echo "Building Backend..."
            cargo build --release --features server
          '';

          installPhase = ''
            mkdir -p $out/bin $out/share/website
            cp target/release/website $out/bin/server
            cp -r target/dx/website/release/web/public $out/share/website/public
          '';

          doCheck = false;
        };

        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs;
        };
      }
    );
}
