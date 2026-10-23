{ pkgs ? import <nixpkgs>{} }:
  let
    libPath = with pkgs; lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
  in {
    devShell = with pkgs; mkShell {
      LD_LIBRARY_PATH = libPath;
    };
  }
	
