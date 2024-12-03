{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    wgsl_analyzer = {
      url = "github:wgsl-analyzer/wgsl-analyzer";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, ... }@inputs:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems =
        function: nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});
    in
    {
      overlays.default = final: prev: {
        rustToolchain = prev.rust-bin.rust.fromRustupToolchainFile ./rust-toolchain.toml;
      };

      devShells = forAllSystems (pkgs: {
        default =
          with pkgs;
          mkShell rec {
            buildInputs = [
              cargo
              rustc
              rust-analyzer-unwrapped
              rustfmt
              clippy
              pkg-config
              vulkan-loader
              renderdoc
              vulkan-headers
              vulkan-validation-layers
              vulkan-tools
              inputs.wgsl_analyzer.packages.${system}.default

              libxkbcommon
              libGL

              wayland

              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
            ];
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };

      });

      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./default.nix { };
      });
    };
}
