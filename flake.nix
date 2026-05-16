{
  description = "A pure Nix environment for your GPUI Project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      formatter = forAllSystems (pkgs: pkgs.alejandra);
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            pkg-config
          ];

          buildInputs = with pkgs; [
            fontconfig
            freetype
            libxkbcommon
            wayland
            xorg.libxcb
            xorg.libX11
            vulkan-loader
            mesa
            libglvnd
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            fontconfig
            freetype
            libxkbcommon
            wayland
            xorg.libxcb
            xorg.libX11
            vulkan-loader
            mesa
            libglvnd
          ]);

          packages = with pkgs; [
            rustc
            clippy
            rust-analyzer
            yazi

          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          SDKROOT = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";
          BINDGEN_EXTRA_CLANG_ARGS = "-isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk -F/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks";

          shellHook = ''
            mkdir -p /tmp/nix-xcode-shims
            ln -sf /usr/bin/xcrun /tmp/nix-xcode-shims/xcrun
            export PATH="/tmp/nix-xcode-shims:/Applications/Xcode.app/Contents/Developer/usr/bin:$PATH"

            if [ -d /run/opengl-driver/lib ]; then
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/run/opengl-driver/lib"
            fi

            echo "Rust development shell active! (rustc ${pkgs.rustc.version})"
          '';
        };
      });
    };
}
