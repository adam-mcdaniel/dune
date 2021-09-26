use dune::{parse_script, VERSION, Environment, Error, Expression, Int, SyntaxError};

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
use os_info::Type;
use chrono::{Local, Timelike, Datelike};

use rand::{distributions::Uniform, seq::SliceRandom, thread_rng, Rng};

use std::{
    borrow::Cow::{self, Borrowed, Owned},
    path::PathBuf,
    env::current_exe,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};



const DEFAULT_PRELUDE: &'static str = r#"


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

let prompt = cwd -> fmt@bold ((fmt@dark@blue "(dune) ") + (fmt@bold (fmt@dark@green cwd)) + (fmt@bold (fmt@dark@blue "$ ")));
let incomplete_prompt = cwd -> ((len cwd) + (len "(dune) ")) * " " + (fmt@bold (fmt@dark@yellow "> "));

let shrew = _ ~> {
"
          _,____ c--.
        /`  \\   ` _^_\\
    `~~~\\  _/---'\\\\  ^
         `~~~     ~~
    ─────────────────────
"
};

let turnedshrew = _ ~> {
"
      .--p_____,_
     /_^_ `   /  `\\
     ^  //'---\\_  /~~~
       ~~     ~~~`
     ──────────────────
"
};

let shrewsay = text -> {
    let title = "Wild Dune Shrew";


    let cols = 22;
    let text = fmt@wrap text cols;
    let text-lines = lines text;
    if (len text-lines) == 1 {
        if (len text) < cols {
            let cols = len text;
        }
    }
    let rows = len text-lines;
    let spacing = 25;

    for line in (lines (widget@create title text cols + 2 rows + 2)) {
        echo " " * spacing line;
    }

    for i in 0 to 2 {
        echo " " * (spacing - i) "╱";
    }

    echo (shrew ());
};

let turnedshrewsay = text -> {
    let title = "Wild Dune Shrew";

    let cols = 27;
    let text = fmt@wrap text cols;
    let text-lines = lines text;
    if (len text-lines) == 1 {
        if (len text) < cols {
            let cols = len text;
        }
    }
    let rows = len text-lines;
    let spacing = 20;

    for line in (lines (widget@create title text cols + 2 rows + 2)) {
        echo " " * spacing line;
    }

    for i in 0 to 2 {
        echo " " * (spacing - i) "╱";
    }

    echo (turnedshrew ());
};


let about = _ ~> {
    echo (
      widget@joiny
        (widget@create "About"
  "          Hello, welcome to " + (fmt@yellow "Dune Shell!") + "
      Written by: " + (fmt@magenta "http://adam-mcdaniel.net") + "
I wrote Dune to be a nice environment for devs while they work! It's a very cozy shell with high customizability, so you can make it how you'd like."
  50 10)
  
        (widget@joinx
          (widget@create "Features"
"Dune has a wide set of
features, it's basically a
full blown language!

It supports several uncommon
features in a shell, such as:
operator overloading,
lambdas, macros, quoted
expressions like Lisp, and
more!

Dune's libraries are very
extensive. There are
libraries for:

☞ A simple widget system🪟
☞ OS information        💽
☞ Randomness            🔀
☞ Basic math, trig, etc.🧮
☞ File system operations📂
☞ Text color and styling📝
☞ Functional programming🔗
☞ Date and time         🕒

And more!"
  30 28)
  
          (widget@joiny
            (widget@create "About the Author" "I'm a sophomore at\nthe University of\nTennessee🏴󠁵󠁳󠁴󠁮󠁿\nstudying Computer💻\nScience🧪.\n\nI'm extremely \ninterested in\nlanguage design\n& compiler design.\nCheck out my other\nprojects on GitHub:\n\nadam-mcdaniel" 20 18)
            (widget@create "Cat" (rand@choose CATS) 20 10)
  )))
};


let welcomebanner = _ ~> {


    let logo = "
        ██████╗ ██╗   ██╗███╗   ██╗███████╗
        ██╔══██╗██║   ██║████╗  ██║██╔════╝
        ██║  ██║██║   ██║██╔██╗ ██║█████╗  
        ██║  ██║██║   ██║██║╚██╗██║██╔══╝  
        ██████╔╝╚██████╔╝██║ ╚████║███████╗
        ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚══════╝
                                     ";

    let logo = "

        ██████╗░██╗░░░██╗███╗░░██╗███████╗
        ██╔══██╗██║░░░██║████╗░██║██╔════╝
        ██║░░██║██║░░░██║██╔██╗██║█████╗░░
        ██║░░██║██║░░░██║██║╚████║██╔══╝░░
        ██████╔╝╚██████╔╝██║░╚███║███████╗
        ╚═════╝░░╚═════╝░╚═╝░░╚══╝╚══════╝
";

    (_ -> {
        let now = time@now ();
        let time-emoji = if now@hour <= 6 "🌃"
            else if now@hour <= 10 "🌅"
            else if now@hour <= 18 "🌤️ "
            else "🌃";
        let date-emoji = if now@month == 1 "⛄"
            else if now@month == 2 "💖"
            else if now@month == 3 "🍀"
            else if now@month == 4 "🌂"
            else if now@month == 5 "🌻"
            else if now@month == 6 "🌞"
            else if now@month == 7 "🌊"
            else if now@month == 8 "📝"
            else if now@month == 9 "🍎"
            else if now@month == 10 "🎃"
            else if now@month == 11 "🍂"
            else if now@month == 12 "🌨️"
            else "📅";
        let zodiac-emoji = if now@month == 1 (if now@day < 20 "🐐" else "🏺")
            else if now@month == 2 (if now@day < 19 "🏺" else "🐟")
            else if now@month == 3 (if now@day < 21 "🐟" else "🐏")
            else if now@month == 4 (if now@day < 20 "🐏" else "🐂")
            else if now@month == 5 (if now@day < 21 "🐂" else "👬")
            else if now@month == 6 (if now@day < 21 "👬" else "🦀")
            else if now@month == 7 (if now@day < 23 "🦀" else "🦁")
            else if now@month == 8 (if now@day < 23 "🦁" else "👩")
            else if now@month == 9 (if now@day < 23 "👩" else "⚖️")
            else if now@month == 10 (if now@day < 23 "⚖️" else "🦂")
            else if now@month == 11 (if now@day < 22 "🦂" else "🏹")
            else if now@month == 12 (if now@day < 22 "🏹" else "🐐")
            else "⭐";
        echo "┌─────────────────Welcome to ...─────────────────┐";
        for ch in (chars logo) {
            print (fmt@bold (if ch == "█" {
               fmt@faint (fmt@red ch)
            } else {
               fmt@faint (fmt@dark@blue ch)
            }));
        }
        echo "";
        echo "        The time is " + (fmt@magenta now@time@str) + " " + time-emoji + " on " + (fmt@cyan now@date@str);
        echo "└────────────────────────────────────────────────┘";
    }) ();

};


let is-leapyear = year -> {
    if year % 4 == 0 && year % 100 != 0 {
        True
    } else if year % 100 == 0 && year % 400 == 0 {
        True
    } else {
        False
    }
};

let days-in-month = month -> year -> {
    if month == 2 {
        28 + (if (is-leapyear year) 1 else 0)
    } else {
        31 - (((month - 1) % 7) % 2)
    }
};

let day-of-week = m -> d -> y -> {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

    if m < 3 {
        let y = y - 1
    }

    (((y + (int(y // 4.0))) - (int(y // 100.0))) + (int(y // 400.0)) + t@(m - 1) + d) % 7
};

let WEEKDAYS = [
	"Sunday",
	"Monday",
	"Tuesday",
	"Wednesday",
	"Thursday",
	"Friday",
	"Saturday"
];

let MONTHS = [
	"January",
	"February",
	"March",
	"April",
	"May",
	"June",
	"July",
	"August",
	"September",
	"October",
	"November",
	"December"
];

let make-calendar = m -> d -> y -> {
    let now = {month: m, day: d, year: y};
    let result = MONTHS@(now@month - 1) + " " + (str now@day) + ", " + (str now@year) + "\n";
	let result = " " * ((28 - (len result)) // 2 + 1) + result;
    let result = result + " Su  Mo  Tu  We  Th  Fr  Sa\n";

    let dof = day-of-week now@month 1 now@year;
    let dim = days-in-month now@month now@year;

    for i in 0 to dof {
        let result = result + "    ";
    }

    for i in 1 to dim + 1 {
        let num = str i;
        if (len num) < 2 {
            let num = (if now@day == i " *" else "  ") + num
        } else {
            let num = (if now@day == i "*" else " ") + num
        }

        let result = result + num + (if (i + dof) % 7 == 0 "\n" else " ")
    }
    widget@create "Calendar" result 30 10
};

let cal = _ ~> {
    (_ -> {
        let now = time@now ();
        make-calendar now@month now@day now@year
    }) ();
};



let welcome = _ ~> {
    welcomebanner ();
    (_ -> {
        let now = time@now ();
        echo (widget@joinx
            (make-calendar now@month now@day now@year)
            (widget@create "Cat" (rand@choose CATS) 20 10));
    }) ();
};


let yesorno = _ -> {
    (input (fmt@blue "(y/n) ")) != "n"
};

let wait = _ -> {
   input (fmt@italics (fmt@blue "(Press enter to continue) "));
};


let intro = _ ~> {

    clear ();
    welcomebanner ();


    shrewsay "Hey there! Is this your first time using Dune?";
    if (yesorno ()) {
        clear ();
        welcomebanner ();
        shrewsay "Then let's get started!";
        wait ();
        
        clear ();
        welcomebanner ();
        about ();
        turnedshrewsay "First off, here's some background information about Dune!";
        wait ();


        clear ();
        welcomebanner ();
        shrewsay "To execute a program in Dune, simply call the program the same way you would in bash or Powershell!\n\n\n$ prog arg1 arg2 ...";
        wait ();
        
        clear ();
        welcomebanner ();
        turnedshrewsay "You can also define macros for Dune, and call them the same way you would a program! (Macros called without arguments are implicitly passed the current working directory as an argument)\n\n\n$ cd ..";
        wait ();
        
        clear ();
        welcomebanner ();
        shrewsay "To define variables (which also act as environment variables), simply use the `let` keyword!\n\n\n$ let x = 5";
        wait ();
        
        clear ();
        welcomebanner ();
        turnedshrewsay "That should be enough to get you started! If you have any questions, just call the `help` macro! To ask for general help, run `help me`!\n\n\n$ help me";
        wait ();
        
        clear ();
        welcomebanner ();
        turnedshrewsay "Good luck! I really hope you enjoy my shell! 😄❤️";
        wait ();

    } else {
        clear ();
        welcomebanner ();
        turnedshrewsay "Oh good! I'll assume you know your way around. To write your own startup script, instead of this default script, write a `.dune-prelude` file in your home directory! Bye!";
        wait ();
    }

    clear ();
    welcome ();
};

intro ();

"#;


fn new_editor(env: &Environment) -> Editor<DuneHelper> {
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
    rl
}


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
        let line = line
            .replace("False", "\x1b[95mFalse\x1b[m\x1b[0m")
            .replace("True", "\x1b[95mTrue\x1b[m\x1b[0m")
            
            .replace("None", "\x1b[91mNone\x1b[m\x1b[0m")
            .replace("()", "\x1b[91m()\x1b[m\x1b[0m")

            .replace("clear ",   "\x1b[94mclear \x1b[m\x1b[0m")
            .replace("echo ",   "\x1b[94mecho \x1b[m\x1b[0m")
            .replace("exit ",   "\x1b[94mexit \x1b[m\x1b[0m")
            .replace("cd ",   "\x1b[94mcd \x1b[m\x1b[0m")
            .replace("rm ",   "\x1b[94mrm \x1b[m\x1b[0m")


            .replace("else ",   "\x1b[94melse \x1b[m\x1b[0m")
            .replace("let ", "\x1b[94mlet \x1b[m\x1b[0m")
            .replace("for ",   "\x1b[94mfor \x1b[m\x1b[0m")
            .replace("if ",   "\x1b[94mif \x1b[m\x1b[0m")
            .replace(" in ",   "\x1b[94m in \x1b[m\x1b[0m")
            .replace(" to ",   "\x1b[94m to \x1b[m\x1b[0m")

            .replace(" == ",   "\x1b[96m == \x1b[m\x1b[0m")
            .replace(" != ",   "\x1b[96m != \x1b[m\x1b[0m")
            .replace(" <= ",   "\x1b[96m <= \x1b[m\x1b[0m")
            .replace(" >= ",   "\x1b[96m >= \x1b[m\x1b[0m")
            .replace(" && ",   "\x1b[96m && \x1b[m\x1b[0m")
            .replace(" || ",   "\x1b[96m || \x1b[m\x1b[0m")

            .replace("@",   "\x1b[96m@\x1b[m\x1b[0m")
            .replace("'",   "\x1b[96m'\x1b[m\x1b[0m")

            .replace("->",   "\x1b[95m->\x1b[m\x1b[0m")
            .replace("~>",   "\x1b[95m~>\x1b[m\x1b[0m")


            .replace(" > ",   "\x1b[96m > \x1b[m\x1b[0m")
            .replace(" < ",   "\x1b[96m < \x1b[m\x1b[0m")

            .replace(" + ",   "\x1b[96m + \x1b[m\x1b[0m")
            .replace(" - ",   "\x1b[96m - \x1b[m\x1b[0m")
            .replace(" * ",   "\x1b[96m * \x1b[m\x1b[0m")
            .replace(" // ",   "\x1b[96m // \x1b[m\x1b[0m")
            ;
        match self.highlighter.highlight(&line, pos) {
            Owned(x) => Owned(x),
            Borrowed(x) => Owned(x.to_owned())
        }
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        if self.highlighter.highlight_char(line, pos) {
            return true;
        }
        let old_line = line;
        let line = line
            .replace("False", "\x1b[95mFalse\x1b[m\x1b[0m")
            .replace("True", "\x1b[95mTrue\x1b[m\x1b[0m")

            .replace("None", "\x1b[91mNone\x1b[m\x1b[0m")
            .replace("()", "\x1b[91m()\x1b[m\x1b[0m")

            .replace("clear ",   "\x1b[94mclear \x1b[m\x1b[0m")
            .replace("echo ",   "\x1b[94mecho \x1b[m\x1b[0m")
            .replace("exit ",   "\x1b[94mexit \x1b[m\x1b[0m")
            .replace("cd ",   "\x1b[94mcd \x1b[m\x1b[0m")
            .replace("rm ",   "\x1b[94mrm \x1b[m\x1b[0m")


            .replace("else ",   "\x1b[94melse\x1b[m\x1b[0m")
            .replace("let ", "\x1b[94mlet \x1b[m\x1b[0m")
            .replace("for ",   "\x1b[94mfor \x1b[m\x1b[0m")
            .replace("if ",   "\x1b[94mif \x1b[m\x1b[0m")
            .replace(" in ",   "\x1b[94m in \x1b[m\x1b[0m")
            .replace(" to ",   "\x1b[94m to \x1b[m\x1b[0m")

            .replace(" == ",   "\x1b[96m == \x1b[m\x1b[0m")
            .replace(" != ",   "\x1b[96m != \x1b[m\x1b[0m")
            .replace(" <= ",   "\x1b[96m <= \x1b[m\x1b[0m")
            .replace(" >= ",   "\x1b[96m >= \x1b[m\x1b[0m")
            .replace(" && ",   "\x1b[96m && \x1b[m\x1b[0m")
            .replace(" || ",   "\x1b[96m || \x1b[m\x1b[0m")

            .replace("@",   "\x1b[96m + \x1b[m\x1b[0m")
            .replace("'",   "\x1b[96m'\x1b[m\x1b[0m")

            .replace("->",   "\x1b[95m->\x1b[m\x1b[0m")
            .replace("~>",   "\x1b[95m~>\x1b[m\x1b[0m")


            .replace(" > ",   "\x1b[96m > \x1b[m\x1b[0m")
            .replace(" < ",   "\x1b[96m < \x1b[m\x1b[0m")

            .replace(" + ",   "\x1b[96m + \x1b[m\x1b[0m")
            .replace(" - ",   "\x1b[96m - \x1b[m\x1b[0m")
            .replace(" * ",   "\x1b[96m * \x1b[m\x1b[0m")
            .replace(" // ",   "\x1b[96m // \x1b[m\x1b[0m")
            ;

        old_line != line
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



fn parse(input: impl ToString) -> Result<Expression, Error> {
    if let Ok(input) = comment::python::strip(input) {
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
    } else {
        Err(Error::CustomError("could not strip comments from command".to_string()))
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
    let mut lines = vec![];
    loop {
        let mut env = atomic_env.lock().unwrap();
        let mut rl = atomic_rl.lock().unwrap();
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
        let line = readline(prompt, &mut rl);
        lines.push(line.clone());
        let text = lines.join("\n");

        match parse(&text) {
            Ok(expr) => {
                rl.add_history_entry(text.as_str());
                rl.save_history("history.txt").unwrap();
                let val = expr.eval(&mut env);
                match val.clone() {
                    Ok(Expression::Symbol(name)) => {
                        if let Err(e) =
                            Expression::Apply(Box::new(Expression::Symbol(name)), vec![])
                                .eval(&mut env)
                        {
                            eprintln!("{}", e)
                        }
                    }
                    Ok(Expression::None) => {}
                    Ok(Expression::Macro(_, _)) => {
                        let _ = Expression::Apply(
                            Box::new(Expression::Symbol("report".to_string())),
                            vec![
                                Expression::Apply(
                                    Box::new(val.unwrap().clone()),
                                    vec![
                                        env.get_cwd().into()
                                    ]
                                )
                            ],
                        ).eval(&mut env);
                    }
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
                lines = vec![];
            }

            Err(e) => {
                if line.is_empty() {
                    eprintln!("{}", e);
                    lines = vec![];
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

            String::from("isodd") => Expression::builtin("isodd", |args, env| {
                check_exact_args_len("odd", &args, 1)?;
                Ok(match args[0].eval(env)? {
                    Expression::Integer(i) => i % 2 == 1,
                    Expression::Float(f) => (f as Int) % 2 == 1,
                    e => return Err(Error::CustomError(format!("invalid isodd argument {}", e)))
                }.into())
            }, "is a number odd?"),

            String::from("iseven") => Expression::builtin("iseven", |args, env| {
                check_exact_args_len("even", &args, 1)?;
                Ok(match args[0].eval(env)? {
                    Expression::Integer(i) => i % 2 == 0,
                    Expression::Float(f) => (f as Int) % 2 == 0,
                    e => return Err(Error::CustomError(format!("invalid iseven argument {}", e)))
                }.into())
            }, "is a number even?"),

            String::from("pow") => Expression::builtin("pow", |args, env| {
                check_exact_args_len("pow", &args, 2)?;
                match (args[0].eval(env)?, args[1].eval(env)?) {
                    (Expression::Float(base), Expression::Float(exponent)) => Ok(base.powf(exponent).into()),
                    (Expression::Float(base), Expression::Integer(exponent)) => Ok(base.powf(exponent as f64).into()),
                    (Expression::Integer(base), Expression::Float(exponent)) => Ok((base as f64).powf(exponent).into()),
                    (Expression::Integer(base), Expression::Integer(exponent)) => match base.checked_pow(exponent as u32) {
                        Some(n) => Ok(n.into()),
                        None => Err(Error::CustomError(format!("overflow when raising int {} to the power {}", base, exponent)))
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
        "shell",
        b_tree_map! {
            String::from("author") => Expression::String("Adam McDaniel (adam-mcdaniel.net)".to_string()),
            String::from("version") => Expression::String(VERSION.to_string()),
            String::from("path") => {
                if let Ok(path) = current_exe() {
                    Expression::String(path.to_str().unwrap().to_string())
                } else {
                    Expression::None
                }
            }
        }
        .into(),
    );

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

                let text_width = match args[2].eval(env)? {
                    Expression::Integer(n) if n > 4 => n,
                    otherwise => return Err(Error::CustomError(format!("expected width argument to be integer greater than 4, but got {}", otherwise)))
                } as usize - 2;

                let text = textwrap::fill(&args[1].eval(env)?.to_string(), text_width);
                
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

                    let mut lines = 1;
                    let mut i = 0;
                    for ch in text.chars() {
                        if i == 0 {
                            result.push(' ');
                            i += 1;
                        }

                        if ch == '\n' {
                            lines += 1;
                            result += &" ".repeat(width-i);
                            i = width;
                        } else {
                            result.push(ch);
                        }

                        if lines == widget_height - 1 {
                            break
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
            }, "sleep for a given number of milliseconds"),
            String::from("now") => Expression::builtin("now", |_, _| {
                let now = Local::now();

                Ok(Expression::Map(b_tree_map! {
                    String::from("year") => Expression::Integer(now.year() as i64),
                    String::from("month") => Expression::Integer(now.month() as i64),
                    String::from("day") => Expression::Integer(now.day() as i64),
                    String::from("hour") => Expression::Integer(now.hour() as i64),
                    String::from("time") => Expression::Map(b_tree_map! {
                        String::from("str") => Expression::String(now.time().format("%-I:%M %p").to_string()),
                    }.into()),
                    String::from("date") => Expression::Map(b_tree_map! {
                        String::from("str") => Expression::String(now.format("%D").to_string()),
                    }.into()),
                }))
            }, "get information about the current time")
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

    let mut dir_tree = b_tree_map! {};

    if let Some(home_dir) = dirs::home_dir() {
        let home_dir = home_dir.into_os_string().into_string().unwrap();
        env.set_cwd(&home_dir);

        dir_tree.insert("home".to_string(), Expression::from(home_dir.clone()));
        env.define("HOME", Expression::String(home_dir));
    }

    if let Some(desk_dir) = dirs::desktop_dir() {
        let desk_dir = desk_dir.into_os_string().into_string().unwrap();
        dir_tree.insert("desk".to_string(), desk_dir.clone().into());
        env.define(
            "DESK",
            Expression::String(desk_dir),
        );
    }

    if let Some(docs_dir) = dirs::document_dir() {
        let docs_dir = docs_dir.into_os_string().into_string().unwrap();
        dir_tree.insert("docs".to_string(), docs_dir.clone().into());
        env.define(
            "DOCS",
            Expression::String(docs_dir),
        );
    }

    if let Some(down_dir) = dirs::download_dir() {
        let down_dir = down_dir.into_os_string().into_string().unwrap();
        dir_tree.insert("down".to_string(), down_dir.clone().into());
        env.define(
            "DOWN",
            Expression::String(down_dir),
        );
    }


    env.define(
        "fs",
        b_tree_map! {
            String::from("dirs") => dir_tree.into(),
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
                    Ok(()) => Ok(Expression::None),
                    Err(_) => Err(Error::CustomError(format!("could not write to file {}", file)))
                }
            }, "write to a file"),
        }
        .into(),
    );


    env.define(
        "fn",
        b_tree_map! {
            String::from("map") => Expression::builtin("map", |args, env| {
                if !(1..=2).contains(&args.len()) {
                    return Err(Error::CustomError(if args.len() > 2 {
                        "too many arguments to function map"
                    } else {
                        "too few arguments to function map"
                    }.to_string()))
                }

                if args.len() == 1 {
                    Expression::Apply(
                        Box::new(parse("f -> list -> for item in list {f item}")?),
                        args.clone()
                    ).eval(env)
                } else if let Expression::List(list) = args[1].eval(env)? {
                    let f = args[0].eval(env)?;
                    let mut result = vec![];
                    for item in list {
                        result.push(Expression::Apply(
                            Box::new(f.clone()),
                            vec![item]
                        ).eval(env)?)
                    }
                    Ok(result.into())
                } else {
                    Err(Error::CustomError(format!("invalid arguments to map: {}", Expression::from(args))))
                }
            }, "map a function over a list of values"),

            String::from("filter") => Expression::builtin("filter", |args, env| {
                if !(1..=2).contains(&args.len()) {
                    return Err(Error::CustomError(if args.len() > 2 {
                        "too many arguments to function filter"
                    } else {
                        "too few arguments to function filter"
                    }.to_string()))
                }

                if args.len() == 1 {
                    Expression::Apply(
                        Box::new(parse("f -> list -> { let result = []; for item in list { if (f item) { let result = result + item }} result}")?),
                        args.clone()
                    ).eval(env)
                } else if let Expression::List(list) = args[1].eval(env)? {
                    let f = args[0].eval(env)?;
                    let mut result = vec![];
                    for item in list {
                        if Expression::Apply(
                            Box::new(f.clone()),
                            vec![item.clone()]
                        ).eval(env)?.is_truthy() {
                            result.push(item)
                        }
                    }
                    Ok(result.into())
                } else {
                    Err(Error::CustomError(format!("invalid arguments to filter: {}", Expression::from(args))))
                }
            }, "filter a list of values with a condition function"),

            String::from("reduce") => Expression::builtin("reduce", |args, env| {
                if !(1..=3).contains(&args.len()) {
                    return Err(Error::CustomError(if args.len() > 3 {
                        "too many arguments to function reduce"
                    } else {
                        "too few arguments to function reduce"
                    }.to_string()))
                }

                if args.len() < 3 {
                    Expression::Apply(
                        Box::new(parse("f -> acc -> list -> { for item in list { let acc = f acc item } acc }")?),
                        args.clone()
                    ).eval(env)
                } else if let Expression::List(list) = args[2].eval(env)? {
                    let f = args[0].eval(env)?;
                    let mut acc = args[1].eval(env)?;
                    for item in list {
                        acc = Expression::Apply(
                            Box::new(f.clone()),
                            vec![acc, item]
                        ).eval(env)?
                    }
                    Ok(acc)
                } else {
                    Err(Error::CustomError(format!("invalid arguments to reduce: {}", Expression::from(args))))
                }
            }, "reduce a function over a list of values")
        }
        .into(),
    );

    env.define(
	"console",
	b_tree_map! {
		String::from("write") => Expression::builtin("write", |args, env| {
			check_exact_args_len("write", &args, 3)?;
        	        print!("\x1b[s\x1b[{line};{column}H\x1b[{line};{column}f{content}\x1b[u",
				line=args[1].eval(env)?,
				column=args[0].eval(env)?,
				content=args[2].eval(env)?
			);
			Ok(Expression::None)
	        }, "write text to a specific position in the console"),
	}.into()
    );

    env.define(
        "fmt",
        b_tree_map! {
            String::from("wrap") => Expression::builtin("wrap", |args, env| {
                check_exact_args_len("wrap", &args, 2)?;
                match args[1].eval(env)? {
                    Expression::Integer(columns) => Ok(textwrap::fill(&args[0].eval(env)?.to_string(), columns as usize).into()),
                    otherwise => Err(Error::CustomError(format!("expected number of columns in wrap, but got {}", otherwise)))
                }
            }, "wrap text such that it fits in a specific number of columns"),

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
                Ok(format!("\x1b[90m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to black on the console"),

            String::from("red") => Expression::builtin("red", |args, env| {
                Ok(format!("\x1b[91m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to red on the console"),

            String::from("green") => Expression::builtin("green", |args, env| {
                Ok(format!("\x1b[92m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to green on the console"),

            String::from("yellow") => Expression::builtin("yellow", |args, env| {
                Ok(format!("\x1b[93m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to yellow on the console"),

            String::from("blue") => Expression::builtin("blue", |args, env| {
                Ok(format!("\x1b[94m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to blue on the console"),

            String::from("magenta") => Expression::builtin("magenta", |args, env| {
                Ok(format!("\x1b[95m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to magenta on the console"),

            String::from("cyan") => Expression::builtin("cyan", |args, env| {
                Ok(format!("\x1b[96m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to cyan on the console"),

            String::from("white") => Expression::builtin("white", |args, env| {
                Ok(format!("\x1b[97m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
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
                    Ok(format!("\x1b[36m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
                }, "convert text to cyan on the console"),

                String::from("white") => Expression::builtin("white", |args, env| {
                    Ok(format!("\x1b[37m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
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
                match &arg {
                    Expression::Symbol(name) if name == "me" => {
                        println!("Hello, welcome to Dune's help macro!

To find information on various topics,
run `help` with the following arguments!
1. `builtin`: to learn about the builtin functions
2. `lib`: to find out about the various builtin libraries
3. `syntax`: to find out about the syntax of Dune
4. `types`: to find out about the various types Dune supports
5. `scripting`: to learn about scripting in Dune
6. `prelude`: to learn about the prelude

You can also call `help` on any builtin function:

$ help echo");
                    }
                    Expression::Symbol(name) if name == "prelude" => {
                        println!("Hello, welcome to Dune's help macro!

The `prelude` is the startup file that Dune runs before executing
interactive mode. This file is stored at `.dune-prelude` in your home
directory. To do anything on Dune's startup, add it to the prelude!");
                    }
                    Expression::Symbol(name) if name == "types" => {
                        println!("Hello, welcome to Dune's help macro!

Dune has the following types in its typesystem:
1. `Integer`: a signed integer
2. `Float`: a floating point number
3. `String`: a string
4. `Boolean`: a boolean
5. `None`: a null value
6. `List`: a list of expressions
7. `Map`: a table of expressions
8. `Lambda`: a function
9. `Macro`: a macro (exactly like a function, but executes within the current scope)
10. `Builtin`: a builtin function");
                    }
                    Expression::Symbol(name) if name == "scripting" => {
                        println!("Hello, welcome to Dune's help macro!

Dune has two modes: interactive, and scripting.
In interactive mode, commands are interpreted in the following way:
1. If the command is an expression, it is evaluated.
2. If the result of the evaluation is an undefined symbol,
   Dune executes the program with that name.
3. If the result of the evaluation is the application of undefined symbol,
   Dune executes the program with that name, and gives it the arguments
   of the application.
4. If the result of the evaluation is a macro, Dune executes the macro
   with the argument of the current working directory.

In scripting mode, you MUST pass arguments to macros and programs explicitly.
For example, if you want to run the program `ls` with no arguments, you must
call it like so:

```
# Pass `None` to `ls`
ls ();
```

All statements in the script are also separated by semicolons.

```
echo \"Hmm!\";
if True {{
    echo \"True is True!\";
    # The last expression in a block statement does not need semicolons
    echo \"Hello, world!\"
}}
for i in 0 to 10 {{
    echo i
}}
# The last statement in a script does not require a semicolon either
echo \"Wow!\"
```");
                    }
                    Expression::Symbol(name) if name == "builtin" => {
                        println!("Hello, welcome to Dune's help macro!

Dune offers the following builtin functions:

1. `echo`: prints the given arguments to the console with a newline
2. `println`: identical to `echo`
3. `print`: prints the given arguments to the console without a newline
4. `help`: prints this message
5. `exit`: exits the shell
6. `quit`: identical to `exit`
7. `neg`: negates a number.
8. `add`: adds two numbers, strings, lists, etc.
9. `sub`: subtracts two numbers.
10. `div`: divides two numbers.
11. `mul`: multiplies two numbers, a number and a string, or a number and a list.
12. `mod`: calculates the remainder of two numbers.
13. `input`: reads input from the console with a prompt and returns it as a string.
14. `range`: returns a list of integers from the given start to the given end.
15. `len`: returns the length of a list, string, or dictionary.
16. `insert`: insert an item into a list or dictionary with a given key or position.
17. `remove`: remove an item from a list or dictionary with a given key or position.
18. `index`: returns the item at an index in a list or dictionary.
19. `chars`: returns a list of characters from a string.
20. `lines`: returns a list of lines from a string.
21. `eval`: evaluates a quoted Dune expression.
22. `cd`: changes the current working directory in the current scope.
23. `prompt`: returns the prompt as a string given the current working directory.
24. `incomplete_prompt`: returns the prompt for incomplete expressions as a string given the current working directory.
25. `report`: prints the result of a user-entered expression to the console.
26. `and`: returns the logical and of two expressions.
27. `or`: returns the logical or of two expressions.
28. `not`: returns the logical not of a boolean expression.
29. `eq`: returns true if two expressions are equal.
30. `neq`: returns true if two expressions are not equal.
31. `lt`: returns true if the first expression is less than the second.
32. `gt`: returns true if the first expression is greater than the second.
33. `lte`: returns true if the first expression is less than or equal to the second.
34. `gte`: returns true if the first expression is greater than or equal to the second.
35. `str`: returns the string representation of an expression.");
                    }
                    Expression::Symbol(name) if name == "lib" => {
                        println!("Hello, welcome to Dune's help macro!

Dune offers the following builtin libraries:
1. `math`: a library with several math helper functions.
2. `time`: a library with time related functions.
3. `rand`: a library with random number generation functions.
4. `os`: a library with operating system related functions.
5. `fs`: a library with file system related functions.
6. `fn`: a library with functional programming constructs.
7. `fmt`: a library with color, formatting, and other text functions.
8. `widget`: a library for creating widgets on the console.

To see all the different functions and constants for each library,
simply print the library itself!

$ echo math");
                    }
                    Expression::Symbol(name) if name == "syntax" => {
                        println!("Hello, welcome to Dune's help macro!

Dune has a very simple syntax.
To apply functions, macros, or programs to arguments, simply juxtapose them!

$ echo 1 2 + 3

To write anonymous functions and macros, use the arrow syntax:

$ # an anonymous incrementing function
$ x -> x + 1
$ # an anonymous incrementing macro
$ # (macros are just like functions,
$ # but they are executed within the current scope)
$ x ~> x + 1
$
$ let identity = x -> x
$
$ # an anonymous function that returns the sum of two numbers
$ x -> y -> {{
>    echo \"your numbers are \" x \"and\" y
>    x + y
> }}

To make lists, use the `[]` or the `to` syntax:

$ [1, 2, 3, 2 + 2, \"testing!\"]
$ # lists are zero indexed
$ echo [1, 2, 3]@0
$ # lists can also be made using the `to` syntax
$ echo 0 to 5

To make dictionaries, use the `{{}}` syntax:

$ let origin = {{x: 0, y: 0}}
$ # use the `@` syntax to index a list or dictionary
$ echo origin@x origin@y

To write an expression that is the result of many statements, use the following syntax:

$ let x = {{
>     let y = 1;
>     let z = 2;
>     y + z
> }}

To write math expressions, use the following operators:

$ # addition
$ x + y
$ # subtraction
$ x - y
$ # multiplication
$ x + y
$ # division
$ x // y
$ # remainder
$ x % y
$ # logical and
$ x && y
$ # logical or
$ x || y
$ # logical not
$ !x

Dune also supports if statements and for loops.

$ if True 1 else if False 2 else 3
$ if x > y {{
>     echo \"x is greater than y\"
> }} else {{
>     echo \"x is not greater than y\"
> }}
$
$ for item in [1, 2, 3, 4] {{
>     echo item
> }}
$ for x in 0 to 5 {{
>     echo x
> }}

If you're a fan of Lisp, you can also try quoting expressions!

$ # when evaluated, a quoted expression returns its expression
$ let expression = '(x + y)
$ let x = 5
$ let y = 6
$ # this will evaluate the expression stored in `expression`
$ echo (eval expression)
$
$ # make `cat` an alias for the program `bat`
$ let cat = 'bat

");
                    }
                    otherwise => {
                        if let Expression::Builtin(_, _, help) = otherwise.eval(env)? {
                            println!("{}", help)
                        }
                    }
                }
            }
            Ok(Expression::None)
        },
        "run `help me`",
    );

    env.define_builtin(
        "print",
        |args, env| {
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?;
                if i < args.len() - 1 {
                    print!("{} ", x)
                } else {
                    print!("{}", x)
                }
            }

            Ok(Expression::None)
        },
        "print the arguments without a newline",
    );

    env.define_builtin(
        "println",
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
        "print the arguments and a newline",
    );
    env.define("echo", env.get("println").unwrap());

    env.define_builtin(
        "input",
        |args, env| {
            let mut prompt = String::new();
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?;
                if i < args.len() - 1 {
                    prompt += &format!("{} ", x)
                } else {
                    prompt += &format!("{}", x)
                }
            }
            let mut rl = new_editor(env);
            Ok(Expression::String(readline(&prompt, &mut rl)))
        },
        "get user input",
    );

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
                args[0].eval(env)? == args[1].eval(env)?,
            ))
        },
        "compare two values for equality",
    );

    env.define_builtin(
        "neq",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].eval(env)? != args[1].eval(env)?,
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
                val = match arg {
                    Expression::Integer(_) | Expression::Symbol(_) => {
                        &val[arg.clone()]
                    }
                    otherwise => {
                        &val[otherwise.eval(env)?]
                    }
                }.clone()
            }
            Ok(val)
        },
        "index a dictionary or list",
    );

    env.define_builtin(
        "str",
        |args, env| {
            Ok(Expression::String(args[0].eval(env)?.to_string()))
        },
        "format an expression to a string",
    );

    env.define_builtin(
        "int",
        |args, env| {
            match args[0].eval(env)? {
                Expression::Integer(x) => Ok(Expression::Integer(x)),
                Expression::Float(x) => Ok(Expression::Integer(x as Int)),
                Expression::String(x) => if let Ok(n) = x.parse::<Int>() {
                    Ok(Expression::Integer(n))
                } else {
                    Err(Error::CustomError(format!("could not convert {:?} to an integer", x)))
                },
                otherwise => Err(Error::CustomError(format!("could not convert {:?} to an integer", otherwise)))
            }
        },
        "format an expression to a string",
    );

    env.define_builtin(
        "insert",
        |args, env| {
            check_exact_args_len("insert", &args, 3)?;
            let mut arr = args[0].eval(env)?;
            let idx = args[1].eval(env)?;
            let val = args[2].eval(env)?;
            match (&mut arr, &idx) {
                (Expression::Map(exprs), Expression::String(key)) => {
                    exprs.insert(key.clone(), val);
                }
                (Expression::List(exprs), Expression::Integer(i)) => {
                    if *i as usize <= exprs.len() {
                        exprs.insert(*i as usize, val);
                    } else {
                        return Err(Error::CustomError(format!("index {} out of bounds for {:?}", idx, arr)))
                    }
                }
                (Expression::String(s), Expression::Integer(i)) => {
                    if *i as usize <= s.len() {
                        s.insert_str(*i as usize, &val.to_string());
                    } else {
                        return Err(Error::CustomError(format!("index {} out of bounds for {:?}", idx, arr)))
                    }
                }
                _ => return Err(Error::CustomError(format!("cannot insert {:?} into {:?} with index {:?}", val, arr, idx)))
            }

            Ok(arr)
        },
        "insert an item a dictionary or list",
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
        "lines",
        |args, env| match args[0].eval(env)? {
            Expression::String(x) => Ok(Expression::List(
                x.lines()
                    .map(|ch| Expression::String(ch.to_string()))
                    .collect::<Vec<Expression>>(),
            )),
            otherwise => Err(Error::CustomError(format!(
                "cannot get lines of non-string {}",
                otherwise
            ))),
        },
        "get the list of lines in a string",
    );

    env.define_builtin(
        "eval",
        |args, env| args[0].clone().eval(env)?.eval(env),
        "evaluate an expression",
    );

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

    // env.define_builtin(
    //     "prompt",
    //     |_, env| Ok(Expression::String(format!("{}$ ", env.get_cwd()))),
    //     "default prompt",
    // );
    
    // env.define_builtin(
    //     "incomplete_prompt",
    //     |_, env| {
    //         Ok(Expression::String(format!(
    //             "{}> ",
    //             " ".repeat(env.get_cwd().len())
    //         )))
    //     },
    //     "default prompt for incomplete commands",
    // );
    // let prompt = cwd -> fmt@bold ((fmt@dark@blue "(dune) ") + (fmt@bold (fmt@dark@green cwd)) + (fmt@bold (fmt@dark@blue "$ ")));
    // let incomplete_prompt = cwd -> ((len cwd) + (len "(dune) ")) * " " + (fmt@bold (fmt@dark@yellow "> "));
    
    parse(r#"let prompt = cwd -> fmt@bold ((fmt@dark@blue "(dune) ") + (fmt@bold (fmt@dark@green cwd)) + (fmt@bold (fmt@dark@blue "$ ")))"#)?.eval(&mut env)?;
    parse(r#"let incomplete_prompt = cwd -> ((len cwd) + (len "(dune) ")) * " " + (fmt@bold (fmt@dark@yellow "> "));"#)?.eval(&mut env)?;

    env.define_builtin(
        "report",
        |args, env| {
            let val = args[0].eval(env)?;
            match val {
                Expression::Map(_) => println!("{}", val),
                Expression::String(s) => println!("{}", s),
                Expression::None => {},
                otherwise => println!("{:?}", otherwise)
            }

            Ok(Expression::None)
        },
        "default function for reporting values",
    );


    if let Some(home_dir) = dirs::home_dir() {
        let prelude_path = home_dir.join(".dune-prelude");
        match std::fs::read_to_string(&prelude_path) {
            Ok(prelude) => match parse(&prelude) {
                Ok(expr) => {
                    if let Err(e) = expr.eval(&mut env) {
                        eprintln!("error while running {:?}: {}", prelude_path, e)
                    }
                }
                Err(e) => {
                    eprintln!("error while running {:?}: {}", prelude_path, e)
                }
            }
            Err(_) => {
                match parse(DEFAULT_PRELUDE) {
                    Ok(expr) => {
                        if let Err(e) = expr.eval(&mut env) {
                            eprintln!("error while running default prelude: {}", e)
                        }
                    }
                    Err(e) => {
                        eprintln!("error while running default prelude: {}", e)
                    }
                }
            }
        }
    }

    let mut rl = new_editor(&env);
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
