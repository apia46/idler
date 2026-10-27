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
    #pkgs-cross-mingw = import pkgs.path {
    #  crossSystem = {
    #    config = "x86_64-w64-mingw32";
    #  };
    #};

    # Our windows cross compiler plus
    # the required libraries and headers.
    #mingw_w64_cc = pkgs-cross-mingw.stdenv.cc;
    #mingw_w64_headers = pkgs-cross-mingw.windows.mingw_w64_headers;
    #mingw_w64 = pkgs-cross-mingw.windows.mingw_w64;
    #mingw_w64_pthreads_w_static = pkgs-cross-mingw.windows.pthreads.overrideAttrs (oldAttrs: {
    #  # TODO: Remove once / if changed successfully upstreamed.
    #  configureFlags = (oldAttrs.configureFlags or []) ++ [
    #    # Rustc require 'libpthread.a' when targeting 'x86_64-pc-windows-gnu'.
    #    # Enabling this makes it work out of the box instead of failing.
    #    "--enable-static"
    #  ];
    #});
  in {
    devShell = with pkgs; mkShell {
      LD_LIBRARY_PATH = libPath;
      buildInputs = [
        vulkan-headers
        vulkan-loader
        vulkan-tools
        pkg-config
        #fontconfig
        #mingw_w64
        #mingw_w64_pthreads_w_static
        #mingw_w64_cc
        #mingw_w64_headers
      ];
      #RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
      #  mingw_w64_cc
      #  mingw_w64
      #  mingw_w64_pthreads_w_static
      #  mingw_w64_headers
      #]);
    };
  }
	
