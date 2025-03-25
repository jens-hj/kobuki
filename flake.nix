{
  inputs = {
    nix-ros-overlay.url = "github:lopsided98/nix-ros-overlay/develop";
    nixpkgs.follows = "nix-ros-overlay/nixpkgs";
  };
  outputs =
    {
      self,
      nix-ros-overlay,
      nixpkgs,
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
      in

      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            fish
            colcon
            (
              with pkgs.rosPackages.${ros2-distribution};
              buildEnv {
                paths = [
                  ros-core
                  # ... other ROS packages
                ];
              }
            )
          ];
        };
      }
    );

  nixConfig = {
    extra-substituters = [ "https://ros.cachix.org" ];
    extra-trusted-public-keys = [ "ros.cachix.org-1:dSyZxI8geDCJrwgvCOHDoAfOm5sV1wCPjBkKL+38Rvo=" ];
  };
}
