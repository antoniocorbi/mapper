{
  description = "eframe devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        #overlays = [ (import rust-overlay) ];
        #pkgs = import nixpkgs { inherit system overlays; };
        pkgs = import nixpkgs {inherit system;};

        # Definimos las librerías que se necesitan tanto en compilación como en ejecución
        runtimeLibs = with pkgs; [
          libxkbcommon
          libGL
          fontconfig
          wayland
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          # Herramientas de compilación
          nativeBuildInputs = with pkgs; [
            pkg-config
            # (rust-bin.stable.latest.default.override {
            #   extensions = [ "rust-src" "rust-analyzer" ];
            # })
            trunk # Útil si decides compilar para Web/WASM más adelante
          ];

          # Librerías necesarias
          buildInputs = with pkgs; [
            openssl
          ] ++ runtimeLibs;

          # Configuración para que Rust encuentre las librerías gráficas
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
        };
      });
}

# {
#   description = "eframe devShell";
#
#   inputs = {
#     nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
#     rust-overlay.url = "github:oxalica/rust-overlay";
#     flake-utils.url = "github:numtide/flake-utils";
#   };
#
#   outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
#     flake-utils.lib.eachDefaultSystem (system:
#       let
#         overlays = [ (import rust-overlay) ];
#         pkgs = import nixpkgs { inherit system overlays; };
#       in with pkgs; {
#         devShells.default = mkShell rec {
#           buildInputs = [
#             # Rust
#             ##################################
#             # rust-bin.stable.latest.default #
#             # trunk                          #
#             ##################################
#
#             # misc. libraries
#             openssl
#             pkg-config
#
#             # GUI libs
#             libxkbcommon
#             libGL
#             fontconfig
#
#             # wayland libraries
#             wayland
#
#             # x11 libraries
#             xorg.libXcursor
#             xorg.libXrandr
#             xorg.libXi
#             xorg.libX11
#
#           ];
#
#           LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
#         };
#       });
# }
