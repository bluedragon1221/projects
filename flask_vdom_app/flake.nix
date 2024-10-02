{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }: let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      pyPkgs = pkgs.python3Packages;
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          # libs
          pyPkgs.fastapi
          pyPkgs.jinja2
          pyPkgs.vdom
          pyPkgs.python-multipart
        ] ++ [
          # programs
          pyPkgs.uvicorn
        ];
      };
    };
}

