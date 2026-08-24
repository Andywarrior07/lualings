use clap::Parser;
use mlua::{Lua, Result as LuaResult};

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
    
    // Prueba de mlua (Lua 5.4)
    let lua = Lua::new();
    let greet = format!("print('Hola, {} desde Lua!')", args.name);
    lua.load(&greet).exec()?;

    Ok(())
}
