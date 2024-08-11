{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    cl-nix-lite.url = "github:hraban/cl-nix-lite";
  };

  outputs = { nixpkgs, cl-nix-lite, ... }: let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system}.extend cl-nix-lite.overlays.default;
    in {
      packages.${system}.default = with pkgs.lispPackagesLite; lispScript {
        name = "htmacro";
        dependencies = [ str arrow-macros ];
        src = "./main.lisp";
      };
    };
}

