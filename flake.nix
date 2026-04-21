{
  inputs = {
    flakelight-rust.url = "github:accelbread/flakelight-rust";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { flakelight-rust, rust-overlay, ... }:
    let
      inherit (flakelight-rust.inputs.flakelight.inputs.nixpkgs) lib;
    in
    flakelight-rust ./. {
      fileset = ./.;
      withOverlays = [ (import rust-overlay) ];
      package = lib.mkForce (
        { naersk, pkgs, ... }:
        let
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
        in
        naersk.buildPackage {
          src = ./.;
          nativeBuildInputs = [
            pkgs.dioxus-cli
            rustToolchain
            pkgs.pkg-config
            pkgs.openssl
            pkgs.lld
            pkgs.binaryen
            pkgs.clang
          ];
          buildInputs = [ pkgs.openssl ];

          overrideMain = old: {
            preBuild = ''
              export HOME=$(mktemp -d)
            '';

            buildPhase = ''
              dx bundle --release --web --ssg
              cargo build --release --bin sitemap
            '';

            installPhase = ''
              mkdir -p $out/app
              cp target/dx/website/release/web/server $out/app/server
              cp -r target/dx/website/release/web/public $out/app/public
              cp target/release/sitemap $out/app/sitemap
            '';
          };
        }
      );
    };
}
