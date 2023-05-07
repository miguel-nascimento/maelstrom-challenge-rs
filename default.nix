# Dependencies to run Maelstrom binary 
{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = [
      pkgs.openjdk
      pkgs.graphviz 
      pkgs.gnuplot
    ];
}