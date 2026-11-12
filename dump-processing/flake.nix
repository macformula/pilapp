{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
  };

  outputs =
    inputs:
    let
      forAll = f: builtins.mapAttrs f inputs.nixpkgs.legacyPackages;
    in
    {
      devShells = forAll (
        system: pkgs:
        let
          py = pkgs.python314.withPackages (
            ps: with ps; [
              numpy
              matplotlib
            ]
          );
        in
        {
          default = pkgs.mkShell {
            packages = [
              py
              pkgs.xxd #view the binary all matrix-style
            ];
          };
        }
      );
    };
}
