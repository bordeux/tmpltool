{
  description = "A fast and simple command-line template rendering tool using MiniJinja templates";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        rustPlatform = pkgs.rustPlatform;

        tmpltool = rustPlatform.buildRustPackage {
          pname = "tmpltool";
          version = "1.2.5";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with pkgs.lib; {
            description = "A fast and simple command-line template rendering tool using MiniJinja templates with environment variables";
            homepage = "https://github.com/bordeux/tmpltool";
            license = licenses.mit;
            maintainers = [ ];
            mainProgram = "tmpltool";
          };
        };
      in
      {
        packages = {
          default = tmpltool;
          tmpltool = tmpltool;
        };

        apps = {
          default = flake-utils.lib.mkApp {
            drv = tmpltool;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rust-analyzer
            clippy
            rustfmt
            cargo-make
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
