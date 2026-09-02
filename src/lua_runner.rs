use mlua::{HookTriggers, Lua, Result as LuaResult, Value, Variadic, VmState};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub const DEFAULT_TIMEOUT_BUDGET: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Fail(String),
    Pass,
    Timeout,
}

fn classify_error(err: mlua::Error) -> Outcome {
    if is_timeout_error(&err) {
        Outcome::Timeout
    } else {
        Outcome::Fail(err.to_string())
    }
}

pub fn run_source(source: &str) -> LuaResult<()> {
    let lua = Lua::new();
    lua.load(source).exec()
}

pub fn install_timeout_hook(lua: &Lua, budget: Duration) -> LuaResult<()> {
    let start = std::time::Instant::now();
    lua.set_hook(
        HookTriggers::new().every_nth_instruction(10_000),
        move |_, _| {
            if start.elapsed() > budget {
                return Err(mlua::Error::RuntimeError(format!(
                    "timeout: script exceeded {budget:?} execution budget"
                )));
            }
            Ok(VmState::Continue)
        },
    )
}

pub fn is_timeout_error(err: &mlua::Error) -> bool {
    err.to_string().contains("timeout")
}

fn run_test_with_budget(source: &str, budget: Duration) -> Outcome {
    let lua = Lua::new();
    if let Err(err) = install_timeout_hook(&lua, budget) {
        return classify_error(err);
    }
    if let Err(err) = lua.load(source).exec() {
        return classify_error(err);
    }

    match lua.globals().get::<Value>("__lualings_pass") {
        Ok(Value::Boolean(true)) => Outcome::Pass,
        _ => Outcome::Fail("_G.__lualings_pass was not set to true".to_string()),
    }
}

pub fn run_test(source: &str) -> Outcome {
    run_test_with_budget(source, DEFAULT_TIMEOUT_BUDGET)
}

fn run_compile_with_budget(source: &str, budget: Duration) -> Outcome {
    let lua = Lua::new();
    if let Err(err) = install_timeout_hook(&lua, budget) {
        return classify_error(err);
    }
    match lua.load(source).exec() {
        Ok(()) => Outcome::Pass,
        Err(err) => classify_error(err),
    }
}

pub fn run_compile(source: &str) -> Outcome {
    run_compile_with_budget(source, DEFAULT_TIMEOUT_BUDGET)
}

pub fn install_print_capture(lua: &Lua) -> LuaResult<Arc<Mutex<Vec<String>>>> {
    let buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let buffer_for_closure = Arc::clone(&buffer);

    let print_fn = lua.create_function(move |_, args: Variadic<Value>| {
        let mut parts: Vec<String> = Vec::with_capacity(args.len());
        for value in args.iter() {
            parts.push(value.to_string()?);
        }
        let line = parts.join("\t");

        buffer_for_closure
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

fn run_compile_capturing_with_budget(source: &str, budget: Duration) -> (Vec<String>, Outcome) {
    let lua = Lua::new();
    let buffer = match install_print_capture(&lua) {
        Ok(buffer) => buffer,
        Err(err) => return (Vec::new(), classify_error(err)),
    };
    if let Err(err) = install_timeout_hook(&lua, budget) {
        return (Vec::new(), classify_error(err));
    }
    let outcome = match lua.load(source).exec() {
        Ok(()) => Outcome::Pass,
        Err(err) => classify_error(err),
    };
    (buffer.lock().unwrap().clone(), outcome)
}

pub fn run_compile_capturing(source: &str) -> (Vec<String>, Outcome) {
    run_compile_capturing_with_budget(source, DEFAULT_TIMEOUT_BUDGET)
}

fn run_test_capturing_with_budget(source: &str, budget: Duration) -> (Vec<String>, Outcome) {
    let lua = Lua::new();
    let buffer = match install_print_capture(&lua) {
        Ok(buffer) => buffer,
        Err(err) => return (Vec::new(), classify_error(err)),
    };
    if let Err(err) = install_timeout_hook(&lua, budget) {
        return (Vec::new(), classify_error(err));
    }
    let outcome = match lua.load(source).exec() {
        Ok(()) => match lua.globals().get::<Value>("__lualings_pass") {
            Ok(Value::Boolean(true)) => Outcome::Pass,
            _ => Outcome::Fail("_G.__lualings_pass was not set to true".to_string()),
        },
        Err(err) => classify_error(err),
    };
    (buffer.lock().unwrap().clone(), outcome)
}

pub fn run_test_capturing(source: &str) -> (Vec<String>, Outcome) {
    run_test_capturing_with_budget(source, DEFAULT_TIMEOUT_BUDGET)
}

#[cfg(test)]
mod tests {
    use super::{
        Outcome, install_print_capture, install_timeout_hook, is_timeout_error, run_compile,
        run_compile_capturing, run_compile_capturing_with_budget, run_compile_with_budget,
        run_source, run_test, run_test_capturing, run_test_capturing_with_budget,
        run_test_with_budget,
    };
    use mlua::Lua;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    fn snapshot(buffer: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
        buffer.lock().unwrap().clone()
    }

    fn assert_pass(outcome: &Outcome) {
        assert!(
            matches!(outcome, Outcome::Pass),
            "Expected Pass, got {outcome:?}"
        );
    }

    fn assert_fail(outcome: &Outcome) {
        assert!(
            matches!(outcome, Outcome::Fail(_)),
            "expected Fail, got {outcome:?}"
        );
    }

    fn assert_timeout(outcome: &Outcome) {
        assert!(
            matches!(outcome, Outcome::Timeout),
            "expected Timeout, got {outcome:?}"
        );
    }

    #[test]
    fn install_timeout_hook_does_not_effect_normal_scripts() {
        let lua = Lua::new();
        install_timeout_hook(&lua, Duration::from_secs(2)).unwrap();
        assert!(lua.load("local x = 1 + 1").exec().is_ok());
    }

    #[test]
    fn install_timeout_hook_stops_infinite_loop_within_budget() {
        let lua = Lua::new();
        let budget = Duration::from_millis(100);
        install_timeout_hook(&lua, budget).unwrap();

        let start = std::time::Instant::now();
        let result = lua.load("while true do end").exec();
        let elapsed = start.elapsed();

        assert!(result.is_err());
        assert!(elapsed < budget + Duration::from_secs(1));
    }

    #[test]
    fn timeout_error_message_is_detectable_regardless_of_error_shape() {
        let lua = Lua::new();
        install_timeout_hook(&lua, Duration::from_millis(100)).unwrap();
        match lua.load("while true do end").exec() {
            Err(mlua::Error::RuntimeError(msg)) => assert!(msg.contains("timeout")),
            Err(mlua::Error::CallbackError { cause, .. }) => {
                assert!(cause.to_string().contains("timeout"));
            }
            other => panic!("expected RuntimeError or CallbackError, got {other:?}"),
        }
    }

    #[test]
    fn is_timeout_error_true_for_actual_timeout() {
        let lua = Lua::new();
        install_timeout_hook(&lua, Duration::from_millis(100)).unwrap();
        let err = lua.load("while true do end").exec().unwrap_err();
        assert!(is_timeout_error(&err));
    }

    #[test]
    fn is_timeout_error_false_for_runtime_error() {
        let err = run_source("error('boom')").unwrap_err();
        assert!(!is_timeout_error(&err));
    }

    #[test]
    fn is_timeout_error_false_for_syntax_error() {
        let err = run_source("local x = ").unwrap_err();
        assert!(!is_timeout_error(&err));
    }

    #[test]
    fn run_test_passes_when_flag_set_to_true() {
        assert_pass(&run_test("_G.__lualings_pass = true"));
    }

    #[test]
    fn run_test_fails_when_script_errors_after_setting_flag() {
        assert_fail(&run_test("_G.__lualings_pass = true\nerror('boom')"));
    }

    #[test]
    fn run_test_fails_when_flag_not_set() {
        assert_fail(&run_test("local x = 1"));
    }

    #[test]
    fn run_test_fails_when_script_errors_before_setting_flag() {
        assert_fail(&run_test("error('boom')"));
    }

    #[test]
    fn run_test_fails_when_flag_is_false() {
        assert_fail(&run_test("_G.__lualings_pass = false"));
    }

    #[test]
    fn run_test_fails_when_flag_is_truthy_non_boolean() {
        assert_fail(&run_test("_G.__lualings_pass = 1"));
        assert_fail(&run_test("_G.__lualings_pass = 'true'"));
    }

    #[test]
    fn run_test_infinite_loop_times_out() {
        assert_timeout(&run_test_with_budget(
            "while true do end",
            Duration::from_millis(100),
        ));
    }

    #[test]
    fn run_compile_valid_script_passes() {
        assert_pass(&run_compile("local x = 1 + 1"));
    }

    #[test]
    fn run_compile_syntax_error_fails() {
        assert_fail(&run_compile("local x = "));
    }

    #[test]
    fn run_compile_runtime_error_fails() {
        assert_fail(&run_compile("error('boom')"));
    }

    #[test]
    fn run_compile_global_assignment_passes() {
        assert_pass(&run_compile("x = 5"));
    }

    #[test]
    fn run_compile_infinite_loop_times_out() {
        assert_timeout(&run_compile_with_budget(
            "while true do end",
            Duration::from_millis(100),
        ));
    }

    #[test]
    fn run_compile_fail_message_is_readable_not_debug() {
        match run_compile("error('boom')") {
            Outcome::Fail(msg) => {
                assert!(msg.contains("boom"));
                assert!(
                    !msg.contains("RuntimeError("),
                    "message looks like Debug output: {msg}"
                );
            }
            other => panic!("expected Fail, got {other:?}"),
        }
    }

    #[test]
    fn install_print_capture_starts_with_empty_buffer() {
        let lua = Lua::new();
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
        lua.load("print(1, true, nil)").exec().unwrap();
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

    #[test]
    fn run_compile_capturing_returns_pass_and_the_printed_output() {
        let (output, outcome) = run_compile_capturing("print('a') print('b')");
        assert_pass(&outcome);
        assert_eq!(output, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn run_test_capturing_returns_pass_and_the_printed_output() {
        let (output, outcome) = run_test_capturing("print('checking') _G.__lualings_pass = true");
        assert_pass(&outcome);
        assert_eq!(output, vec!["checking".to_string()]);
    }

    #[test]
    fn run_compile_capturing_returns_output_printed_before_the_failure() {
        let (output, outcome) = run_compile_capturing("print('before') error('boom')");
        assert_fail(&outcome);
        assert_eq!(output, vec!["before".to_string()]);
    }

    #[test]
    fn run_test_capturing_times_out_and_still_returns_partial_output() {
        let (output, outcome) = run_test_capturing_with_budget(
            "print('before') while true do end",
            Duration::from_millis(100),
        );
        assert_timeout(&outcome);
        assert_eq!(output, vec!["before".to_string()]);
    }

    #[test]
    fn run_compile_capturing_times_out_and_still_returns_partial_output() {
        let (output, outcome) = run_compile_capturing_with_budget(
            "print('before') while true do end",
            Duration::from_millis(100),
        );
        assert_timeout(&outcome);
        assert_eq!(output, vec!["before".to_string()]);
    }
}
