use emacs::{defun, Env, Result, Value};

// use syntect::easy::HighlightLines;
// use syntect::parsing::SyntaxSet;
// use syntect::highlighting::{ThemeSet, Style};
// use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// Emacs won't load the module without this.
emacs::plugin_is_GPL_compatible!();

use syntect::parsing::{Scope as OtherScope};
trait Scope
{
    fn emacs_fn_
}

#[cfg(test)]
mod tests {
    // use syntect::parsing::{ParseState, ScopeStackOp};
    // use syntect::easy::HighlightLines;
    use syntect::parsing::{ScopeStackOp};

    use syntect::parsing::SyntaxSet;
    // use syntect::highlighting::{ThemeSet, Style};
    // use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
    #[test]
    fn ss1()
    {
    }

    #[test]
    fn ss() {
        let ps = SyntaxSet::load_defaults_newlines();
        let syntax = ps.find_syntax_by_extension("java").unwrap();
        // let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        let s = "package temp;\n\nclass App {\n  public static void main(String[] args) {\n    System.out.print(123);\n    foo();\n    if (true) {\n       foo();\n    }\n    bar();\n  }\n\n  static int foo() {\n    new App();\n    return 10;\n  }\n\n  static int bar() {\n    new App();\n    return 10;\n  }\n}\n";

        let s1= "import React, { Component } from 'react';\nimport logo from './logo.svg';\nimport './App.css';\n\n// console.l\n\nclass App extends Component {\n  render() {\n    return (\n      <div className=\"App\">\n        <div className=\"App-header\">\n          <img src={logo} className=\"App-logo\" alt=\"logo\" />\n          <h2>Welcome to React</h2>\n        </div>\n        <p className=\"App-intro\">\n          To get started, edit <code>src/App.js</code> and save to reload.\n        </p>\n      </div>\n    );\n  }\n}\n\nexport default App;\n";

        let mut pp = syntect::parsing::ParseState::new(syntax);

        let x = pp.parse_line(s, &ps);

        println!("Size before {}", x.len());

        let mut i = 0;

        let bb: Vec<_> =
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

        println!("Size after {}", bb.len());

        let jointed = bb.join("\n");
        println!("{}", &jointed);
    }
}

fn demo (_env: &Env) -> ()
{
}

// Register the initialization hook that Emacs will call when it loads the module.
#[emacs::module]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done loading!")
}

// Define a function callable by Lisp code.
#[defun]
fn say_hello(env: &Env, name: String) -> Result<Value<'_>> {
    demo(env);
    // env.intern
    env.message(&format!("Hello, {}!", name))
}
