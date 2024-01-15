{ pkgs ? import <nixpkgs> { } }:
let
  alias-run = pkgs.writeShellScriptBin "d" ''cargo-watch -x run -c'';
  alias-test = pkgs.writeShellScriptBin "t" ''cargo-watch -x test -c'';
in
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  buildInputs = with pkgs; [
    cargo-watch
  ] ++ [ alias-run alias-test ];
  shellHook = ''
    printf "\e[33m
      \e[1md\e[0m\e[33m  -> to run
      \e[1mt\e[0m\e[33m -> to tests
    \e[0m"
  '';
}
