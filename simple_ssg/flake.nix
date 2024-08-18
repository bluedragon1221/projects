{
  inputs = {
    # nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { nixpkgs, naersk, ... }: let
      system = "x86_64-linux";
    in {
      packages.${system}.default = naersk.buildPackage {
        src = ./.;
      };
    };
}

