{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = with pkgs; [ 
            rustc 
            cargo
            rustfmt
            clippy
            rls
          ];
          
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
 
}
