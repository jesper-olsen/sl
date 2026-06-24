use clap::Parser;
use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read},
    execute, queue, style,
    terminal::{self, Clear, ClearType, size},
};
use std::io::Result;
use std::io::Write;
//use std::io::{Write, stdout};
use std::time::Duration;

const D51HEIGHT: usize = 10;
const D51FUNNEL: i32 = 7;
const D51LENGTH: i32 = 83;
const D51PATTERNS: usize = 6;

const D51STR1: &str = r#"      ====        ________                ___________ "#;
const D51STR2: &str = r#"  _D _|  |_______/        \__I_I_____===__|_________| "#;
const D51STR3: &str = r#"   |(_)---  |   H\________/ |   |        =|___ ___|   "#;
const D51STR4: &str = r#"   /     |  |   H  |  |     |   |         ||_| |_||   "#;
const D51STR5: &str = r#"  |      |  |   H  |__--------------------| [___] |   "#;
const D51STR6: &str = r#"  | ________|___H__/__|_____/[][]~\_______|       |   "#;
const D51STR7: &str = r#"  |/ |   |-----------I_____I [][] []  D   |=======|__ "#;

const D51WHL11: &str = r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#;
const D51WHL12: &str = r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#;
const D51WHL13: &str = r#"  \_/      \O=====O=====O=====O_/      \_/            "#;

const D51WHL21: &str = r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#;
const D51WHL22: &str = r#" |/-=|___|=O=====O=====O=====O   |_____/~\___/        "#;
const D51WHL23: &str = r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#;

const D51WHL31: &str = r#"__/ =| o |=-O=====O=====O=====O \ ____Y___________|__ "#;
const D51WHL32: &str = r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#;
const D51WHL33: &str = r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#;

const D51WHL41: &str = r#"__/ =| o |=-~O=====O=====O=====O\ ____Y___________|__ "#;
const D51WHL42: &str = r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#;
const D51WHL43: &str = r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#;

const D51WHL51: &str = r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#;
const D51WHL52: &str = r#" |/-=|___|=   O=====O=====O=====O|_____/~\___/        "#;
const D51WHL53: &str = r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#;

const D51WHL61: &str = r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#;
const D51WHL62: &str = r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#;
const D51WHL63: &str = r#"  \_/      \_O=====O=====O=====O/      \_/            "#;

const D51DEL: &str = r#"                                                      "#;

const COAL01: &str = r#"                              "#;
const COAL02: &str = r#"                              "#;
const COAL03: &str = r#"   _________________          "#;
const COAL04: &str = r#"  _|                \_____A   "#;
const COAL05: &str = r#" =|                        |  "#;
const COAL06: &str = r#" -|                        |  "#;
const COAL07: &str = r#"__|________________________|_ "#;
const COAL08: &str = r#"|__________________________|_ "#;
const COAL09: &str = r#"   |_D__D__D_|  |_D__D__D_|   "#;
const COAL10: &str = r#"    \_/   \_/    \_/   \_/    "#;
const COALDEL: &str = r#"                              "#;

const LOGOHEIGHT: usize = 6;
const LOGOFUNNEL: i32 = 4;
const LOGOLENGTH: i32 = 84;
const LOGOPATTERNS: usize = 6;

const LOGO1: &str = r#"     ++      +------ "#;
const LOGO2: &str = r#"     ||      |+-+ |  "#;
const LOGO3: &str = r#"   /---------|| | |  "#;
const LOGO4: &str = r#"  + ========  +-+ |  "#;

const LWHL11: &str = r#" _|--O========O~\-+  "#;
const LWHL12: &str = r#"//// \_/      \_/    "#;

const LWHL21: &str = r#" _|--/O========O\-+  "#;
const LWHL22: &str = r#"//// \_/      \_/    "#;

const LWHL31: &str = r#" _|--/~O========O-+  "#;
const LWHL32: &str = r#"//// \_/      \_/    "#;

const LWHL41: &str = r#" _|--/~\------/~\-+  "#;
const LWHL42: &str = r#"//// \_O========O    "#;

const LWHL51: &str = r#" _|--/~\------/~\-+  "#;
const LWHL52: &str = r#"//// \O========O/    "#;

const LWHL61: &str = r#" _|--/~\------/~\-+  "#;
const LWHL62: &str = r#"//// O========O_/    "#;

const LCOAL1: &str = r#"____                 "#;
const LCOAL2: &str = r#"|   \@@@@@@@@@@@     "#;
const LCOAL3: &str = r#"|    \@@@@@@@@@@@@@_ "#;
const LCOAL4: &str = r#"|                  | "#;
const LCOAL5: &str = r#"|__________________| "#;
const LCOAL6: &str = r#"   (O)       (O)     "#;

const LCAR1: &str = r#"____________________ "#;
const LCAR2: &str = r#"|  ___ ___ ___ ___ | "#;
const LCAR3: &str = r#"|  |_| |_| |_| |_| | "#;
const LCAR4: &str = r#"|__________________| "#;
const LCAR5: &str = r#"|__________________| "#;
const LCAR6: &str = r#"   (O)        (O)    "#;

const DELLN: &str = r#"                     "#;

const C51HEIGHT: usize = 11;
const C51FUNNEL: i32 = 7;
const C51LENGTH: i32 = 87;
const C51PATTERNS: usize = 6;

const C51DEL: &str = r#"                                                       "#;
const C51STR1: &str = r#"        ___                                            "#;
const C51STR2: &str = r#"       _|_|_  _     __       __             ___________"#;
const C51STR3: &str = r#"    D__/   \_(_)___|  |__H__|  |_____I_Ii_()|_________|"#;
const C51STR4: &str = r#"     | `---'   |:: `--'  H  `--'         |  |___ ___|  "#;
const C51STR5: &str = r#"    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  "#;
const C51STR6: &str = r#"    ||        | ::       H  +=====+      |  |::  ...|  "#;
const C51STR7: &str = r#"|    | _______|_::-----------------[][]-----|       |  "#;

const C51WH61: &str = r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#;
const C51WH62: &str = r#"------'|oOo|==[]=-     ||      ||      |  ||=======_|__"#;
const C51WH63: &str = r#"/~\____|___|/~\_|   O=======O=======O  |__|+-/~\_|     "#;
const C51WH64: &str = r#"\_/         \_/  \____/  \____/  \____/      \_/       "#;

const C51WH51: &str = r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#;
const C51WH52: &str = r#"------'|oOo|===[]=-    ||      ||      |  ||=======_|__"#;
const C51WH53: &str = r#"/~\____|___|/~\_|    O=======O=======O |__|+-/~\_|     "#;
const C51WH54: &str = r#"\_/         \_/  \____/  \____/  \____/      \_/       "#;

const C51WH41: &str = r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#;
const C51WH42: &str = r#"------'|oOo|===[]=- O=======O=======O  |  ||=======_|__"#;
const C51WH43: &str = r#"/~\____|___|/~\_|      ||      ||      |__|+-/~\_|     "#;
const C51WH44: &str = r#"\_/         \_/  \____/  \____/  \____/      \_/       "#;

const C51WH31: &str = r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#;
const C51WH32: &str = r#"------'|oOo|==[]=- O=======O=======O   |  ||=======_|__"#;
const C51WH33: &str = r#"/~\____|___|/~\_|      ||      ||      |__|+-/~\_|     "#;
const C51WH34: &str = r#"\_/         \_/  \____/  \____/  \____/      \_/       "#;

const C51WH21: &str = r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#;
const C51WH22: &str = r#"------'|oOo|=[]=- O=======O=======O    |  ||=======_|__"#;
const C51WH23: &str = r#"/~\____|___|/~\_|      ||      ||      |__|+-/~\_|     "#;
const C51WH24: &str = r#"\_/         \_/  \____/  \____/  \____/      \_/       "#;

const C51WH11: &str = r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#;
const C51WH12: &str = r#"------'|oOo|=[]=-      ||      ||      |  ||=======_|__"#;
const C51WH13: &str = r#"/~\____|___|/~\_|  O=======O=======O   |__|+-/~\_|     "#;
const C51WH14: &str = r#"\_/         \_/  \____/  \____/  \____/      \_/       "#;

const SMOKEPTNS: usize = 16;
const SMOKE: [[&str; SMOKEPTNS]; 2] = [
    [
        r#"(   )"#,
        r#"(    )"#,
        r#"(    )"#,
        r#"(   )"#,
        r#"(  )"#,
        r#"(  )"#,
        r#"( )"#,
        r#"( )"#,
        r#"()"#,
        r#"()"#,
        r#"O"#,
        r#"O"#,
        r#"O"#,
        r#"O"#,
        r#"O"#,
        r#" "#,
    ],
    [
        r#"(@@@)"#,
        r#"(@@@@)"#,
        r#"(@@@@)"#,
        r#"(@@@)"#,
        r#"(@@)"#,
        r#"(@@)"#,
        r#"(@)"#,
        r#"(@)"#,
        r#"@@"#,
        r#"@@"#,
        r#"@"#,
        r#"@"#,
        r#"@"#,
        r#"@"#,
        r#"@"#,
        r#" "#,
    ],
];

const ERASER: [&str; SMOKEPTNS] = [
    r#"     "#,
    r#"      "#,
    r#"      "#,
    r#"     "#,
    r#"    "#,
    r#"    "#,
    r#"   "#,
    r#"   "#,
    r#"  "#,
    r#"  "#,
    r#" "#,
    r#" "#,
    r#" "#,
    r#" "#,
    r#" "#,
    r#" "#,
];

const DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

// Aggregate arrays
const D51_COAL: [&str; D51HEIGHT + 1] = [
    COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
];
const D51_ARR: [[&str; D51HEIGHT + 1]; D51PATTERNS] = [
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL11, D51WHL12,
        D51WHL13, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL21, D51WHL22,
        D51WHL23, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL31, D51WHL32,
        D51WHL33, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL41, D51WHL42,
        D51WHL43, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL51, D51WHL52,
        D51WHL53, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL61, D51WHL62,
        D51WHL63, D51DEL,
    ],
];

const C51_COAL: [&str; C51HEIGHT + 1] = [
    COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10,
    COALDEL,
];
const C51_ARR: [[&str; C51HEIGHT + 1]; C51PATTERNS] = [
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH11, C51WH12, C51WH13,
        C51WH14, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH21, C51WH22, C51WH23,
        C51WH24, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH31, C51WH32, C51WH33,
        C51WH34, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH41, C51WH42, C51WH43,
        C51WH44, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH51, C51WH52, C51WH53,
        C51WH54, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH61, C51WH62, C51WH63,
        C51WH64, C51DEL,
    ],
];

const SL_ARR: [[&str; LOGOHEIGHT + 1]; LOGOPATTERNS] = [
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN],
];
const LCOAL_ARR: [&str; LOGOHEIGHT + 1] = [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];
const LCAR_ARR: [&str; LOGOHEIGHT + 1] = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

struct SmokeDrop {
    y: i32,
    x: i32,
    ptrn: usize,
    kind: usize,
}

struct SmokeState {
    drops: Vec<SmokeDrop>,
    next_kind: usize,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Steam Locomotive (sl) in Rust", long_about = None)]
struct Config {
    #[arg(short = 'a', help = "Accident - people cry for help")]
    accident: bool,
    #[arg(short = 'F', help = "Fly - train flies")]
    fly: bool,
    #[arg(short = 'l', help = "Logo - draw a miniature sl")]
    logo: bool,
    #[arg(short = 'c', help = "C51 - draw C51 train instead of D51")]
    c51: bool,
}

fn my_mvaddstr(
    stdout: &mut impl Write,
    y: i32,
    x: i32,
    s: &str,
    cols: u16,
    rows: u16,
) -> Result<()> {
    if y < 0 || y >= rows as i32 {
        return Ok(());
    }

    let chars: Vec<char> = s.chars().collect();
    let mut draw_x = x;
    let mut start_idx = 0;

    if x < 0 {
        start_idx = (-x) as usize;
        draw_x = 0;
    }

    if start_idx >= chars.len() {
        return Ok(());
    }

    let max_len = (cols as i32 - draw_x).max(0) as usize;
    let len = std::cmp::min(chars.len() - start_idx, max_len);

    if len == 0 {
        return Ok(());
    }

    let display_str: String = chars[start_idx..start_idx + len].iter().collect();
    queue!(
        stdout,
        cursor::MoveTo(draw_x as u16, y as u16),
        style::Print(display_str)
    )?;

    Ok(())
}

fn add_man(
    stdout: &mut impl Write,
    y: i32,
    x: i32,
    global_x: i32,
    cols: u16,
    rows: u16,
) -> Result<()> {
    let man = [["", "(O)"], ["Help!", "\\O/"]];
    let ptrn = (((LOGOLENGTH + global_x) / 12) % 2) as usize;
    for (i,m) in man[ptrn].iter().enumerate() {
        my_mvaddstr(stdout, y + i as i32, x, m, cols, rows)?;
    }
    Ok(())
}

fn add_smoke(
    stdout: &mut impl Write,
    y: i32,
    x: i32,
    state: &mut SmokeState,
    cols: u16,
    rows: u16,
) -> Result<()> {
    if x % 4 == 0 {
        for drop in state.drops.iter_mut() {
            my_mvaddstr(stdout, drop.y, drop.x, ERASER[drop.ptrn], cols, rows)?;
            drop.y -= DY[drop.ptrn];
            drop.x += DX[drop.ptrn];
            if drop.ptrn < SMOKEPTNS - 1 {
                drop.ptrn += 1;
            }
            my_mvaddstr(
                stdout,
                drop.y,
                drop.x,
                SMOKE[drop.kind][drop.ptrn],
                cols,
                rows,
            )?;
        }
        my_mvaddstr(stdout, y, x, SMOKE[state.next_kind % 2][0], cols, rows)?;
        state.drops.push(SmokeDrop {
            y,
            x,
            ptrn: 0,
            kind: state.next_kind % 2,
        });
        state.next_kind += 1;
    }
    state.drops.retain(|d| d.ptrn < SMOKEPTNS - 1 || d.x < cols as i32);
    Ok(())
}

fn add_d51(
    stdout: &mut impl Write,
    x: i32,
    config: &Config,
    cols: u16,
    rows: u16,
    smokes: &mut SmokeState,
) -> Result<bool> {
    if x < -D51LENGTH {
        return Ok(false);
    }
    let mut y = (rows / 2) as i32 - 5;
    let mut dy = 0;

    if config.fly {
        y = (x / 7) + rows as i32 - (cols as i32 / 7) - D51HEIGHT as i32;
        dy = 1;
    }

    let ptrn = ((D51LENGTH + x) % D51PATTERNS as i32) as usize;
    for i in 0..=D51HEIGHT {
        my_mvaddstr(stdout, y + i as i32, x, D51_ARR[ptrn][i], cols, rows)?;
        my_mvaddstr(stdout, y + i as i32 + dy, x + 53, D51_COAL[i], cols, rows)?;
    }

    if config.accident {
        add_man(stdout, y + 2, x + 43, x, cols, rows)?;
        add_man(stdout, y + 2, x + 47, x, cols, rows)?;
    }

    add_smoke(stdout, y - 1, x + D51FUNNEL, smokes, cols, rows)?;
    Ok(true)
}

fn add_c51(
    stdout: &mut impl Write,
    x: i32,
    config: &Config,
    cols: u16,
    rows: u16,
    smokes: &mut SmokeState,
) -> Result<bool> {
    if x < -C51LENGTH {
        return Ok(false);
    }
    let mut y = (rows / 2) as i32 - 5;
    let mut dy = 0;

    if config.fly {
        y = (x / 7) + rows as i32 - (cols as i32 / 7) - C51HEIGHT as i32;
        dy = 1;
    }

    let ptrn = ((C51LENGTH + x) % C51PATTERNS as i32) as usize;
    for i in 0..=C51HEIGHT {
        my_mvaddstr(stdout, y + i as i32, x, C51_ARR[ptrn][i], cols, rows)?;
        my_mvaddstr(stdout, y + i as i32 + dy, x + 55, C51_COAL[i], cols, rows)?;
    }

    if config.accident {
        add_man(stdout, y + 3, x + 45, x, cols, rows)?;
        add_man(stdout, y + 3, x + 49, x, cols, rows)?;
    }

    add_smoke(stdout, y - 1, x + C51FUNNEL, smokes, cols, rows)?;
    Ok(true)
}

fn add_sl(
    stdout: &mut impl Write,
    x: i32,
    config: &Config,
    cols: u16,
    rows: u16,
    smokes: &mut SmokeState,
) -> Result<bool> {
    if x < -LOGOLENGTH {
        return Ok(false);
    }
    let mut y = (rows / 2) as i32 - 3;
    let mut py1 = 0;
    let mut py2 = 0;
    let mut py3 = 0;

    if config.fly {
        y = (x / 6) + rows as i32 - (cols as i32 / 6) - LOGOHEIGHT as i32;
        py1 = 2;
        py2 = 4;
        py3 = 6;
    }

    let ptrn = (((LOGOLENGTH + x) / 3) % LOGOPATTERNS as i32) as usize;
    for i in 0..=LOGOHEIGHT {
        my_mvaddstr(stdout, y + i as i32, x, SL_ARR[ptrn][i], cols, rows)?;
        my_mvaddstr(stdout, y + i as i32 + py1, x + 21, LCOAL_ARR[i], cols, rows)?;
        my_mvaddstr(stdout, y + i as i32 + py2, x + 42, LCAR_ARR[i], cols, rows)?;
        my_mvaddstr(stdout, y + i as i32 + py3, x + 63, LCAR_ARR[i], cols, rows)?;
    }

    if config.accident {
        add_man(stdout, y + 1, x + 14, x, cols, rows)?;
        add_man(stdout, y + 1 + py2, x + 45, x, cols, rows)?;
        add_man(stdout, y + 1 + py2, x + 53, x, cols, rows)?;
        add_man(stdout, y + 1 + py3, x + 66, x, cols, rows)?;
        add_man(stdout, y + 1 + py3, x + 74, x, cols, rows)?;
    }

    add_smoke(stdout, y - 1, x + LOGOFUNNEL, smokes, cols, rows)?;
    Ok(true)
}

/// Initialise the terminal and make sure it is not left in raw mode when the
/// program exits.
pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> Result<Self> {
        crossterm::execute!(
            std::io::stdout(),
            style::ResetColor,
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        terminal::enable_raw_mode()?;

        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            terminal::LeaveAlternateScreen,
            cursor::Show
        );
    }
}

fn main() -> Result<()> {
    let config = Config::parse();

    let mut stdout = std::io::stdout();

    let _guard = TerminalGuard::new()?; // prefixed with _ to avoid 'unused' compiler warning.

    let (mut cols, mut rows) = size()?;
    let mut smokes = SmokeState {
        drops: Vec::new(),
        next_kind: 0,
    };
    let mut x = cols as i32 - 1;

    loop {
        if poll(Duration::from_millis(40))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers,
                    ..
                }) if modifiers.contains(KeyModifiers::CONTROL) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break,
                Event::Resize(c, r) => {
                    cols = c;
                    rows = r;
                    execute!(stdout, Clear(ClearType::All))?;
                }
                _ => {}
            }
        }

        let continue_loop = if config.logo {
            add_sl(&mut stdout, x, &config, cols, rows, &mut smokes)?
        } else if config.c51 {
            add_c51(&mut stdout, x, &config, cols, rows, &mut smokes)?
        } else {
            add_d51(&mut stdout, x, &config, cols, rows, &mut smokes)?
        };

        if !continue_loop {
            break;
        }

        stdout.flush()?;
        x -= 1;
    }

    Ok(())
}
