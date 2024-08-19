{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }: let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      pyEnv = pkgs.python3.withPackages (ps: with ps; [
        ipython
        ipykernel
        jupyter
        pandas
        numpy
        matplotlib
      ]);

      customCode = pkgs.vscode-with-extensions.override {
        vscodeExtensions = with pkgs.vscode-extensions; [
          ms-python.python
          ms-toolsai.jupyter
        ];
      };
    in {
      packages.${system} = {
        jupyter = pkgs.writeShellScriptBin "j" "${pyEnv}/bin/jupyter notebook";
        code = pkgs.writeShellScriptBin "code" "${customCode}/bin/code .";
      };
    };
}

