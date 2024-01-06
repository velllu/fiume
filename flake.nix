{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./api/toolchain.toml;
  in {
    devShells.${system}.default = pkgs.mkShell {
      # This is not for production
      DATABASE_URL="postgresql://username:password@localhost:8001/fiume";

      packages = [
        # Rust API
        toolchain
        pkgs.openssl.dev pkgs.pkg-config

        # Others
        pkgs.sqlx-cli
        pkgs.postgresql

        # Vue stuff
        pkgs.nodejs_20
      ];
    };
  };
}
