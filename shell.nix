{ pkgs ? import <nixpkgs>{} }:
  let
    libPath = with pkgs; lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
      vulkan-headers
      vulkan-loader
      vulkan-tools
    ];
  in {
    devShell = with pkgs; mkShell {
      LD_LIBRARY_PATH = libPath;
        buildInputs = [
            vulkan-headers
            vulkan-loader
            vulkan-tools
            pkg-config
            fontconfig
        ];
    };
  }
	
