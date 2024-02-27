{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    neovim-flake.url = "github:notashelf/neovim-flake";
  };

  outputs = {
    self,
    nixpkgs,
    neovim-flake,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    customNeovim = neovim-flake.lib.neovimConfiguration {
      inherit pkgs;
      modules = [(pkgs.callPackage ./nix/config.nix {})];
    };
  in {
    packages.${system} = {
    };
    devShell.${system} = let
      inherit
        (pkgs)
        udev
        cmake
        alsaLib
        clang
        pkg-config
        llvmPackages
        rustup
        libxkbcommon
        vulkan-tools
        vulkan-loader
        vulkan-headers
        wayland
        vulkan-validation-layers
        ctags
        ;
      inherit (pkgs.lib) readFile makeLibraryPath;
    in
      pkgs.mkShell {
        packages = [cmake customNeovim.neovim ctags];
        buildInputs = [
          clang
          pkg-config
          llvmPackages.bintools
          rustup
          alsaLib
          (pkgs.glfw.override {waylandSupport = true;})

          wayland
          udev
          vulkan-validation-layers
          vulkan-headers
          vulkan-tools
          vulkan-loader
        ];

        RUSTC_VERSION = readFile ./rust-toolchain;
        # https://github.com/rust-lang/rust-bindgen#environment-variables
        LIBCLANG_PATH = makeLibraryPath [llvmPackages.libclang.lib];
        LD_LIBRARY_PATH = makeLibraryPath [
          libxkbcommon
          alsaLib
          wayland
          vulkan-loader
          vulkan-headers
          udev
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
          export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
        '';
        # Add precompiled library to rustc search path
        RUSTFLAGS = builtins.map (a: ''-L ${a}/lib'') [
          # add libraries here (e.g. pkgs.libvmi)
        ];
        # Add glibc, clang, glib and other headers to bindgen search path
        BINDGEN_EXTRA_CLANG_ARGS =
          # Includes with normal include path
          (builtins.map (a: ''-I"${a}/include"'') [
            # add dev libraries here (e.g. pkgs.libvmi.dev)
            pkgs.glibc.dev
          ])
          # Includes with special directory paths
          ++ [
            ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
            ''-I"${pkgs.glib.dev}/include/glib-2.0"''
            ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          ];
      };
  };
}
