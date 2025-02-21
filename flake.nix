{
  outputs = { nixpkgs, ... } :  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    pkgsCross = pkgs.pkgsCross.i686-embedded;
  in {
    devShells.${system}.default = pkgsCross.mkShell {
      name = "kfs";
      nativeBuildInputs = with pkgs; [
        # i686-elf-ld
        buildPackages.bintools

        # Make iso with grub-mkrescue
        grub2
        xorriso

        # Assembly
        nasm

        # Make...
        gnumake

        # Rust tools
        rustup

        # cc
        gcc
      ];
    };
  };
}
