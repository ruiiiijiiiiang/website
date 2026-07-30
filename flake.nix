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
        let
          rustToolchain = pkgs.rust-bin.stable."1.96.1".default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
        in
        naersk.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            clang
            binaryen
            dioxus-cli
            lld
            openssl
            pkg-config
            rustToolchain
            wasm-bindgen-cli_0_2_114
          ];
          buildInputs = [ pkgs.openssl ];

          overrideMain = old: {
            preBuild = ''
              export HOME=$(mktemp -d)
            '';

            buildPhase = ''
              export CARGO_TARGET_DIR=$(mktemp -d)
              dx bundle --release --web --ssg --fullstack true @client --features web @server --features server
              cargo build --release --bin sitemap
            '';

            installPhase = ''
              mkdir -p $out/app
              cp $CARGO_TARGET_DIR/dx/website/release/web/server $out/app/server
              cp -r $CARGO_TARGET_DIR/dx/website/release/web/public $out/app/public
              cp $CARGO_TARGET_DIR/release/sitemap $out/app/sitemap
            '';
          };
        }
      );
    };
}
