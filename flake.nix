{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs = {
      nixpkgs.follows = "nixpkgs";
    };

  };
  nixConfig = {
    extra-trusted-public-keys = "iced-aw.cachix.org-1:fQP3/gBialDFi/hSqadx5ZalTWyYTJ3LWgU7b/elZ0A=";
    extra-substituters = "https://iced-aw.cachix.org";
  };
  outputs =
    {
      self,
      nixpkgs,
      devenv,
      systems,
      ...
    }@inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      devShells = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
          lib = pkgs.lib;

          comet = pkgs.rustPlatform.buildRustPackage {
            pname = "iced_comet";
            version = "0.14.0";

            src = pkgs.fetchFromGitHub {
              owner = "iced-rs";
              repo = "comet";
              rev = "0.14.0"; # == bb2a21dc9475b44b90bfebea57ac539502d2535b
              hash = "sha256-4p5y8NdI1r1mX4gCGn3Ee8+/41zTv4n1CPlKV0iUcOw="; # ← src hash
            };
            cargoHash = "sha256-Tw0zvk3G3cd3h/MwH/zesK/F17l5S5cke6fVfqRnq4E=";

            nativeBuildInputs = with pkgs; [
              pkg-config
              makeWrapper
            ];
            buildInputs = [ ];

            postInstall = ''
              wrapProgram $out/bin/iced_comet \
              # expose the conventional `comet` name too
              ln -s iced_comet $out/bin/comet
            '';

            doCheck = false;
          };

        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              {
                languages.rust.enable = true;
                languages.rust.mold.enable = true; # DO NOT USE WILD
                languages.rust.channel = "stable";
                languages.rust.targets = [
                  "aarch64-linux-android"
                  "armv7-linux-androideabi"
                  "x86_64-linux-android"
                  "i686-linux-android"
                ];
                # https://devenv.sh/reference/options/
                packages = [
                  pkgs.act

                  comet
                ];
                enterShell = "";
                env = {
                  LD_LIBRARY_PATH = lib.makeLibraryPath (
                    with pkgs;
                    [
                      libGL
                      libxkbcommon
                      vulkan-loader
                      wayland
                      libXcursor
                      libXrandr
                      libXi
                      libX11
                    ]
                  );
                  ICED_BACKEND = "wgpu";
                  RUST_LOG = "info";
                };
              }
            ];
          };
        }
      );
    };
}
