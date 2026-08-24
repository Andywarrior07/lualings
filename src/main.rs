use clap::Parser;
use lualings::lua_runner;
use mlua::Result as LuaResult;

/// Un simple programa de ejemplo
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Nombre a saludar
    #[arg(short, long, default_value = "Mundo")]
    name: String,
}

fn main() -> LuaResult<()> {
    // Parseo de argumentos con Clap
    let args = Args::parse();

    let greet = format!("print('Hola, {} desde Lua!')", args.name);
    lua_runner::run_source(&greet)
}
