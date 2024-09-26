{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }: let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      pyEnv = pkgs.python3.withPackages (ps: with ps; [
        pydantic
      ]);
    in {
      packages.${system}.default = pkgs.writeShellScriptBin "s" ''
        ${pyEnv}/bin/python3 app.py
      '';
    };
}

