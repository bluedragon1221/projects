{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }: let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      pyEnv = pkgs.python3.withPackages (ps: with ps; [
        uvicorn
        starlette
        jinja2
        pandas
        matplotlib
      ]);
    in {
      packages.${system}.default = pkgs.writeShellScriptBin "suad" ''
        ${pyEnv}/bin/uvicorn app:app
      '';
    };
}

