{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }: let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      pyPkgs = pkgs.python3Packages;
      epub_meta = let
        pname = "epub_meta";
        version = "0.0.7";
      in pkgs.python3Packages.buildPythonPackage {
        inherit pname version;
        src = pkgs.fetchFromGitHub {
          owner = "paulocheque";
          repo = "epub-meta";
          rev = "3cbbe936d97ec9b78918f6a4f4c8d4d3c89c29c6";
          sha256 = "sha256-Wor0sDLaNbPa+D3tcDfX208vRvEBDha4deCJOUkDU2I=";
        };
        doCheck = false;
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          # libs
          pyPkgs.fastapi
          pyPkgs.jinja2
          pyPkgs.python-multipart
          epub_meta
        ] ++ [
          # programs
          pkgs.calibre
          pkgs.loc
          pkgs.just
          pyPkgs.uvicorn
        ];
      };
    };
}

