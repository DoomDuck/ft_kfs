{
  outputs = { nixpkgs, ... } :  let 
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      name = "kfs";

      buildInputs = with pkgs; [
        # Make iso with grub-mkrescue
        grub2

        # Make...
        gnumake

        lld

        # Rust tools
        rustup
      ];
    };
  };
}
