#[macro_use]
extern crate prettytable;
extern crate term;

use prettytable::Table;

use std::io;
use std::io::Write;
use std::str::FromStr;

const CROSS: &'static str = "X";
const EMPTY: &'static str = " ";
const ROUND: &'static str = "O";

fn main() {
    let mut table = table![
        [EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY]
    ];
    let mut height = table.printstd();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut current = CROSS;
    let mut terminal = term::stdout().unwrap();
    loop {
        let mut line = String::new();
        print!("{} plays > ", current);
        height += 1;
        stdout.flush().unwrap();
        stdin.read_line(&mut line).expect("Cannot read input");
        let i = match usize::from_str(line.trim()) {
            Ok(i) => i,
            _ => {
                println!("Bad input");
                height += 1;
                continue;
            }
        };
        if i < 1 || i > 9 {
            println!("Bad input, should be between 1 and 9");
            height += 1;
            continue;
        }
        let x = (i - 1) % 3;
        let y = (i - 1) / 3;
        {
            let row = table.get_mut_row(y).unwrap();
            if row.get_cell(x).unwrap().to_string() != EMPTY {
                println!("There's already someone there");
                height += 1;
                continue;
            }
            row.set_cell(cell!(current), x).unwrap();
        }
        for _ in 0..height {
            terminal.cursor_up().unwrap();
            terminal.delete_line().unwrap();
        }
        height = table.printstd();
        if check(&table) {
            return;
        }
        if current == CROSS {
            current = ROUND;
        } else {
            current = CROSS;
        }
    }
}

fn get(table: &Table, x: usize, y: usize) -> String {
    match table.get_row(y) {
        Some(r) => match r.get_cell(x) {
            Some(c) => c.to_string(),
            _ => EMPTY.to_string(),
        },
        _ => EMPTY.to_string(),
    }
}

fn is(table: &Table, s: &str, x: usize, y: usize) -> bool {
    get(table, x, y).as_str() == s
}

fn check(table: &Table) -> bool {
    let mut full = true;
    for y in 0..3 {
        for x in 0..3 {
            if is(table, EMPTY, x, y) {
                full = false;
                continue;
            }
            let current = get(table, x, y);
            let c = current.as_str();
            if is(table, c, x + 1, y) && is(table, c, x + 2, y)
                || is(table, c, x + 1, y + 1) && is(table, c, x + 2, y + 2)
                || x >= 2 && is(table, c, x - 1, y + 1) && is(table, c, x - 2, y + 2)
                || is(table, c, x, y + 1) && is(table, c, x, y + 2)
            {
                println!("Game is over. {} is the winner", current);
                return true;
            }
        }
    }
    if full {
        println!("Game is over. It's a draw");
    }
    full
}
