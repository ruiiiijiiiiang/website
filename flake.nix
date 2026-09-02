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
      devShell.env =
        pkgs:
        let
          inherit (pkgs) openssl libclang;
        in
        {
          OPENSSL_DIR = "${openssl.dev}";
          OPENSSL_LIB_DIR = "${openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
          LIBCLANG_PATH = "${libclang.lib}/lib";
        };
      package = lib.mkForce (
        { naersk, pkgs, ... }:
        naersk.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            clang
            binaryen
            dioxus-cli
            lld
            openssl
            pkg-config
            rustc
            wasm-bindgen-cli_0_2_114
          ];
          buildInputs = [ pkgs.openssl ];

          overrideMain = old: {
            preBuild = ''
              export HOME=$(mktemp -d)
            '';

            buildPhase = ''
              export CARGO_TARGET_DIR=$PWD/target
              cargo clean
              dx bundle --release --web --ssg --fullstack true --force-sequential true
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
