// use syntect::parsing::Scope;
use emacs::IntoLisp;
use emacs::{defun, Env, Result, Value};


// Emacs won't load the module without this.
emacs::plugin_is_GPL_compatible!();


use std::collections::HashMap;
use std::cell::RefCell;

thread_local!{
    pub static SCOPE_TO_FACE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new())
}

fn get_emacs_face(k: &String) -> String {
    return SCOPE_TO_FACE.with(|data| data.borrow().get(k).unwrap().clone());
}

fn set_emacs_face(k: &String) -> String {
    return SCOPE_TO_FACE.with(|data| data.borrow().get(k).unwrap().clone());
}

#[cfg(test)]
mod tests {
}

use syntect::parsing::{ScopeStackOp};
use syntect::parsing::SyntaxSet;

fn syntax(s: &String, syntax: &String) -> std::vec::Vec<std::string::String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let syntax = ps.find_syntax_by_extension(syntax).unwrap();
    let mut parser_state = syntect::parsing::ParseState::new(syntax);
    let x = parser_state.parse_line(&s, &ps);

    let mut i = 0;

    let bb: Vec<String> =
        x.iter()
        .filter_map(|x1|
                    match x1.1 {
                        ScopeStackOp::Push(z) => {
                            i += 1;
                            Some(format!("{} - Push {} {}",
                                         " ".repeat(i),
                                         z.build_string(),
                                         x1.0))
                        },
                        ScopeStackOp::Pop(z) => {
                            i -= z;
                            Some(format!("{} - Pop {}", " ".repeat(i + 1), z))
                        },
                        ScopeStackOp::Restore  => Some(format!("Restore")),
                        _ => None
                    }).collect();

    bb
}

// Register the initialization hook that Emacs will call when it loads the module.
#[emacs::module]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done loading!")
}

// impl IntoLisp<'_> for Vec<IntoLisp<'_>>
// {
//     fn into_lisp(self, env: &Env) -> Result<Value<'_>> {

// {}{}}}


// Define a function callable by Lisp code.
#[defun]
fn fontlock_data(env: &Env, buffer_string: String, file_name: String) {
    let mut vec : Vec<Value<'_>> = Vec::with_capacity(1);
    let xx =  "xx".into_lisp(env).unwrap();
    vec.push(xx);
    return env.call("list", &vec);
}
