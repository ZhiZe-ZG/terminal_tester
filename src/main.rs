use crossterm::{
    cursor::{self, Hide},
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute, queue,
    style::{
        self, Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor, Stylize,
    },
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};
use std::io::{stdout, Write};
use std::{
    thread,
    time::{Duration, SystemTime},
};

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture,Hide)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    // time
    let mut start_time = SystemTime::now();
    let start_time2 = SystemTime::now();
    // draw box
    draw_box(&mut stdout, 0, 0, 48, 160)?;
    // write some thing
    // color list
    let colors = [
        Color::White,
        Color::Grey,
        Color::Cyan,
        Color::DarkCyan,
        Color::Magenta,
        Color::DarkMagenta,
        Color::Blue,
        Color::DarkBlue,
        Color::Yellow,
        Color::DarkYellow,
        Color::Green,
        Color::DarkGreen,
        Color::Red,
        Color::DarkRed,
        Color::DarkGrey,
        Color::Black,
    ];
    let mut x = 1;
    let mut y = 1;
    for color in colors {
        queue!(
            stdout,
            cursor::MoveTo(x, y),
            Print("I shot ".with(color)),
            Print("an arrow ".with(color).underlined()),
            Print("into the air, ".with(color).bold()),
            Print("it fell ".with(color).dim()),
            Print("to earth, ".with(color).italic()),
            Print("I knew ".with(color).slow_blink()),
            Print("not ".with(color).rapid_blink()),
            Print("whe".with(color).reverse()),
            Print("re.".with(color).reverse().dim())
        )?;
        y = y + 1;
    }
    x = 70;
    y = 1;
    for color in colors {
        queue!(
            stdout,
            cursor::MoveTo(x, y),
            Print("I shot ".on(Color::White).with(color)),
            Print("an arrow ".on(Color::White).with(color).underlined()),
            Print("into the air, ".on(Color::White).with(color).bold()),
            Print("it fell ".on(Color::White).with(color).dim()),
            Print("to earth, ".on(Color::White).with(color).italic()),
            Print("I knew ".on(Color::White).with(color).slow_blink()),
            Print("not ".on(Color::White).with(color).rapid_blink()),
            Print("whe".on(Color::White).with(color).reverse()),
            Print("re.".on(Color::White).with(color).reverse().dim())
        )?;
        y = y + 1;
    }
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print("0我朝天空射箭，不知箭落何方。2")
    )?;
    y = y + 1;
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print(
            "0我朝天空射箭，不知箭落何方。2"
                .with(Color::Rgb {
                    r: 17,
                    g: 228,
                    b: 108
                })
                .reverse()
        )
    )?;
    y = y + 1;
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print(
            "0我朝天空射箭，不知箭落何方。2"
                .with(Color::Rgb {
                    r: 255,
                    g: 153,
                    b: 0
                })
                .reverse()
        )
    )?;
    y = y + 1;

    stdout.flush()?;
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print(
            format!(
                "Time:{}",
                SystemTime::now()
                    .duration_since(start_time)
                    .unwrap()
                    .as_secs_f64()
            )
            .with(Color::Rgb {
                r: 255,
                g: 153,
                b: 0
            })
        )
    )?;
    stdout.flush()?;

    // wait loop
    let mut flag = false;
    let sh_x = 80;
    let sh_y = 30;
    let mut sh_flag = false;
    let mut sh_time = SystemTime::now();
    let mut move_x = 80;
    let mut move_y = 31;
    loop {
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                } else if let KeyCode::Char('a') = key.code {
                    flag = if flag { false } else { true };
                }else if let KeyCode::Char('h') = key.code {
                    move_x = if move_x-1>=0 {move_x-1} else{0};
                }else if let KeyCode::Char('l') = key.code {
                    move_x = if move_x+1<160 {move_x+1} else{159};
                }
            }
            // thread::sleep_ms(2000);
        }

        if flag {
            start_time = SystemTime::now();
            // draw_box(&mut stdout, 0, 0, 48, 160)?;
            execute!(
                stdout,
                cursor::MoveTo(x, y),
                Print(
                    format!(
                        "Time:{}",
                        SystemTime::now()
                            .duration_since(start_time)
                            .unwrap()
                            .as_secs_f64(),
                    )
                    .with(Color::Rgb {
                        r: 255,
                        g: 153,
                        b: 0
                    })
                ),
                cursor::MoveTo(x, y+1),
                Print(
                    format!(
                        "now {}",
                        SystemTime::now()
                            .duration_since(start_time2)
                            .unwrap()
                            .as_secs_f64(),
                    )
                    .with(Color::Rgb {
                        r: 255,
                        g: 153,
                        b: 0
                    })
                ),
                cursor::MoveTo(x, y+2),
                Print(
                    format!(
                        "rate {}",
                        1.0 / (SystemTime::now()
                            .duration_since(start_time)
                            .unwrap()
                            .as_secs_f64())
                    )
                    .with(Color::Rgb {
                        r: 255,
                        g: 153,
                        b: 0
                    })
                )
            )?;
            stdout.flush()?;
            if SystemTime::now()
                .duration_since(sh_time)
                .unwrap()
                .as_secs_f32()
                > 0.5
            {
                sh_time = SystemTime::now();
                sh_flag = if sh_flag { false } else { true };
            }
            queue!(
                stdout,
                cursor::MoveTo(sh_x, sh_y),
                Print("我朝天空歌唱，不知声去何方".with(if sh_flag {
                    Color::DarkGrey
                } else {
                    Color::White
                }))
            )?;
            queue!(
                stdout,
                cursor::MoveTo(move_x, move_y),
                Print("我".with(if sh_flag {
                    Color::Cyan
                } else {
                    Color::DarkCyan
                }))
            )?;
            if move_x >0{
                queue!(
                    stdout,
                    cursor::MoveTo(move_x-1, move_y),
                    Print(" ")
                )?;
            }
            if move_x >1{
                queue!(
                    stdout,
                    cursor::MoveTo(move_x-2, move_y),
                    Print(" ")
                )?;
            }
            if move_x <159{
                queue!(
                    stdout,
                    cursor::MoveTo(move_x+1, move_y),
                    Print(" ")
                )?;
            }
            if move_x <158{
                queue!(
                    stdout,
                    cursor::MoveTo(move_x+2, move_y),
                    Print(" ")
                )?;
            }
            stdout.flush()?;
        }
    }

    // prepare to close terminal
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}

fn draw_box(
    stdout: &mut std::io::Stdout,
    anchor_y: u16,
    anchor_x: u16,
    height: u16,
    width: u16,
) -> Result<()> {
    let x_first = anchor_x;
    let x_limit = anchor_x + width; // not include
    let y_first = anchor_y;
    let y_limit = anchor_y + height; // not include
    for y in y_first..y_limit {
        for x in x_first..x_limit {
            if (y == y_first || y == y_limit - 1) || (x == x_first || x == x_limit - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("█".yellow())
                )?;
            }
        }
    }
    Ok(())
}
