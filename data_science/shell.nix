let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in
pkgs.mkShellNoCC {
  packages = with pkgs.python312Packages; [
    ipykernel ipython
    numpy pandas
    matplotlib seaborn
    pydantic
  ];
}
