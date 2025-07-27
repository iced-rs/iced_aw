{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";

  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem =
        {
          lib,
          pkgs,
          ...
        }:
        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.
          devenv.shells.default = {
            # https://devenv.sh/reference/options/
            dotenv.disableHint = true;

            languages.rust.enable = true;
            # languages.rust.channel = "stable";
            packages = with pkgs; [ ];

            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath (
                with pkgs;
                [
                  libGL
                  libxkbcommon
                  vulkan-loader
                  wayland
                  xorg.libXcursor
                  xorg.libXrandr
                  xorg.libXi
                  xorg.libX11
                ]
              );
              ICED_BACKEND = "wgpu"; # wgpu or tiny-skia
              RUST_LOG = "info";
            };
          };
        };
    };
}
