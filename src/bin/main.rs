use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyModifiers, poll, read},
    queue,
    style::Print,
};
use std::io::{Result, Write, stdout};
use std::time::Duration;

// --- D51 Constants ---
const D51HEIGHT: i32 = 10;
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
const COAL04: &str = r#"  _|                 \_____A  "#;
const COAL05: &str = r#" =|                        |  "#;
const COAL06: &str = r#" -|                        |  "#;
const COAL07: &str = r#"__|________________________|_ "#;
const COAL08: &str = r#"|__________________________|_ "#;
const COAL09: &str = r#"   |_D__D__D_|  |_D__D__D_|   "#;
const COAL10: &str = r#"    \_/   \_/    \_/   \_/    "#;
const COALDEL: &str = r#"                              "#;

// --- LOGO Constants ---
const LOGOHEIGHT: i32 = 6;
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

// --- C51 Constants ---
const C51HEIGHT: i32 = 11;
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

// --- State Structs ---
struct State {
    accident: bool,
    logo: bool,
    fly: bool,
    c51: bool,
}

// --- Drawing Helper ---
fn my_mvaddstr(
    stdout: &mut impl Write,
    y: i32,
    x: i32,
    s: &str,
    cols: u16,
    rows: u16,
) -> Result<()> {
    if (0..rows as i32).contains(&y) {
        for (i, c) in s.chars().enumerate() {
            let x = x + i as i32;
            if (0..cols as i32).contains(&x) {
                queue!(stdout, MoveTo(x as u16, y as u16), Print(c))?;
            }
        }
    }
    Ok(())
}

// --- Smoke Logic ---
const SMOKEPTNS: usize = 16;
const SMOKE: [[&str; SMOKEPTNS]; 2] = [
    [
        "(   )", "(    )", "(    )", "(   )", "(  )", "(  )", "( )", "( )", "()", "()", "O", "O",
        "O", "O", "O", " ",
    ],
    [
        "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)", "(@@)", "(@)", "(@)", "@@", "@@", "@", "@",
        "@", "@", "@", " ",
    ],
];
const ERASER: [&str; SMOKEPTNS] = [
    "     ", "      ", "      ", "     ", "    ", "    ", "   ", "   ", "  ", "  ", " ", " ", " ",
    " ", " ", " ",
];
const DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

struct Smoke {
    y: i32,
    x: i32,
    ptrn: usize,
    kind: usize,
}

struct SmokeEnv {
    smokes: Vec<Smoke>,
    smoke_count: usize,
}

impl SmokeEnv {
    fn add_smoke(
        &mut self,
        stdout: &mut impl Write,
        y: i32,
        x: i32,
        cols: u16,
        rows: u16,
    ) -> Result<()> {
        if x % 4 == 0 {
            for smoke in &mut self.smokes {
                my_mvaddstr(stdout, smoke.y, smoke.x, ERASER[smoke.ptrn], cols, rows)?;
                smoke.y -= DY[smoke.ptrn];
                smoke.x += DX[smoke.ptrn];
                if smoke.ptrn < SMOKEPTNS - 1 {
                    smoke.ptrn += 1;
                }
                my_mvaddstr(
                    stdout,
                    smoke.y,
                    smoke.x,
                    SMOKE[smoke.kind][smoke.ptrn],
                    cols,
                    rows,
                )?;
            }
            my_mvaddstr(stdout, y, x, SMOKE[self.smoke_count % 2][0], cols, rows)?;
            self.smokes.push(Smoke {
                y,
                x,
                ptrn: 0,
                kind: self.smoke_count % 2,
            });
            self.smoke_count += 1;
        }
        Ok(())
    }
}

// --- Little Men Logic ---
fn add_man(stdout: &mut impl Write, y: i32, x: i32, cols: u16, rows: u16) -> Result<()> {
    let man = [["", "(O)"], ["Help!", "\\O/"]];
    let idx = ((LOGOLENGTH + x) / 12 % 2) as usize;
    for i in 0..2 {
        my_mvaddstr(stdout, y + i, x, man[idx][i as usize], cols, rows)?;
    }
    Ok(())
}

// --- Train Logic ---
fn add_sl(
    stdout: &mut impl Write,
    env: &mut SmokeEnv,
    state: &State,
    x: i32,
    cols: u16,
    rows: u16,
) -> Result<bool> {
    if x < -LOGOLENGTH {
        return Ok(false);
    }

    let sl: [[&str; 7]; LOGOPATTERNS] = [
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN],
    ];
    let coal = [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];
    let car = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

    let mut y = (rows as i32) / 2 - 3;
    let mut py1 = 0;
    let mut py2 = 0;
    let mut py3 = 0;

    if state.fly {
        y = (x / 6) + (rows as i32) - (cols as i32 / 6) - LOGOHEIGHT;
        py1 = 2;
        py2 = 4;
        py3 = 6;
    }

    let ptn = ((LOGOLENGTH + x) / 3 % LOGOPATTERNS as i32) as usize;

    for i in 0..=LOGOHEIGHT {
        let i_usize = i as usize;
        my_mvaddstr(stdout, y + i, x, sl[ptn][i_usize], cols, rows)?;
        my_mvaddstr(stdout, y + i + py1, x + 21, coal[i_usize], cols, rows)?;
        my_mvaddstr(stdout, y + i + py2, x + 42, car[i_usize], cols, rows)?;
        my_mvaddstr(stdout, y + i + py3, x + 63, car[i_usize], cols, rows)?;
    }

    if state.accident {
        add_man(stdout, y + 1, x + 14, cols, rows)?;
        add_man(stdout, y + 1 + py2, x + 45, cols, rows)?;
        add_man(stdout, y + 1 + py2, x + 53, cols, rows)?;
        add_man(stdout, y + 1 + py3, x + 66, cols, rows)?;
        add_man(stdout, y + 1 + py3, x + 74, cols, rows)?;
    }
    env.add_smoke(stdout, y - 1, x + LOGOFUNNEL, cols, rows)?;
    Ok(true)
}

fn add_d51(
    stdout: &mut impl Write,
    env: &mut SmokeEnv,
    state: &State,
    x: i32,
    cols: u16,
    rows: u16,
) -> Result<bool> {
    if x < -D51LENGTH {
        return Ok(false);
    }

    let d51: [[&str; 11]; D51PATTERNS] = [
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
    let coal = [
        COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
    ];

    let mut y = (rows as i32) / 2 - 5;
    let mut dy = 0;

    if state.fly {
        y = (x / 7) + (rows as i32) - (cols as i32 / 7) - D51HEIGHT;
        dy = 1;
    }

    let ptn = ((D51LENGTH + x) as usize) % D51PATTERNS;

    for i in 0..=D51HEIGHT {
        let i_usize = i as usize;
        my_mvaddstr(stdout, y + i, x, d51[ptn][i_usize], cols, rows)?;
        my_mvaddstr(stdout, y + i + dy, x + 53, coal[i_usize], cols, rows)?;
    }

    if state.accident {
        add_man(stdout, y + 2, x + 43, cols, rows)?;
        add_man(stdout, y + 2, x + 47, cols, rows)?;
    }
    env.add_smoke(stdout, y - 1, x + D51FUNNEL, cols, rows)?;
    Ok(true)
}

fn add_c51(
    stdout: &mut impl Write,
    env: &mut SmokeEnv,
    state: &State,
    x: i32,
    cols: u16,
    rows: u16,
) -> Result<bool> {
    if x < -C51LENGTH {
        return Ok(false);
    }

    let c51: [[&str; 12]; C51PATTERNS] = [
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH11, C51WH12,
            C51WH13, C51WH14, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH21, C51WH22,
            C51WH23, C51WH24, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH31, C51WH32,
            C51WH33, C51WH34, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH41, C51WH42,
            C51WH43, C51WH44, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH51, C51WH52,
            C51WH53, C51WH54, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH61, C51WH62,
            C51WH63, C51WH64, C51DEL,
        ],
    ];
    let coal = [
        COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10,
        COALDEL,
    ];

    let mut y = (rows as i32) / 2 - 5;
    let mut dy = 0;

    if state.fly {
        y = (x / 7) + (rows as i32) - (cols as i32 / 7) - C51HEIGHT;
        dy = 1;
    }

    let ptn = ((C51LENGTH + x) as usize) % C51PATTERNS;

    for i in 0..=C51HEIGHT {
        let i_usize = i as usize;
        my_mvaddstr(stdout, y + i, x, c51[ptn][i_usize], cols, rows)?;
        my_mvaddstr(stdout, y + i + dy, x + 55, coal[i_usize], cols, rows)?;
    }

    if state.accident {
        add_man(stdout, y + 3, x + 45, cols, rows)?;
        add_man(stdout, y + 3, x + 49, cols, rows)?;
    }
    env.add_smoke(stdout, y - 1, x + C51FUNNEL, cols, rows)?;
    Ok(true)
}

// Make sure terminal raw mode is disabled before exit
pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> Result<Self> {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::style::ResetColor,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0)
        )?;
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );
    }
}

// --- Main Program ---
fn main() -> Result<()> {
    let _guard = TerminalGuard::new();

    let mut state = State {
        accident: false,
        logo: false,
        fly: false,
        c51: false,
    };

    // Parse command line arguments
    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => state.accident = true,
                    'F' => state.fly = true,
                    'l' => state.logo = true,
                    'c' => state.c51 = true,
                    _ => {}
                }
            }
        }
    }

    // Terminal initialization
    let mut stdout = stdout();
    let _ = stdout.flush();

    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let mut x = cols as i32 - 1;
    let mut env = SmokeEnv {
        smokes: Vec::new(),
        smoke_count: 0,
    };

    loop {
        // Render step
        let keep_going = if state.logo {
            add_sl(&mut stdout, &mut env, &state, x, cols, rows)?
        } else if state.c51 {
            add_c51(&mut stdout, &mut env, &state, x, cols, rows)?
        } else {
            add_d51(&mut stdout, &mut env, &state, x, cols, rows)?
        };

        if !keep_going {
            break;
        }
        let _ = stdout.flush();

        // Check for interrupt signal (Ctrl+C) early exit
        if poll(Duration::from_millis(0)).unwrap()
            && let Event::Key(event) = read().unwrap()
            && event.code == KeyCode::Char('c')
            && event.modifiers.contains(KeyModifiers::CONTROL)
        {
            break;
        }

        std::thread::sleep(Duration::from_millis(40));
        x -= 1;
    }
    Ok(())
}
