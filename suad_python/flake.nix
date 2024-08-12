{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, cl-nix-lite, ... }: let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = derivation {
        name = "Seven up and down tracker";
        buildInputs = with pkgs.python312Packages; [
          starlette jinja2 pandas matplotlib
        ];
        builder = pkgs.python312Packages.uvicorn;
        args = "app:app";
        src = ./.;
        system = system;
      };
    };
}

