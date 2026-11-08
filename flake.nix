{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
  };

  outputs = inputs:
    let
      forAll = f: builtins.mapAttrs f inputs.nixpkgs.legacyPackages;
    in {
        devShells = forAll (system: pkgs: {
      default = pkgs.mkShell {
        packages = [ pkgs.cargo pkgs.rustc pkgs.rust-analyzer pkgs.rustfmt pkgs.clippy ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        shellHook = ''
          export CARGO_INSTALL_ROOT="$PWD/.cargo-install"
          export PATH="$CARGO_INSTALL_ROOT/bin:$PATH"
        '';
      };
    });
  };
}
