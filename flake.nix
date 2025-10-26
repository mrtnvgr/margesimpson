{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { flake-parts, ... } @ inputs: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" ];

    perSystem = { pkgs, ... }: {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "margesimpson";
        version = "0.0.0-dev";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
    };
  };
}
