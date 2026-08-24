use mlua::{Lua, Result as LuaResult};

pub fn run_source(source: &str) -> LuaResult<()> {
    let lua = Lua::new();
    lua.load(source).exec()
}

#[cfg(test)]
mod tests {
    use super::run_source;

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
