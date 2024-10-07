
{
  outputs = { nixpkgs, ... } :  let 
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      name = "kfs-linker";

      buildInputs = with pkgs; [ gcc binutils ];
    };
  };
}
