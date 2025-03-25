{
  inputs = {
    nix-ros-overlay.url = "github:lopsided98/nix-ros-overlay/develop";
    nixpkgs.follows = "nix-ros-overlay/nixpkgs";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };
  outputs =
    {
      self,
      nix-ros-overlay,
      nixpkgs,
      nixpkgs-unstable,
    }:
    let
      # List of ROS2 distributions can be found here: https://docs.ros.org/en/jazzy/Releases.html
      ros2-distribution = "humble";
    in
    nix-ros-overlay.inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ nix-ros-overlay.overlays.default ];
        };
        pkgs-unstable = import nixpkgs-unstable { inherit system; };
      in

      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = buildInputs;
          buildInputs =
            (with pkgs-unstable; [
              rustc
              rustfmt
              rust-analyzer
              cargo
            ])
            ++ (with pkgs; [
              libclang.lib
              clang
              clang-tools
              gcc
              stdenv
              pkg-config
              cmake
              binutils
              colcon
              (
                with pkgs.rosPackages.${ros2-distribution};
                buildEnv {
                  paths = [
                    ros-core
                    turtlesim
                    # ... other ROS packages
                  ];
                }
              )
            ]);
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );

  nixConfig = {
    extra-substituters = [ "https://ros.cachix.org" ];
    extra-trusted-public-keys = [ "ros.cachix.org-1:dSyZxI8geDCJrwgvCOHDoAfOm5sV1wCPjBkKL+38Rvo=" ];
  };
}
