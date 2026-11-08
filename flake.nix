{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
  };

  outputs = inputs:
    let
      # map some function f against legacyPackages
      forAll = f: builtins.mapAttrs f inputs.nixpkgs.legacyPackages;
    in {
      # devShells have structure devShells.<system>.<name>
      # use `nix flake show` to see everything that it could build for
      # parens are to demarcate a lambda that then gets passed to forAll as one arg
      devShells = forAll (system: pkgs:
        let
          cross = pkgs.pkgsCross.aarch64-multiplatform-musl;
          target = "aarch64-unknown-linux-musl";
        in {
          # devShells.<system>.default is the default devShell.
          # mkShell is a function in nixpkgs: https://nixos.org/manual/nixpkgs/stable/#sec-pkgs-mkShell
          default = pkgs.mkShell {
            # default.packages is the packages that are built or taken from store and then put on PATH in shell
            packages = [
              cross.buildPackages.rustc
              cross.buildPackages.cargo
              cross.buildPackages.clippy
              cross.buildPackages.rustfmt
              pkgs.rust-analyzer
              cross.stdenv.cc
            ];
            # where is rust, for rust-analyzer
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
            # what are we building rust for
            CARGO_BUILD_TARGET = target;
            # linker for target
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER =
              "${cross.stdenv.cc}/bin/aarch64-unknown-linux-musl-gcc";
            # we want to scp the whole binary to the pi (no rust toolchain on pi is ideal), so use static linking
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS = "-C target-feature=+crt-static -C relocation-model=static";
            # for stuff that will change depending on where it is run, put it in the shellHook
            shellHook = ''
              echo -e "\e[32mWelcome to the $PWD Rust devShell!\e[0m"
              export CARGO_INSTALL_ROOT="$PWD/.cargo-install"
              export PATH="$CARGO_INSTALL_ROOT/bin:$PATH"
            '';
          };
        });
    };
}