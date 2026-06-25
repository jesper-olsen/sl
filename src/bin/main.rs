use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyModifiers, poll, read},
    queue,
    style::Print,
};
use std::io::{Result, Write};
use std::time::Duration;

// --- D51 Constants ---
const D51HEIGHT: i32 = 10;
const D51FUNNEL: i32 = 7;
const D51LENGTH: i32 = 83;
const D51PATTERNS: usize = 6;

// The static upper body of the D51 train
const D51_BODY: [&str; 7] = [
    r#"      ====        ________                ___________ "#,
    r#"  _D _|  |_______/        \__I_I_____===__|_________| "#,
    r#"   |(_)---  |   H\________/ |   |        =|___ ___|   "#,
    r#"   /     |  |   H  |  |     |   |         ||_| |_||   "#,
    r#"  |      |  |   H  |__--------------------| [___] |   "#,
    r#"  | ________|___H__/__|_____/[][]~\_______|       |   "#,
    r#"  |/ |   |-----------I_____I [][] []  D   |=======|__ "#,
];

// The animated wheels (6 frames, 3 lines each)
const D51_WHEELS: [[&str; 3]; D51PATTERNS] = [
    [
        r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#,
        r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#,
        r#"  \_/      \O=====O=====O=====O_/      \_/            "#,
    ],
    [
        r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#,
        r#" |/-=|___|=O=====O=====O=====O   |_____/~\___/        "#,
        r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#,
    ],
    [
        r#"__/ =| o |=-O=====O=====O=====O \ ____Y___________|__ "#,
        r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#,
        r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#,
    ],
    [
        r#"__/ =| o |=-~O=====O=====O=====O\ ____Y___________|__ "#,
        r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#,
        r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#,
    ],
    [
        r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#,
        r#" |/-=|___|=   O=====O=====O=====O|_____/~\___/        "#,
        r#"  \_/      \__/  \__/  \__/  \__/      \_/            "#,
    ],
    [
        r#"__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__ "#,
        r#" |/-=|___|=    ||    ||    ||    |_____/~\___/        "#,
        r#"  \_/      \_O=====O=====O=====O/      \_/            "#,
    ],
];

// The coal car
const D51_COAL: [&str; 11] = [
    r#"                              "#,
    r#"                              "#,
    r#"    _________________         "#,
    r#"   _|                \_____A  "#,
    r#" =|                        |  "#,
    r#" -|                        |  "#,
    r#"__|________________________|_ "#,
    r#"|__________________________|_ "#,
    r#"   |_D__D__D_|  |_D__D__D_|   "#,
    r#"    \_/   \_/    \_/   \_/    "#,
    r#"                              "#,
];

const C51_COAL: [&str; 12] = [
    r#"                              "#,
    r#"                              "#,
    r#"                              "#,
    r#"    _________________         "#,
    r#"   _|                \_____A  "#,
    r#" =|                        |  "#,
    r#" -|                        |  "#,
    r#"__|________________________|_ "#,
    r#"|__________________________|_ "#,
    r#"   |_D__D__D_|  |_D__D__D_|   "#,
    r#"    \_/   \_/    \_/   \_/    "#,
    r#"                              "#,
];

const MAN_FRAMES: [[&str; 2]; 2] = [["", "(O)"], ["Help!", "\\O/"]];

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

const C51_BODY: [&str; 7] = [
    r#"        ___                                            "#,
    r#"       _|_|_  _     __       __             ___________"#,
    r#"    D__/   \_(_)___|  |__H__|  |_____I_Ii_()|_________|"#,
    r#"     | `---'   |:: `--'  H  `--'         |  |___ ___|  "#,
    r#"    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  "#,
    r#"    ||        | ::       H  +=====+      |  |::  ...|  "#,
    r#"|    | _______|_::-----------------[][]-----|       |  "#,
];

const C51_WHEELS: [[&str; 4]; C51PATTERNS] = [
    [
        r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#,
        r#"------'|oOo|==[]=-     ||      ||      |  ||=======_|__"#,
        r#"/~\____|___|/~\_|   O=======O=======O  |__|+-/~\_|     "#,
        r#"\_/         \_/  \____/  \____/  \____/      \_/       "#,
    ],
    [
        r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#,
        r#"------'|oOo|===[]=-    ||      ||      |  ||=======_|__"#,
        r#"/~\____|___|/~\_|    O=======O=======O |__|+-/~\_|     "#,
        r#"\_/         \_/  \____/  \____/  \____/      \_/       "#,
    ],
    [
        r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#,
        r#"------'|oOo|===[]=- O=======O=======O  |  ||=======_|__"#,
        r#"/~\____|___|/~\_|      ||      ||      |__|+-/~\_|     "#,
        r#"\_/         \_/  \____/  \____/  \____/      \_/       "#,
    ],
    [
        r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#,
        r#"------'|oOo|==[]=- O=======O=======O   |  ||=======_|__"#,
        r#"/~\____|___|/~\_|      ||      ||      |__|+-/~\_|     "#,
        r#"\_/         \_/  \____/  \____/  \____/      \_/       "#,
    ],
    [
        r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#,
        r#"------'|oOo|=[]=- O=======O=======O    |  ||=======_|__"#,
        r#"/~\____|___|/~\_|      ||      ||      |__|+-/~\_|     "#,
        r#"\_/         \_/  \____/  \____/  \____/      \_/       "#,
    ],
    [
        r#"| /~~ ||   |-----/~~~~\  /[I_____I][][] --|||_______|__"#,
        r#"------'|oOo|=[]=-      ||      ||      |  ||=======_|__"#,
        r#"/~\____|___|/~\_|  O=======O=======O   |__|+-/~\_|     "#,
        r#"\_/         \_/  \____/  \____/  \____/      \_/       "#,
    ],
];

const WHEELS_ERASER: &str = "                                                       ";

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
    fn add_smoke(&mut self, tui: &mut Tui, y: i32, x: i32) -> Result<()> {
        if x % 4 == 0 {
            for smoke in &mut self.smokes {
                tui.my_mvaddstr(smoke.y, smoke.x, ERASER[smoke.ptrn])?;
                smoke.y -= DY[smoke.ptrn];
                smoke.x += DX[smoke.ptrn];
                if smoke.ptrn < SMOKEPTNS - 1 {
                    smoke.ptrn += 1;
                }
                tui.my_mvaddstr(smoke.y, smoke.x, SMOKE[smoke.kind][smoke.ptrn])?;
            }
            tui.my_mvaddstr(y, x, SMOKE[self.smoke_count % 2][0])?;
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

// --- Train Logic ---
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

// --- State Structs ---
struct State {
    accident: bool,
    train: Train,
    fly: bool,
}

enum Train {
    C51,
    D51,
    Logo,
}

struct Tui {
    _guard: TerminalGuard,
    stdout: std::io::Stdout,
    rows: u16,
    cols: u16,
}

impl Tui {
    fn new() -> Result<Self> {
        let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
        let mut stdout = std::io::stdout();
        stdout.flush()?;
        Ok(Tui {
            _guard: TerminalGuard::new()?,
            stdout,
            rows,
            cols,
        })
    }

    fn my_mvaddstr(&mut self, y: i32, x: i32, s: &str) -> Result<()> {
        let (rows, cols) = (self.rows as i32, self.cols as i32);
        if (0..rows).contains(&y) {
            for (i, c) in s.chars().enumerate() {
                let x = x + i as i32;
                if (0..cols).contains(&x) {
                    queue!(self.stdout, MoveTo(x as u16, y as u16), Print(c))?;
                }
            }
        }
        Ok(())
    }

    fn add_man(&mut self, y: i32, x: i32) -> Result<()> {
        let idx = ((LOGOLENGTH + x) / 12 % 2) as usize;
        let frame = MAN_FRAMES[idx];
        for (i, s) in frame.iter().enumerate() {
            self.my_mvaddstr(y + i as i32, x, s)?;
        }
        Ok(())
    }

    // return true while train is still on screen
    fn render_train(&mut self, env: &mut SmokeEnv, state: &State, x: i32) -> Result<bool> {
        let done = match state.train {
            Train::Logo => self.render_sl(env, state, x)?,
            Train::C51 => self.render_c51(env, state, x)?,
            Train::D51 => self.render_d51(env, state, x)?,
        };
        self.stdout.flush()?;
        Ok(!done)
    }

    fn render_sl(&mut self, env: &mut SmokeEnv, state: &State, x: i32) -> Result<bool> {
        if x < -LOGOLENGTH {
            return Ok(true);
        }

        let (rows, cols) = (self.rows, self.cols);

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
            self.my_mvaddstr(y + i, x, sl[ptn][i_usize])?;
            self.my_mvaddstr(y + i + py1, x + 21, coal[i_usize])?;
            self.my_mvaddstr(y + i + py2, x + 42, car[i_usize])?;
            self.my_mvaddstr(y + i + py3, x + 63, car[i_usize])?;
        }

        if state.accident {
            self.add_man(y + 1, x + 14)?;
            self.add_man(y + 1 + py2, x + 45)?;
            self.add_man(y + 1 + py2, x + 53)?;
            self.add_man(y + 1 + py3, x + 66)?;
            self.add_man(y + 1 + py3, x + 74)?;
        }
        env.add_smoke(self, y - 1, x + LOGOFUNNEL)?;
        Ok(false)
    }

    fn render_d51(&mut self, env: &mut SmokeEnv, state: &State, x: i32) -> Result<bool> {
        if x < -D51LENGTH {
            return Ok(true);
        }
        let mut y = (self.rows as i32) / 2 - 5;
        let mut dy = 0;

        if state.fly {
            y = (x / 7) + (self.rows as i32) - (self.cols as i32 / 7) - D51HEIGHT;
            dy = 1;
        }

        let ptn = ((D51LENGTH + x) as usize) % D51PATTERNS;

        for (i, line) in D51_BODY.iter().enumerate() {
            self.my_mvaddstr(y + i as i32, x, line)?;
        }
        // Render the animated wheels below the body
        let height = D51_BODY.len() as i32;
        for (i, line) in D51_WHEELS[ptn].iter().enumerate() {
            self.my_mvaddstr(y + height + i as i32, x, line)?;
        }
        let height = height + D51_WHEELS[0].len() as i32;
        self.my_mvaddstr(y + height, x, WHEELS_ERASER)?;

        // Render the attached coal car
        let car_x_offset = 53;
        let car_y_offset = 0;
        for (i, line) in D51_COAL.iter().enumerate() {
            self.my_mvaddstr(y + i as i32 + dy + car_y_offset, x + car_x_offset, line)?;
        }

        if state.accident {
            self.add_man(y + 2, x + 43)?;
            self.add_man(y + 2, x + 47)?;
        }
        env.add_smoke(self, y - 1, x + D51FUNNEL)?;
        Ok(false)
    }

    fn render_c51(&mut self, env: &mut SmokeEnv, state: &State, x: i32) -> Result<bool> {
        if x < -C51LENGTH {
            return Ok(true);
        }
        let mut y = (self.rows as i32) / 2 - 5;
        let mut dy = 0;

        if state.fly {
            y = (x / 7) + (self.rows as i32) - (self.cols as i32 / 7) - C51HEIGHT;
            dy = 1;
        }

        let ptn = ((C51LENGTH + x) as usize) % C51PATTERNS;

        for (i, line) in C51_BODY.iter().enumerate() {
            self.my_mvaddstr(y + i as i32, x, line)?;
        }
        // Render the animated wheels below the body
        let height = C51_BODY.len() as i32;
        for (i, line) in C51_WHEELS[ptn].iter().enumerate() {
            self.my_mvaddstr(y + height + i as i32, x, line)?;
        }
        let height = height + C51_WHEELS[0].len() as i32;
        self.my_mvaddstr(y + height, x, WHEELS_ERASER)?;

        // Render the attached coal car
        let car_x_offset = 55;
        let car_y_offset = 0;
        for (i, line) in C51_COAL.iter().enumerate() {
            self.my_mvaddstr(y + i as i32 + dy + car_y_offset, x + car_x_offset, line)?;
        }

        if state.accident {
            self.add_man(y + 3, x + 45)?;
            self.add_man(y + 3, x + 49)?;
        }
        env.add_smoke(self, y - 1, x + C51FUNNEL)?;
        Ok(false)
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::new()?;

    let mut state = State {
        accident: false,
        fly: false,
        train: Train::D51,
    };

    // Parse command line arguments
    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => state.accident = true,
                    'F' => state.fly = true,
                    'l' => state.train = Train::Logo,
                    'c' => state.train = Train::C51,
                    _ => {}
                }
            }
        }
    }

    let mut x = tui.cols as i32 - 1;
    let mut env = SmokeEnv {
        smokes: Vec::new(),
        smoke_count: 0,
    };

    while tui.render_train(&mut env, &state, x)? {
        // Check for interrupt signal (Ctrl+C) early exit
        if poll(Duration::from_millis(0))?
            && let Event::Key(event) = read()?
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
