{
  inputs = {
    flakelight-rust.url = "github:accelbread/flakelight-rust";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { flakelight-rust, rust-overlay, ... }:
    let
      lib = flakelight-rust.inputs.flakelight.inputs.nixpkgs.lib;
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
              mkdir -p $out/bin
              # In some Dioxus versions, 'server' is the actual binary
              if [ -f target/dx/website/release/web/server ]; then
                cp target/dx/website/release/web/server $out/bin/website
              else
                cp target/dx/website/release/web/server/website $out/bin/website
              fi
              cp -r target/dx/website/release/web/public $out/bin/public
              cp target/release/sitemap $out/bin/sitemap
            '';
          };
        }
      );
    };
}
