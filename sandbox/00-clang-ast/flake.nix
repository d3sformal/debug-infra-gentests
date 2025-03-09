{
  description = "Nix Flake for C++ sandbox";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self , nixpkgs ,... }: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
      llvm = pkgs.llvmPackages_latest;
    in pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } rec {
      packages = with pkgs; [
        libgcc
        clang-tools
        cppcheck
        cmake
        llvm.lldb
        llvm.libstdcxxClang
        llvm.libllvm
        llvm.libcxx
        valgrind
        gtest
      ];

      shellHook = ''
        echo "clang: `${pkgs.clang}/bin/clang --version`"
      '';
    };
  };
}