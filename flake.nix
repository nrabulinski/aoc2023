{
  inputs = {
    aoc-utils.url = "github:nrabulinski/aoc-utils";
    nixpkgs.follows = "aoc-utils/nixpkgs";
    fenix.follows = "aoc-utils/fenix";
  };

  outputs = {
    nixpkgs,
    fenix,
    aoc-utils,
    ...
  }: let
    forEachSystem = f:
      with nixpkgs.lib; let
        systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
      in
        genAttrs systems (system: f nixpkgs.legacyPackages.${system});
  in {
    devShells = forEachSystem (pkgs: {
      default = pkgs.mkShell {
        packages =
          [
            fenix.packages.${pkgs.system}.complete.toolchain # rust toolchain
            aoc-utils.packages.${pkgs.system}.aoc-cli
            pkgs.pkg-config
            pkgs.z3
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      };
    });

    formatter = forEachSystem (pkgs: pkgs.alejandra);
  };
}
