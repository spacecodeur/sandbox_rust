// -----------------------------------------------------------------------------
// The `goals!` macro is a helper to make multiple calls to the `goal!` macro,
// while preserving the function names as *literal paths* for logging.
//
// WHY THIS MACRO EXISTS:
//
// Normally, if we stored function pointers in a vector like this:
//     vec![(some_module::some_func, "Some message")]
// and then looped over it:
//     for (func, msg) in vec { goal!(func, msg); }
//
// The macro `goal!` would receive `func` (a variable name), and calling
// `stringify!(func)` inside the macro would just give the string "func",
// NOT "some_module::some_func" — and that defeats the purpose of showing
// the function path in logs.
//
// This `goals!` macro solves that problem by letting us write all function
// calls inline, *explicitly using* their full paths. This way, `goal!` can
// stringify the function name properly.
//
// USAGE:
//
// Instead of using a vector, we write:
//
//     goals! {
//         some_module::some_func => "First goal description",
//         another::func => "Second goal description",
//     }
//
// This expands to:
//
//     goal!(some_module::some_func, "First goal description");
//     goal!(another::func, "Second goal description");
//
// SYNTAX BREAKDOWN:
//
// - $( ... ),* : A repetition over multiple items separated by commas.
// - $func:path : Matches a Rust path like `module::submodule::func`.
// - $msg:expr  : Matches any valid Rust expression (e.g., a string literal).
// - $(,)?      : Allows an optional trailing comma.
//
// The macro expands into multiple calls to `goal!`, forwarding the function
// path and message.
//
// NOTE:
//
// - `$crate::goal!` is used to call the `goal!` macro from the same crate,
//   even if this macro is used in another crate that depends on it. This
//   ensures macro resolution works correctly across crate boundaries.
// -----------------------------------------------------------------------------
#[macro_export]
macro_rules! goals {
    (
        $( $func:path => $msg:expr ),* $(,)?
    ) => {
        $(
            $crate::goal!($func, $msg);
        )*
    };
}

#[macro_export]
macro_rules! goal {
    ($func:path, $message:expr) => {{
        goal_log::log(stringify!($func), $message);
        $func();
    }};
}
