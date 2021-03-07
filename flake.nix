{
  description = "nix development shell for rust-skia";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages."${system}";
      in {
        devShell = pkgs.mkShell {
          SKIA_NINJA_COMMAND = "${pkgs.ninja}/bin/ninja";
          SKIA_GN_COMMAND = "${pkgs.gn}/bin/gn";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib/libclang.so";

          # necessary to override nix's defaults which cannot be overriden as others are
          shellHook = ''
            export CC="${pkgs.clang}/bin/clang"
            export CXX="${pkgs.clang}/bin/clang++"
            rustup override set stable
            '';

          nativeBuildInputs = with pkgs; [ rustup python fontconfig clang ];
        };
      });
}
