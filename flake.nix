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
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # version lockdown required to ensure build consistency; used with dioxus 0.7.1
        wasm-bindgen-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "wasm-bindgen-cli";
          version = "0.2.106";
          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "sha256-M6WuGl7EruNopHZbqBpucu4RWz44/MSdv6f0zkYw+44=";
          };
          cargoHash = "sha256-ElDatyOwdKwHg3bNH/1pcxKI7LXkhsotlDPQjiLHBwA=";
          doCheck = false;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl
          dioxus-cli
          rustToolchain
          wasm-bindgen-cli
          binaryen
        ];

        buildInputs = with pkgs; [
          openssl
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

          inherit nativeBuildInputs buildInputs;

          buildPhase = ''
            echo "Building Frontend..."
            dx build --release --features web --platform web

            echo "Building Backend..."
            cargo build --release --no-default-features --features server
          '';

          installPhase = ''
            mkdir -p $out/bin $out/share/website
            cp target/release/website $out/bin/server
            cp -r target/dx/website/release/web/public $out/share/website/public
          '';

          doCheck = false;
        };

        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
        };
      }
    );
}
