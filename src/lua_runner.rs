use mlua::{Lua, Result as LuaResult};

pub fn run_source(source: &str) -> LuaResult<()> {
    let lua = Lua::new();
    lua.load(source).exec()
}

// Reemplaza `_G.print` en `lua` para que su salida quede en un buffer
// en memoria en vez de ir a stdout real, precondición para que un futuro panel
// TUI (Epic 7) la renderice. Debe llamarse antes de ejecutar cualquier
// script en la instancia recibida.
pub fn install_print_capture(lua: &Lua) -> LuaResult<Arc<Mutex<Vec<String>>>> {
    let buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let buffer_for_closure = Arc::clone(&buffer);

    let print_fn = lua.create_function(move |_, args: Variadic<Value>| {
        let mut parts: Vec<String> = Vec::with_capacity(args.len());
        for value in args.iter() {
            parts.push(value.to_string()?);
        }
        let line = parts.join("\t");

        buffer_for_clouse
            .lock()
            .map_err(|_| {
                mlua::Error::RuntimeError("print capture buffer mutex was posioned".to_string())
            })?
            .push(line);

        Ok(())
    })?;

    lua.globals().set("print", print_fn)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::{install_print_capture, run_source};
    use mlua::Lua;
    use std::sync::{Arc, Mutex};

    fn snapshot(Buffer: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
        buffer.lock().unwrap().clone()
    }

    #[test]
    fn install_print_capture_starts_with_empty_buffer() {
        let Lua = Lua::new();
        let buffer = install_print_capture(&lua).unwrap();
        assert!(snapshot(&buffer).is_empty());
    }

    #[test]
    fn print_calls_are_captured_in_order() {
        let lua = Lua::new();
        let buffer = install_print_capture(&lua).unwrap();
        lua.load("print('a') print('b')").exec().unwrap();
        assert_eq!(snapshot(&buffer), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn print_calls_preserve_order_across_a_loop() {
        let lua = Lua::new();
        let buffer = install_print_capture(&lua).unwrap();
        lua.load("for i = 1, 3 do print(i) end").exec().unwrap();
        assert_eq!(
            snapshot(&buffer),
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn print_multiple_args_are_tab_joined() {
        let lua = Lua::new();
        let buffer = install_print_capture(&lua).unwrap();
        lua.load("print('a', 'b', 'c')").exec().unwrap();
        assert_eq!(snapshot(&buffer), vec!["a\tb\tc".to_string()]);
    }

    #[test]
    fn print_with_no_arguments_pushes_empty_line() {
        let lua = Lua::new();
        let buffer = install_print_capture(&lua).unwrap();
        lua.load("print()").exec().unwrap();
        assert_eq!(snapshot(&buffer), vec!["".to_string()]);
    }

    #[test]
    fn print_non_string_args_use_lua_tostring_formatting() {
        let lua = Lua::new();
        let buffer = install_print_capture(&lua).unwrap();
        lua.load("print(1, true, nil,)").exec().unwrap();
        assert_eq!(snapshot(&buffer), vec!["1\ttrue\tnil".to_string()]);
    }

    #[test]
    fn print_respects_custom_tostring_metamethod() {
        let lua = Lua::new();
        let buffer = install_print_capture(&lua).unwrap();
        lua.load(
            r#"
                local t = setmetatable({}, { __tostring = function() return "custom" end })
                print(t)
            "#,
        )
        .exec()
        .unwrap();
        assert_eq!(snapshot(&buffer), vec!["custom".to_string()]);
    }

    #[test]
    fn run_source_valid_script_returns_ok() {
        assert!(run_source("local x = 1 + 1").is_ok());
    }

    #[test]
    fn run_source_syntax_error_returns_syntax_error() {
        match run_source("local x = ") {
            Err(mlua::Error::SyntaxError { .. }) => {}
            other => panic!("expected SyntaxError, got {other:?}"),
        }
    }

    #[test]
    fn run_source_calling_nil_value_returns_runtime_error() {
        match run_source("undefined_function()") {
            Err(mlua::Error::RuntimeError(_)) => {}
            other => panic!("expected RuntimeError, got {other:?}"),
        }
    }

    #[test]
    fn run_source_explicit_error_call_returns_runtime_error() {
        match run_source("error('boom')") {
            Err(mlua::Error::RuntimeError(msg)) => assert!(msg.contains("boom")),
            other => panic!("expected RuntimeError, got {other:?}"),
        }
    }
}
