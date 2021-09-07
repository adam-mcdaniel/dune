use dune::{parse_script, Environment, Error, Expression, Int, SyntaxError};

use rustyline::{error::ReadlineError, Editor, Helper};
use rustyline::completion::{Completer, FilenameCompleter, Pair as PairComplete};
use rustyline::config::OutputStreamType;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{
    MatchingBracketValidator, ValidationContext, ValidationResult, Validator,
};
use rustyline::{CompletionType, Config, Context, EditMode};
use rustyline_derive::Helper;


use common_macros::b_tree_map;

use rand::{distributions::Uniform, seq::SliceRandom, thread_rng, Rng};

use std::{
    borrow::Cow::{self, Borrowed, Owned},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use os_info::Type;

#[derive(Helper)]
struct DuneHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
    env: Environment,
}

impl DuneHelper {
    fn set_prompt(&mut self, prompt: impl ToString) {
        self.colored_prompt = prompt.to_string();
    }

    fn update_env(&mut self, env: &Environment) {
        self.env = env.clone();
    }
}

impl Completer for DuneHelper {
    type Candidate = PairComplete;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<PairComplete>), ReadlineError> {
        let mut path = PathBuf::from(self.env.get_cwd());
        let mut segment = String::new();

        if !line.is_empty() {
            for (i, ch) in line.chars().enumerate() {
                if ch.is_whitespace()
                    || ch == ';'
                    || ch == '\''
                    || ch == '('
                    || ch == ')'
                    || ch == '{'
                    || ch == '}'
                    || ch == '"'
                {
                    segment = String::new();
                } else {
                    segment.push(ch);
                }

                if i == pos {
                    break;
                }
            }

            if !segment.is_empty() {
                path.push(segment.clone());
            }
        }

        let path_str = (path.into_os_string().into_string().unwrap()
            + if segment.is_empty() { "/" } else { "" })
        .replace("/./", "/")
        .replace("//", "/");
        let (pos, mut pairs) = self
            .completer
            .complete(path_str.as_str(), path_str.len(), ctx)?;
        for pair in &mut pairs {
            pair.replacement = String::from(line) + &pair.replacement.replace(&path_str, "");
        }
        Ok((pos, pairs))
    }
}

impl Hinter for DuneHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        let mut segment = String::new();

        if !line.is_empty() {
            for (i, ch) in line.chars().enumerate() {
                if ch.is_whitespace()
                    || ch == ';'
                    || ch == '\''
                    || ch == '('
                    || ch == ')'
                    || ch == '{'
                    || ch == '}'
                    || ch == '"'
                {
                    segment = String::new();
                } else {
                    segment.push(ch);
                }

                if i == pos {
                    break;
                }
            }
        }

        let cmds = vec![
            "exit 0", "ls ", "rm -ri ", "cp -r ", "head ", "tail ", "cd ", "clear",
        ];
        if line.trim().is_empty() {
            return self.hinter.hint(line, pos, ctx);
        } else {
            for cmd in &cmds {
                if cmd.contains(line) {
                    return Some(cmd.trim_start_matches(line).to_string());
                }
            }
        }
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for DuneHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        // if default {
        //     Borrowed(&self.colored_prompt)
        // } else {
        //     Borrowed(prompt)
        // }
        Borrowed(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for DuneHelper {
    fn validate(&self, _: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

fn readline(prompt: impl ToString, rl: &mut Editor<impl Helper>) -> String {
    loop {
        match rl.readline(&prompt.to_string()) {
            Ok(line) => return line,
            Err(ReadlineError::Interrupted) => {
                return String::new();
            }
            Err(ReadlineError::Eof) => {
                return String::new();
            }
            Err(err) => {
                eprintln!("error: {:?}", err);
            }
        }
    }
}

fn get_os_name(t: &Type) -> String {
    match t {
        Type::Alpine => "alpine",
        Type::Amazon => "amazon",
        Type::Android => "android",
        Type::Arch => "arch",
        Type::CentOS => "centos",
        Type::Debian => "debian",
        Type::Macos => "macos",
        Type::Fedora => "fedora",
        Type::Linux => "linux",
        Type::Manjaro => "manjaro",
        Type::Mint => "mint",
        Type::openSUSE => "opensuse",
        Type::EndeavourOS => "endeavouros",
        Type::OracleLinux => "oraclelinux",
        Type::Pop => "pop",
        Type::Redhat => "redhat",
        Type::RedHatEnterprise => "redhatenterprise",
        Type::Redox => "redox",
        Type::Solus => "solus",
        Type::SUSE => "suse",
        Type::Ubuntu => "ubuntu",
        Type::Windows => "windows",
        Type::Unknown | _ => "unknown",
    }
    .to_string()
}

fn get_os_family(t: &Type) -> String {
    match t {
        Type::Amazon | Type::Android => "android",
        Type::Alpine
        | Type::Arch
        | Type::CentOS
        | Type::Debian
        | Type::Fedora
        | Type::Linux
        | Type::Manjaro
        | Type::Mint
        | Type::openSUSE
        | Type::EndeavourOS
        | Type::OracleLinux
        | Type::Pop
        | Type::Redhat
        | Type::RedHatEnterprise
        | Type::SUSE
        | Type::Ubuntu => "linux",

        Type::Macos | Type::Solus | Type::Redox => "unix",

        Type::Windows => "windows",

        Type::Unknown | _ => "unknown",
    }
    .to_string()
}

const PRELUDE: &'static str = r#"
let CATS = ["
     _
   |\\'/-..--.
  / _ _   ,  ;
 `~=`Y'~_<._./
  <`-....__.'",
"

 |\\__/,|   (`\\
 |_ _  |.--.) )
 ( T   )     /
(((^_(((/(((_/",
"

    \\    /\\
     )  ( ')
    (  /  )
     \\(__)|",
"

      ^~^  ,
     ('Y') )
     /   \\/ 
    (\\|||/)",
"   .       .
   \\`-\"'\"-'/
    } 6 6 {
   =.  Y  ,=
     /^^^\\  .
    /     \\  )
   (  )-(  )/
    \"\"   \"\"",
"

         /\\_/\\
    ____/ o o \\
  /~____  =Y= /
 (______)__m_m)"
];

let map = f -> list -> {
    for item in list {
        f item
    }
};

let reduce = f -> acc -> list -> {
    for item in list {
        let acc = f acc item
    }
    acc
};

let sum = reduce add 0;
let product = reduce mul 1;
let prod = product;

let join = sep -> list -> {
    let out = "";
    let count = 0;
    let size = len list;

    for item in list {
        let count = count + 1;
        let out = out + item;
        if count < size {
            let out = out + sep;
        }
    }

    out
};

let fact = n -> prod (2 to n + 1);


let inc = n -> n + 1;
let dec = n -> n - 1;

let double = n -> n * 2;
let triple = n -> n * 3;

let half = n -> n // 2;
let third = n -> n // 3;
let quarter = n -> n // 4;

let ls = 'lsd;
let cat = 'bat;


let prompt = cwd -> (fmt@blue "(dune) ") + (fmt@green (fmt@italics cwd)) + (fmt@blue "$ ");
let incomplete_prompt = cwd -> ((len cwd) + (len "(dune) ")) * " " + (fmt@yellow "> ");


let about = _ -> {
  clear ();
  echo (
    widget@joiny
      (widget@create "About"
"        Hello, welcome to " + (fmt@blue "⚛Atom⚛ Shell!") + "
      Written by: " + (fmt@magenta "http://adam-mcdaniel.net") + "\n
The goal of atom shell is to make shell scriptingmuch more powerful and formal. Most shells don't\noffer powerful libraries or good enough language\nfeatures to make scripting easy."
50 10)

      (widget@joinx
        (widget@create "Features"
"Atom offers a (very simple)\nwidget system. The entire\nsplash page is made using it!\nIt supports lambda calculus, macros, and traditional\niterative constructs.\n\nAtom's libraries are also\nextremely extensive.\nThere are libraries for:\n * Date and time\n * OS information\n * Shell information\n * File operations\nAnd much more. Atom even has\nlibraries for things like\ncard games and chess!\n\nAnd remember, if atom can do\nall of that, just imagine\nwhat it could do for your\nbuild scripts."
30 26)

        (widget@joiny
          (widget@create "About the Author" "I'm a freshman at\nthe University of\nTennessee🏴󠁵󠁳󠁴󠁮󠁿\nstudying Computer💻\nScience🧪.\n\nI'm extremely \ninterested in\nlanguage design\n& compiler design.\nCheck out my other\nprojects on GitHub:\nadam-mcdaniel" 20 16)
          (widget@create "Cat" (rand@choose CATS) 20 10)
)))
};
"#;

fn parse(input: impl ToString) -> Result<Expression, Error> {
    let input = input.to_string();
    match parse_script(input.as_str(), true) {
        Ok((unparsed, result)) => {
            if !unparsed.is_empty() {
                eprintln!("UNPARSED: `{}`", unparsed);
                return Err(Error::CustomError("incomplete input".to_string()));
            }
            Ok(result)
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(Error::SyntaxError(e)),
        Err(nom::Err::Incomplete(_)) => Err(Error::SyntaxError(SyntaxError::InternalError)),
    }
}

fn check_args_len(
    name: impl ToString,
    args: &Vec<Expression>,
    expected_len: std::ops::RangeFrom<usize>,
) -> Result<(), Error> {
    if expected_len.contains(&args.len()) {
        Ok(())
    } else {
        Err(Error::CustomError(format!(
            "too few arguments to function {}",
            name.to_string()
        )))
    }
}

fn check_exact_args_len(
    name: impl ToString,
    args: &Vec<Expression>,
    expected_len: usize,
) -> Result<(), Error> {
    if args.len() == expected_len {
        Ok(())
    } else {
        Err(Error::CustomError(if args.len() > expected_len {
            format!("too many arguments to function {}", name.to_string())
        } else {
            format!("too few arguments to function {}", name.to_string())
        }))
    }
}

fn repl(
    atomic_rl: Arc<Mutex<Editor<DuneHelper>>>,
    atomic_env: Arc<Mutex<Environment>>,
) -> Result<(), Error> {
    loop {
        let mut env = atomic_env.lock().unwrap();
        let mut rl = atomic_rl.lock().unwrap();
        let mut lines = vec![];
        let cwd = env.get_cwd();
        // let prompt = format!("{}", Expression::Apply(Box::new(env.get("prompt").unwrap()), vec![env.get_cwd().into()]).eval(&mut env)?);

        let prompt = Expression::Apply(
            Box::new(Expression::Symbol(
                if lines.is_empty() {
                    "prompt"
                } else {
                    "incomplete_prompt"
                }
                .to_string(),
            )),
            vec![cwd.clone().into()],
        )
        .eval(&mut env)
        .unwrap_or(format!("{}$", cwd).into())
        .to_string();
        rl.helper_mut()
            .expect("No helper")
            .set_prompt(format!("{}", prompt));
        rl.helper_mut().expect("No helper").update_env(&env);
        let text = readline(prompt, &mut rl);
        let x = text.trim();

        match parse(&text) {
            Ok(expr) => {
                match expr.eval(&mut env) {
                    Ok(Expression::Symbol(name)) => {
                        if let Err(e) =
                            Expression::Apply(Box::new(Expression::Symbol(name)), vec![])
                                .eval(&mut env)
                        {
                            eprintln!("{}", e)
                        }
                    }
                    Ok(Expression::None) => {}
                    Ok(val) => {
                        let _ = Expression::Apply(
                            Box::new(Expression::Symbol("report".to_string())),
                            vec![Expression::Quote(Box::new(val))],
                        )
                        .eval(&mut env);
                    }
                    Err(e) => {
                        eprintln!("{}", e)
                    }
                }
                rl.add_history_entry(text.as_str());
                rl.save_history("history.txt").unwrap();
            }

            Err(e) => {
                if !x.is_empty() {
                    lines.push(x.to_string())
                } else {
                    eprintln!("{}", e);
                }
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let mut env = Environment::new();
    env.define(
        "math",
        b_tree_map! {
            String::from("E")   => std::f64::consts::E.into(),
            String::from("PI")  => std::f64::consts::PI.into(),
            String::from("TAU") => std::f64::consts::TAU.into(),

            String::from("pow") => Expression::builtin("pow", |args, env| {
                check_exact_args_len("pow", &args, 2)?;
                match (args[0].eval(env)?, args[1].eval(env)?) {
                    (Expression::Float(base), Expression::Float(exponent)) => Ok(base.powf(exponent).into()),
                    (Expression::Float(base), Expression::Integer(exponent)) => Ok(base.powf(exponent as f64).into()),
                    (Expression::Integer(base), Expression::Float(exponent)) => Ok((base as f64).powf(exponent).into()),
                    (Expression::Integer(base), Expression::Integer(exponent)) => match base.checked_pow(exponent as u32) {
                        Some(n) => Ok(n.into()),
                        None => Err(Error::CustomError(format!("overflow when raisint {} to the power {}", base, exponent)))
                    },
                    (a, b) => Err(Error::CustomError(format!("cannot raise {} to the power {}", a, b)))
                }
            }, "raise a number to a power"),


            String::from("ln") => Expression::builtin("ln", |args, env| {
                check_exact_args_len("ln", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid natural log argument {}", e)))
                };

                Ok(x.ln().into())
            }, "get the natural log of a number"),


            String::from("log") => Expression::builtin("log", |args, env| {
                check_exact_args_len("log", &args, 2)?;

                let base = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid log base {}", e)))
                };

                let x = match args[1].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid log argument {}", e)))
                };

                Ok(x.log(base).into())
            }, "get the log of a number using a given base"),


            String::from("log2") => Expression::builtin("log2", |args, env| {
                check_exact_args_len("log2", &args, 1)?;

                let base = 2.0;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid log2 argument {}", e)))
                };

                Ok(x.log(base).into())
            }, "get the log base 2 of a number"),

            String::from("log10") => Expression::builtin("log10", |args, env| {
                check_exact_args_len("log10", &args, 1)?;

                let base = 10.0;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid log10 argument {}", e)))
                };

                Ok(x.log(base).into())
            }, "get the log base 10 of a number"),

            String::from("sqrt") => Expression::builtin("sqrt", |args, env| {
                check_exact_args_len("sqrt", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid sqrt argument {}", e)))
                };

                Ok(x.sqrt().into())
            }, "get the square root of a number"),

            String::from("cbrt") => Expression::builtin("cbrt", |args, env| {
                check_exact_args_len("cbrt", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid cbrt argument {}", e)))
                };

                Ok(x.cbrt().into())
            }, "get the cube root of a number"),


            String::from("sin") => Expression::builtin("sin", |args, env| {
                check_exact_args_len("sin", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid sin argument {}", e)))
                };

                Ok(x.sin().into())
            }, "get the sin of a number"),

            String::from("cos") => Expression::builtin("cos", |args, env| {
                check_exact_args_len("cos", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid cos argument {}", e)))
                };

                Ok(x.cos().into())
            }, "get the cosine of a number"),

            String::from("tan") => Expression::builtin("tan", |args, env| {
                check_exact_args_len("tan", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid tan argument {}", e)))
                };

                Ok(x.tan().into())
            }, "get the tangent of a number"),



            String::from("asin") => Expression::builtin("asin", |args, env| {
                check_exact_args_len("asin", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid asin argument {}", e)))
                };

                Ok(x.asin().into())
            }, "get the inverse sin of a number"),

            String::from("acos") => Expression::builtin("acos", |args, env| {
                check_exact_args_len("acos", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid acos argument {}", e)))
                };

                Ok(x.acos().into())
            }, "get the inverse cosine of a number"),

            String::from("atan") => Expression::builtin("atan", |args, env| {
                check_exact_args_len("atan", &args, 1)?;

                let x = match args[0].eval(env)? {
                    Expression::Float(f) => f,
                    Expression::Integer(i) => i as f64,
                    e => return Err(Error::CustomError(format!("invalid atan argument {}", e)))
                };

                Ok(x.atan().into())
            }, "get the inverse tangent of a number"),
        }.into()
    );

    let os = os_info::get();
    let os_type = os.os_type();

    env.define(
        "os",
        b_tree_map! {
            String::from("name") => Expression::from(get_os_name(&os_type)),
            String::from("family") => get_os_family(&os_type).into(),
            String::from("version") => os.version().to_string().into(),
        }
        .into(),
    );

    env.define(
        "widget",
        b_tree_map! {
            String::from("create") => Expression::builtin("create", |args, env| {
                check_exact_args_len("create", &args, 4)?;
                let title = args[0].eval(env)?.to_string();
                let title_len = title.chars().collect::<Vec<char>>().len();

                let text = args[1].eval(env)?.to_string();
                let text_width = match args[2].eval(env)? {
                    Expression::Integer(n) if n > 4 => n,
                    otherwise => return Err(Error::CustomError(format!("expected width argument to be integer greater than 4, but got {}", otherwise)))
                } as usize - 2;
                let widget_height = match args[3].eval(env)? {
                    Expression::Integer(n) if n >= 3 => n,
                    otherwise => return Err(Error::CustomError(format!("expected height argument to be an integer greater than 2, but got {}", otherwise)))
                } as usize;

                if text_width < title_len {
                    Err(Error::CustomError(String::from("width is less than title length")))
                } else {
                    let mut left_border_half = "─".repeat(((text_width - title_len) as f64 / 2.0).round() as usize);
                    let right_border_half = left_border_half.clone();
                    let left_len = left_border_half.chars().collect::<Vec<char>>().len();
                    if (left_len * 2 + title_len + 2) > text_width + 2 {
                        left_border_half.pop();
                    }

                    let mut result = format!("┌{left_side}{}{right_side}┐\n", title, left_side=left_border_half, right_side=right_border_half);
                    let width = result.chars().collect::<Vec<char>>().len() - 1;

                    let mut i = 0;
                    for ch in text.chars() {
                        if i == 0 {
                            result.push(' ');
                            i += 1;
                        }

                        if ch == '\n' {
                            result += &" ".repeat(width-i);
                            i = width;
                        } else {
                            result.push(ch);
                        }

                        if i >= width-1 {
                            result += "\n";
                            i = 0;
                        } else {
                            i += 1;
                        }
                    }


                    result += &" ".repeat(width-i);

                    while result.lines().collect::<Vec<&str>>().len() < widget_height - 1 {
                        result += &(String::from("\n") + &" ".repeat(width));
                    }

                    result += &format!("\n└{left_side}{}{right_side}┘", "─".repeat(title_len), left_side=left_border_half, right_side=right_border_half);

                    Ok(result.into())
                }
            }, "create a text widget"),

            String::from("joinx") => Expression::builtin("joinx", |args, env| {
                check_args_len("joinx", &args, 2..)?;

                let mut string_args = vec![];
                let mut height = 0;
                for (i, arg) in args.iter().enumerate() {
                    match arg.eval(env)? {
                        Expression::String(s) => {
                            let lines = s.lines().map(ToString::to_string).collect::<Vec<String>>();
                            string_args.push(lines.clone());

                            height = string_args[0].len();

                            if height != lines.len() {
                                return Err(Error::CustomError(format!("Heights of horizontally added widgets must be equal, first widget height={}, {}th widget height={}", height, i, lines.len())))
                            }
                        }
                        otherwise => return Err(Error::CustomError(format!("expected string, but got {}", otherwise)))
                    }
                }

                let mut result = String::new();

                for line_n in 0..height {
                    for arg in &string_args {
                        result += &arg[line_n];
                    }
                    result += "\n";
                }

                Ok(result.into())
            }, "join two widgets horizontally"),

            String::from("joiny") => Expression::builtin("joiny", |args, env| {
                check_args_len("joiny", &args, 2..)?;

                let mut string_args = vec![];
                for (i, arg) in args.iter().enumerate() {
                    match arg.eval(env)? {
                        Expression::String(s) => {
                            string_args.push(s.trim().to_string());

                            let width = string_args[0].lines().next().unwrap().chars().collect::<Vec<char>>().len();
                            let this_width = string_args[i].lines().next().unwrap().chars().collect::<Vec<char>>().len();
                            if width != this_width {
                                return Err(Error::CustomError(format!("Widths of vertically added widgets must be equal, first widget height={}, {}th widget height={}", width, i, this_width)))
                            }
                        }
                        otherwise => return Err(Error::CustomError(format!("expected string, but got {}", otherwise)))
                    }
                }

                Ok(string_args.join("\n").into())
            }, "join two widgets vertically")
        }.into()
    );

    env.define(
        "time",
        b_tree_map! {
            String::from("sleep") => Expression::builtin("sleep", |args, env| {
                check_exact_args_len("sleep", &args, 1)?;

                match args[0].eval(env)? {
                    Expression::Float(n)   => sleep(Duration::from_millis(n as u64)),
                    Expression::Integer(n) => sleep(Duration::from_millis(n as u64)),
                    otherwise => return Err(Error::CustomError(format!("expected integer or float, but got {}", otherwise)))
                }

                Ok(Expression::None)
            }, "sleep for a given number of milliseconds")
        }.into()
    );

    env.define(
        "rand",
        b_tree_map! {
            String::from("int") => Expression::builtin("int", |args, env| {
                check_exact_args_len("int", &args, 2)?;
                match (args[0].eval(env)?, args[1].eval(env)?) {
                    (Expression::Integer(l), Expression::Integer(h)) => {
                        let mut rng = thread_rng();
                        let n = Uniform::new(l, h);
                        Ok(Expression::Integer(rng.sample(n)))
                    }
                    (l, h) => Err(Error::CustomError(format!("expected two integers, but got {} and {}", l, h)))
                }
            }, "get a random integer between two numbers (exclusive)"),

            String::from("choose") => Expression::builtin("choose", |args, env| {
                check_exact_args_len("choose", &args, 1)?;
                match args[0].eval(env)? {
                    Expression::List(list) => {
                        let mut rng = thread_rng();
                        let n = Uniform::new(0, list.len());
                        Ok(list[rng.sample(n)].clone())
                    }
                    otherwise => Err(Error::CustomError(format!("expected a list, but got {}", otherwise)))
                }
            }, "choose a random item in a list"),

            String::from("shuffle") => Expression::builtin("shuffle", |args, env| {
                check_exact_args_len("shuffle", &args, 1)?;
                match args[0].eval(env)? {
                    Expression::List(mut list) => {
                        let mut rng = thread_rng();
                        list.shuffle(&mut rng);
                        Ok(list.into())
                    }
                    otherwise => Err(Error::CustomError(format!("expected a list, but got {}", otherwise)))
                }
            }, "shuffle a list randomly"),
        }.into()
    );

    env.define(
        "fs",
        b_tree_map! {
            String::from("exists") => Expression::builtin("exists", |args, env| {
                check_exact_args_len("exists", &args, 1)?;
                let path = PathBuf::from(env.get_cwd());

                Ok(path.join(args[0].eval(env)?.to_string()).exists().into())
            }, "check if a given file path exists"),

            String::from("isdir") => Expression::builtin("isdir", |args, env| {
                check_exact_args_len("isdir", &args, 1)?;
                let path = PathBuf::from(env.get_cwd());

                Ok(path.join(args[0].eval(env)?.to_string()).is_dir().into())
            }, "check if a given path is a directory"),

            String::from("isfile") => Expression::builtin("isfile", |args, env| {
                check_exact_args_len("isfile", &args, 1)?;
                let path = PathBuf::from(env.get_cwd());

                Ok(path.join(args[0].eval(env)?.to_string()).is_file().into())
            }, "check if a given path is a file"),

            String::from("read") => Expression::builtin("read", |args, env| {
                check_exact_args_len("read", &args, 1)?;
                let mut path = PathBuf::from(env.get_cwd());
                let file = args[0].eval(env)?;
                path = path.join(file.to_string());

                match std::fs::read_to_string(path) {
                    Ok(contents) => Ok(contents.into()),
                    Err(_) => Err(Error::CustomError(format!("could not read file {}", file)))
                }
            }, "read a file"),

            String::from("write") => Expression::builtin("write", |args, env| {
                check_exact_args_len("write", &args, 2)?;
                let mut path = PathBuf::from(env.get_cwd());
                let file = args[0].eval(env)?;
                path = path.join(file.to_string());
                match std::fs::write(path, args[1].eval(env)?.to_string()) {
                    Ok(()) => Ok(file),
                    Err(_) => Err(Error::CustomError(format!("could not write to file {}", file)))
                }
            }, "write to a file"),
        }
        .into(),
    );

    env.define(
        "fmt",
        b_tree_map! {
            String::from("bold") => Expression::builtin("bold", |args, env| {
                Ok(format!("\x1b[1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to bold on the console"),

            String::from("faint") => Expression::builtin("faint", |args, env| {
                Ok(format!("\x1b[2m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to italics on the console"),

            String::from("italics") => Expression::builtin("italics", |args, env| {
                Ok(format!("\x1b[3m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to italics on the console"),

            String::from("underline") => Expression::builtin("underline", |args, env| {
                Ok(format!("\x1b[4m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "underline text on the console"),

            String::from("blink") => Expression::builtin("blink", |args, env| {
                Ok(format!("\x1b[5m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "blink text on the console"),

            String::from("invert") => Expression::builtin("invert", |args, env| {
                Ok(format!("\x1b[7m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "invert text on the console"),

            String::from("strike") => Expression::builtin("strike", |args, env| {
                Ok(format!("\x1b[9m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "strike out text on the console"),

            String::from("black") => Expression::builtin("black", |args, env| {
                Ok(format!("\x1b[30;1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to black on the console"),

            String::from("red") => Expression::builtin("red", |args, env| {
                Ok(format!("\x1b[31;1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to red on the console"),

            String::from("green") => Expression::builtin("green", |args, env| {
                Ok(format!("\x1b[32;1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to green on the console"),

            String::from("yellow") => Expression::builtin("yellow", |args, env| {
                Ok(format!("\x1b[33;1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to yellow on the console"),

            String::from("blue") => Expression::builtin("blue", |args, env| {
                Ok(format!("\x1b[34;1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to blue on the console"),

            String::from("magenta") => Expression::builtin("magenta", |args, env| {
                Ok(format!("\x1b[35;1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to magenta on the console"),

            String::from("cyan") => Expression::builtin("cyan", |args, env| {
                Ok(format!("\x1b[35;1m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
            }, "convert text to cyan on the console"),

            String::from("white") => Expression::builtin("white", |args, env| {
                Ok(format!("\x1b[36;1m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
            }, "convert text to white on the console"),

            String::from("dark") => b_tree_map! {
                String::from("black") => Expression::builtin("black", |args, env| {
                    Ok(format!("\x1b[30m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
                }, "convert text to black on the console"),

                String::from("red") => Expression::builtin("red", |args, env| {
                    Ok(format!("\x1b[31m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
                }, "convert text to red on the console"),

                String::from("green") => Expression::builtin("green", |args, env| {
                    Ok(format!("\x1b[32m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
                }, "convert text to green on the console"),

                String::from("yellow") => Expression::builtin("yellow", |args, env| {
                    Ok(format!("\x1b[33m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
                }, "convert text to yellow on the console"),

                String::from("blue") => Expression::builtin("blue", |args, env| {
                    Ok(format!("\x1b[34m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
                }, "convert text to blue on the console"),

                String::from("magenta") => Expression::builtin("magenta", |args, env| {
                    Ok(format!("\x1b[35m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
                }, "convert text to magenta on the console"),

                String::from("cyan") => Expression::builtin("cyan", |args, env| {
                    Ok(format!("\x1b[35m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
                }, "convert text to cyan on the console"),

                String::from("white") => Expression::builtin("white", |args, env| {
                    Ok(format!("\x1b[36m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
                }, "convert text to white on the console"),
            }.into()
        }
        .into(),
    );

    env.define_builtin(
        "add",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? + arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot add {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "add two numbers",
    );

    env.define_builtin(
        "sub",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? - arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot subtract {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "subtract two numbers",
    );

    env.define_builtin(
        "neg",
        |args, env| match args[0].clone().eval(env)? {
            Expression::Integer(n) => Ok(Expression::Integer(-n)),
            Expression::Float(n) => Ok(Expression::Float(-n)),
            x => Err(Error::CustomError(format!("cannot negate {:?}", x))),
        },
        "negate a number",
    );

    env.define_builtin(
        "mul",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? * arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot multiply {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "multiply two numbers",
    );

    env.define_builtin(
        "div",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? / arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot divide {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "divide two numbers",
    );

    env.define_builtin(
        "rem",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? % arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot remainder {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "remainder two numbers",
    );

    env.define_builtin(
        "exit",
        |args, env| {
            if args.is_empty() {
                std::process::exit(0);
            } else if let Expression::Integer(n) = args[0].clone().eval(env)? {
                std::process::exit(n as i32);
            } else {
                Err(Error::CustomError(format!(
                    "expected integer but got `{:?}`",
                    args[0]
                )))
            }
        },
        "exit the shell",
    );
    env.define("quit", env.get("exit").unwrap());

    env.define_builtin(
        "help",
        |args, env| {
            for arg in args {
                match arg.eval(env)? {
                    Expression::Builtin(_, _, help) => {
                        println!("{}", help)
                    }
                    _ => {}
                }
            }
            Ok(Expression::None)
        },
        "read the purpose of builtin functions",
    );

    env.define_builtin(
        "print",
        |args, env| {
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?;
                if i < args.len() - 1 {
                    print!("{} ", x)
                } else {
                    println!("{}", x)
                }
            }

            Ok(Expression::None)
        },
        "print the arguments",
    );
    env.define("echo", env.get("print").unwrap());

    env.define_builtin(
        "range",
        |args, env| {
            if args.len() == 2 {
                match (args[0].clone().eval(env)?, args[1].clone().eval(env)?) {
                    (Expression::Integer(m), Expression::Integer(n)) => Ok(Expression::List(
                        (m..n).map(Expression::Integer).collect::<Vec<Expression>>(),
                    )),
                    _ => Err(Error::CustomError(format!(
                        "Arguments to range must be integers"
                    ))),
                }
            } else {
                Err(Error::CustomError(format!(
                    "Must supply 2 arguments to range"
                )))
            }
        },
        "get a list of integers from (inclusive) one to another (exclusive)",
    );

    env.define_builtin(
        "and",
        |args, env| {
            Ok(Expression::Boolean(
                args.into_iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<Expression>, Error>>()?
                    .iter()
                    .all(|item| item.is_truthy()),
            ))
        },
        "perform a boolean and for a list of truthy values",
    );

    env.define_builtin(
        "or",
        |args, env| {
            Ok(Expression::Boolean(
                args.into_iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<Expression>, Error>>()?
                    .iter()
                    .any(|item| item.is_truthy()),
            ))
        },
        "perform a boolean or for a list of truthy values",
    );

    env.define_builtin(
        "not",
        |args, env| {
            Ok(Expression::Boolean(
                args.into_iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<Expression>, Error>>()?
                    .iter()
                    .all(|item| !item.is_truthy()),
            ))
        },
        "perform a boolean not for one or many truthy values",
    );

    env.define_builtin(
        "eq",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? == args[1].clone().eval(env)?,
            ))
        },
        "compare two values for equality",
    );

    env.define_builtin(
        "neq",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? != args[1].clone().eval(env)?,
            ))
        },
        "compare two values for inequality",
    );

    env.define_builtin(
        "lt",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? < args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "lte",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? <= args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "gt",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? > args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "gte",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? >= args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "index",
        |args, env| {
            let mut val = args[0].eval(env)?;
            for arg in &args[1..] {
                val = val[arg.eval(env)?].clone()
            }
            Ok(val)
        },
        "index a dictionary or list",
    );

    env.define_builtin(
        "len",
        |args, env| match args[0].eval(env)? {
            Expression::Map(m) => Ok(Expression::Integer(m.len() as Int)),
            Expression::List(list) => Ok(Expression::Integer(list.len() as Int)),
            Expression::Symbol(x) | Expression::String(x) => Ok(Expression::Integer(
                x.chars().collect::<Vec<char>>().len() as Int,
            )),
            otherwise => Err(Error::CustomError(format!(
                "cannot get length of {}",
                otherwise
            ))),
        },
        "get the length of an expression",
    );

    env.define_builtin(
        "chars",
        |args, env| match args[0].eval(env)? {
            Expression::Symbol(x) | Expression::String(x) => Ok(Expression::List(
                x.chars()
                    .map(|ch| Expression::String(ch.to_string()))
                    .collect::<Vec<Expression>>(),
            )),
            otherwise => Err(Error::CustomError(format!(
                "cannot get characters of non-string {}",
                otherwise
            ))),
        },
        "get the list of characters for a string or symbol",
    );

    env.define_builtin(
        "eval",
        |args, env| args[0].clone().eval(env)?.eval(env),
        "evaluate an expression",
    );

    env.define_builtin(
        "exec",
        |args, env| {
            if args.is_empty() {
                Err(Error::CustomError(format!("too few arguments")))
            } else {
                Expression::Apply(Box::new(args[0].clone()), args[1..].to_vec()).eval(env)
            }
        },
        "execute a program",
    );

    env.define_builtin(
        "list",
        |args, env| {
            Ok(Expression::List(
                args.into_iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<Expression>, Error>>()?,
            ))
        },
        "create a list from the given arguments",
    );

    if let Some(home_dir) = dirs::home_dir() {
        let home_dir = home_dir.into_os_string().into_string().unwrap();

        env.set_cwd(&home_dir);
        env.define("HOME", Expression::String(home_dir));
    }

    if let Some(desk_dir) = dirs::desktop_dir() {
        env.define(
            "DESK",
            Expression::String(desk_dir.into_os_string().into_string().unwrap()),
        );
    }

    if let Some(docs_dir) = dirs::document_dir() {
        env.define(
            "DOCS",
            Expression::String(docs_dir.into_os_string().into_string().unwrap()),
        );
    }

    if let Some(down_dir) = dirs::download_dir() {
        env.define(
            "DOWN",
            Expression::String(down_dir.into_os_string().into_string().unwrap()),
        );
    }

    env.define_builtin(
        "cd",
        |args, env| match args[0].clone().eval(env)? {
            Expression::Symbol(path) | Expression::String(path) => {
                if let Ok(new_cwd) = dunce::canonicalize(PathBuf::from(env.get_cwd()).join(path)) {
                    env.set_cwd(new_cwd.into_os_string().into_string().unwrap());
                }
                Ok(Expression::None)
            }
            _ => Err(Error::CustomError(format!(
                "expected string, got {:?}",
                args[0]
            ))),
        },
        "change directories",
    );

    env.define_builtin(
        "prompt",
        |_, env| Ok(Expression::String(format!("{}$ ", env.get_cwd()))),
        "default prompt",
    );

    env.define_builtin(
        "incomplete_prompt",
        |_, env| {
            Ok(Expression::String(format!(
                "{}> ",
                " ".repeat(env.get_cwd().len())
            )))
        },
        "default prompt for incomplete commands",
    );

    env.define_builtin(
        "report",
        |args, env| {
            // match args[0].clone().eval(env)? {
            //     Expression::None => {},
            //     other => println!("{:?}", other),
            // }
            println!("{:?}", args[0].clone().eval(env)?);

            Ok(Expression::None)
        },
        "default function for reporting values",
    );

    match parse(PRELUDE) {
        Ok(expr) => {
            let _ = expr.eval(&mut env);
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }

    let config = Config::builder()
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .auto_add_history(false)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();

    let mut rl = Editor::with_config(config);

    let h = DuneHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_string(),
        validator: MatchingBracketValidator::new(),
        env: env.clone(),
    };
    rl.set_helper(Some(h));
    if rl.load_history("history.txt").is_err() {}

    let editor_ref = Arc::new(Mutex::new(rl));
    let editor_ref_copy = editor_ref.clone();

    let env_ref = Arc::new(Mutex::new(env));
    let env_ref_copy = env_ref.clone();

    ctrlc::set_handler(move || {
        repl(editor_ref_copy.clone(), env_ref_copy.clone()).expect("Error in REPL");
    })
    .expect("Error setting Ctrl-C handler");
    repl(editor_ref, env_ref)?;

    Ok(())
}
