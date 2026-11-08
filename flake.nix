{
  description = "cantrip — speak plainly, get the incantation";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
  outputs = { self, nixpkgs }:
    let
      systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ];
      eachSystem = nixpkgs.lib.genAttrs systems;
      mkPackage = pkgs: pkgs.rustPlatform.buildRustPackage {
        pname = "cantrip-cli";
        version = "0.1.0";
        src = pkgs.lib.cleanSource self;
        cargoLock.lockFile = ./Cargo.lock;
        nativeBuildInputs = [ pkgs.installShellFiles ];
        postInstall = ''
          installShellCompletion --cmd cantrip \
            --bash <($out/bin/cantrip completions bash) \
            --zsh <($out/bin/cantrip completions zsh) \
            --fish <($out/bin/cantrip completions fish)
        '';
        meta = {
          description = "A beautiful, keyless shell companion";
          homepage = "https://github.com/flockcap/cantrip";
          license = with pkgs.lib.licenses; [ mit asl20 ];
          mainProgram = "cantrip";
          platforms = systems;
        };
      };
    in {
      packages = eachSystem (system: let pkgs = import nixpkgs { inherit system; }; in rec {
        cantrip = mkPackage pkgs;
        default = cantrip;
      });
      overlays.default = final: _prev: { cantrip = mkPackage final; };
      homeManagerModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.programs.cantrip;
          toml = pkgs.formats.toml { };
        in {
          options.programs.cantrip = {
            enable = lib.mkEnableOption "cantrip";
            package = lib.mkOption {
              type = lib.types.package;
              default = self.packages.${pkgs.stdenv.hostPlatform.system}.cantrip;
              description = "The cantrip package to install.";
            };
            settings = lib.mkOption {
              type = toml.type;
              default = { };
              description = "Configuration written to cantrip/config.toml.";
              example = { theme = "gruvbox"; include_history = false; };
            };
            enableZshIntegration = lib.mkEnableOption "the Alt-G zsh widget";
            enableFishIntegration = lib.mkEnableOption "the Alt-G fish widget";
          };
          config = lib.mkIf cfg.enable {
            home.packages = [ cfg.package ] ++ lib.optional (cfg.enableZshIntegration || cfg.enableFishIntegration) pkgs.python3;
            xdg.configFile."cantrip/config.toml".source = toml.generate "cantrip-config.toml" cfg.settings;
            programs.zsh.initContent = lib.mkIf cfg.enableZshIntegration ''
              source <(${cfg.package}/bin/cantrip widget zsh)
            '';
            programs.fish.interactiveShellInit = lib.mkIf cfg.enableFishIntegration ''
              ${cfg.package}/bin/cantrip widget fish | source
            '';
          };
        };
      homeManagerModules.cantrip = self.homeManagerModules.default;
      checks = eachSystem (system: { package = self.packages.${system}.cantrip; });
    };
}
