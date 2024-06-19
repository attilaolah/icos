{
  description = "Icosahedral Goldberg polyhedra.";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

  outputs = {nixpkgs, ...}: let
    pkgs = import nixpkgs {inherit system;};
    system = "x86_64-linux";
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        clangStdenv
      ];
    };
  };
}
