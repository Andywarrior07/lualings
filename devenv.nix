{ pkgs, ... }:

{
  # 1. Configuración del lenguaje Rust
  languages.rust = {
    enable = true;
    # Opcional: puedes fijar un canal específico si lo deseas
    # channel = "stable";
  };

  # 2. Paquetes del sistema necesarios para tu stack
  packages = with pkgs; [
    gcc # Compilador de C (Requerido por mlua 'vendored' para compilar Lua)
    pkg-config # Ayuda a encontrar librerías nativas durante la compilación
    openssl # Muy común en proyectos Rust (por si alguna dependencia lo requiere)
    cargo-watch # Herramienta essential para desarrollo: recompila al guardar
    bacon # Alternativa excelente a cargo-watch para TUIs
  ];

  # 3. Variables de entorno útiles
  env = {
    # Muestra los errores de compilación de Rust en formato JSON (ideal para editores)
    RUST_BACKTRACE = "1";
    # Evita que Cargo haga fetch de la red si está offline (opcional)
    # CARGO_NET_OFFLINE = "false";
  };

  # 4. Scripts de utilidad (se ejecutan con `devenv run <nombre>`)
  scripts = {
    dev.exec = "cargo watch -x run"; # Ejecuta `devenv run dev` para empezar a programar
    check.exec = "cargo clippy -- -D warnings"; # Lints estrictos
  };

  # 5. Integración con editores (opcional pero recomendado)
  # Hace que rust-analyzer funcione perfectamente dentro de devenv
  enterShell = ''
    echo "🦀 Entorno Rust listo! Ejecuta 'devenv run dev' para empezar."
  '';
}
