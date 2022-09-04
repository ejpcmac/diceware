{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    # Build toolchain.
    rustup
  ];
}
