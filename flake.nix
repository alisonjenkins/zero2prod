{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { ... } @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import inputs.rust-overlay) ];

        pkgs = import inputs.nixpkgs {
          inherit system overlays;
        };

        myRust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [
            "x86_64-unknown-linux-musl"
            "aarch64-unknown-linux-musl"
          ];
        };
      in
      {
        devShells.default = with pkgs;
          mkShell {
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            buildInputs = [
              dive
              just
              mold
              myRust
              openssl
              pkg-config
              postgresql
              postgresql.out
              rocmPackages.llvm.clang
              sqlx-cli
            ];
          };
      }
    );
}
