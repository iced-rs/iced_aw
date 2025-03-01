{
  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/unstable";
    # https://nixpk.gs/pr-tracker.html?pr=383902
    nixpkgs.url = "github:NixOS/nixpkgs/staging";
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
            languages.rust.mold.enable = false;
            # packages = with pkgs; [ ];

            env = {
              RUST_LOG = "info";
            };
          };
        };
    };
}
