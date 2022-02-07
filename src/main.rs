use std::io::{Write, self, BufRead};

use wordle::*;

use termcolor::{
    Color, ColorChoice, ColorSpec, WriteColor, StandardStream,
};

fn print_check(buffer: &mut impl WriteColor, Check(check): &Check) -> io::Result<()> {
    use GuessState::*;
    let mut style: ColorSpec = ColorSpec::new();
    style.set_bold(true);
    for (ch, state) in check {
        let color = match state {
            Right => Color::Green,
            Elsewhere => Color::Yellow,
            Wrong => Color::Red,
        };
        style.set_fg(Some(color));
        buffer.set_color(&style)?;
        write!(buffer, "{ch}")?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let word = Word::from("style");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let stdout = &mut stdout;

    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut lines = lock.lines();

    let mut checks = Vec::new();

    loop {
        stdout.set_color(&ColorSpec::new())?;
        write!(stdout, "Enter your guess: ")?;
        io::stdout().flush()?;

        let guess : Word = match lines.next() {
            Some(result) => result?,
            None => break,
        }.into();

        let check = match word.check(&guess) {
            Some(check) => check,
            None => {
                writeln!(stdout, "Invalid guess. Your guess must contain {} letters.", word.0.len())?;
                continue;
            }
        };

        checks.push(check.clone());

        for check in &checks {
            print_check(stdout, check)?;
            writeln!(stdout)?;
        }
        writeln!(stdout)?;

        if check.all_good() {
            writeln!(stdout, "Congradulations! You won!")?;
            break;
        }
    }
    
    Ok(())
}
