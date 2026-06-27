use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyModifiers, poll, read},
    queue,
    style::Print,
};
use std::collections::VecDeque;
use std::io::{Result, Write};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "Steam Locomotive (sl) in Rust", long_about = None)]
struct Config {
    #[arg(short = 'a', help = "Accident - people cry for help")]
    accident: bool,

    #[arg(short = 'F', help = "Fly - train flies")]
    fly: bool,

    #[arg(short = 'c', long = "c51", help = "Model C51", conflicts_with = "logo")]
    c51: bool,

    #[arg(
        short = 'l',
        long = "logo",
        help = "Model SL Logo",
        conflicts_with = "c51"
    )]
    logo: bool,

    #[arg(short = '1', long = "one", help = "Start at screen bottom")]
    one: bool,
}

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

const LOGO_BODY: [&str; 4] = [
    r#"     ++      +------ "#,
    r#"     ||      |+-+ |  "#,
    r#"   /---------|| | |  "#,
    r#"  + ========  +-+ |  "#,
];

// The animated wheels (6 frames, 2 lines each)
const LOGO_WHEELS: [[&str; 2]; LOGOPATTERNS] = [
    [r#" _|--O========O~\-+  "#, r#"//// \_/      \_/    "#],
    [r#" _|--/O========O\-+  "#, r#"//// \_/      \_/    "#],
    [r#" _|--/~O========O-+  "#, r#"//// \_/      \_/    "#],
    [r#" _|--/~\------/~\-+  "#, r#"//// \_O========O    "#],
    [r#" _|--/~\------/~\-+  "#, r#"//// \O========O/    "#],
    [r#" _|--/~\------/~\-+  "#, r#"//// O========O_/    "#],
];

const LOGO_ERASER: &str = r#"                     "#;

const LOGO_COAL: [&str; 7] = [
    r#"____                 "#,
    r#"|   \@@@@@@@@@@@     "#,
    r#"|    \@@@@@@@@@@@@@_ "#,
    r#"|                  | "#,
    r#"|__________________| "#,
    r#"   (O)       (O)     "#,
    r#"                     "#,
];

const LOGO_CAR: [&str; 7] = [
    r#"____________________ "#,
    r#"|  ___ ___ ___ ___ | "#,
    r#"|  |_| |_| |_| |_| | "#,
    r#"|__________________| "#,
    r#"|__________________| "#,
    r#"   (O)        (O)    "#,
    r#"                     "#,
];

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
// trajectory of smoke puffs
const DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

#[derive(Debug, Clone, Copy, Default)]
enum PuffKind {
    Black = 0,
    #[default]
    White,
}

impl PuffKind {
    fn succ(&self) -> Self {
        match self {
            PuffKind::Black => PuffKind::White,
            PuffKind::White => PuffKind::Black,
        }
    }
}

struct SmokePuff {
    y: i32,
    x: i32,
    frame: usize,
    kind: PuffKind,
}

struct SmokePlume {
    puffs: VecDeque<SmokePuff>,
    next_puff: PuffKind,
}

impl SmokePlume {
    fn new() -> Self {
        SmokePlume {
            // Pre-allocate capacity to avoid reallocation
            puffs: VecDeque::with_capacity(256),
            next_puff: PuffKind::default(),
        }
    }

    fn add_smoke(&mut self, tui: &mut Tui, y: i32, x: i32) -> Result<()> {
        if x % 4 != 0 {
            return Ok(());
        }

        // Update and draw existing puffs
        for puff in &mut self.puffs {
            tui.draw_text(puff.y, puff.x, ERASER[puff.frame])?; // Erase old 

            // Update positions
            puff.y -= DY[puff.frame];
            puff.x += DX[puff.frame];

            if puff.frame < SMOKEPTNS - 1 {
                puff.frame += 1;
            }
            tui.draw_text(puff.y, puff.x, SMOKE[puff.kind as usize][puff.frame])?;
        }

        // Add new puff at the funnel
        let kind = self.next_puff;
        tui.draw_text(y, x, SMOKE[kind as usize][0])?;

        self.puffs.push_back(SmokePuff {
            y,
            x,
            frame: 0,
            kind,
        });
        self.next_puff = self.next_puff.succ();

        // Cap the length
        // A puff is added every 4 'x' units. Capping at 256 means we retain
        // smoke history for 1,024 terminal columns - more than enough
        if self.puffs.len() > 256 {
            self.puffs.pop_front();
        }

        Ok(())
    }
}

// Make sure terminal raw mode is disabled before exit
pub struct TerminalGuard;

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

struct Tui {
    _guard: TerminalGuard,
    stdout: std::io::Stdout,
    rows: u16,
    cols: u16,
}

impl Tui {
    fn new() -> Result<Self> {
        let mut stdout = std::io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::style::ResetColor,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0)
        )?;
        crossterm::terminal::enable_raw_mode()?;

        let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
        stdout.flush()?;
        Ok(Tui {
            _guard: TerminalGuard,
            stdout,
            rows,
            cols,
        })
    }

    //fn draw_text(&mut self, y: i32, x: i32, s: &str) -> Result<()> {
    //    let (rows, cols) = (self.rows as i32, self.cols as i32);
    //    if (0..rows).contains(&y) {
    //        for (i, c) in s.chars().enumerate() {
    //            let x = x + i as i32;
    //            if (0..cols).contains(&x) {
    //                queue!(self.stdout, MoveTo(x as u16, y as u16), Print(c))?;
    //            }
    //        }
    //    }
    //    Ok(())
    //}

    fn draw_text(&mut self, y: i32, x: i32, s: &str) -> Result<()> {
        let (rows, cols) = (self.rows as i32, self.cols as i32);

        // Skip if completely off-screen vertically or horizontally
        let len = s.len() as i32;
        if y < 0 || y >= rows || x >= cols || x + len <= 0 {
            return Ok(());
        }
        debug_assert!(
            s.is_ascii(),
            "draw_text assumes pure ASCII for safe byte slicing"
        );

        // Calculate the visible boundaries safely
        let start_idx = if x < 0 { (-x) as usize } else { 0 };
        let end_idx = if x + len > cols {
            (cols - x) as usize
        } else {
            s.len()
        };

        // Slice and draw
        if start_idx < end_idx && end_idx <= s.len() {
            let visible_s = &s[start_idx..end_idx];
            let draw_x = if x < 0 { 0 } else { x } as u16;
            queue!(self.stdout, MoveTo(draw_x, y as u16), Print(visible_s))?;
        }

        Ok(())
    }

    fn add_man(&mut self, y: i32, x: i32, train_length: i32) -> Result<()> {
        let idx = ((train_length + x) / 12 % 2) as usize;
        let frame = MAN_FRAMES[idx];
        for (i, s) in frame.iter().enumerate() {
            self.draw_text(y + i as i32, x, s)?;
        }
        Ok(())
    }

    // return true while the train is visible on screen
    fn render_train(&mut self, env: &mut SmokePlume, args: &Config, x: i32) -> Result<bool> {
        let visible = if args.logo {
            self.render_logo(env, args, x)?
        } else if args.c51 {
            self.render_c51(env, args, x)?
        } else {
            self.render_d51(env, args, x)?
        };
        self.stdout.flush()?;
        Ok(visible)
    }

    fn render_logo(&mut self, env: &mut SmokePlume, args: &Config, x: i32) -> Result<bool> {
        if x < -LOGOLENGTH {
            return Ok(false);
        }

        let mut y = if args.one {
            self.rows as i32 - LOGOHEIGHT
        } else {
            (self.rows as i32) / 2 - 3
        };
        let mut py1 = 0;
        let mut py2 = 0;
        let mut py3 = 0;

        if args.fly {
            y = (x / 6) + (self.rows as i32) - (self.cols as i32 / 6) - LOGOHEIGHT;
            py1 = 2;
            py2 = 4;
            py3 = 6;
        }

        let ptn = ((LOGOLENGTH + x) / 3 % LOGOPATTERNS as i32) as usize;

        // Render train body + animated wheels below the body
        self.draw_lines(y, x, &LOGO_BODY)?;
        let height = LOGO_BODY.len() as i32;
        self.draw_lines(y + height, x, &LOGO_WHEELS[ptn])?;
        let height = height + LOGO_WHEELS[0].len() as i32;
        self.draw_text(y + height, x, LOGO_ERASER)?;

        // Render the attached cars
        self.draw_lines(y + py1, x + 21, &LOGO_COAL)?;
        self.draw_lines(y + py2, x + 42, &LOGO_CAR)?;
        self.draw_lines(y + py3, x + 63, &LOGO_CAR)?;

        if args.accident {
            self.add_man(y + 1, x + 14, LOGOLENGTH)?;
            self.add_man(y + 1 + py2, x + 45, LOGOLENGTH)?;
            self.add_man(y + 1 + py2, x + 53, LOGOLENGTH)?;
            self.add_man(y + 1 + py3, x + 66, LOGOLENGTH)?;
            self.add_man(y + 1 + py3, x + 74, LOGOLENGTH)?;
        }
        env.add_smoke(self, y - 1, x + LOGOFUNNEL)?;
        Ok(true)
    }

    fn render_d51(&mut self, env: &mut SmokePlume, args: &Config, x: i32) -> Result<bool> {
        if x < -D51LENGTH {
            return Ok(false);
        }
        let mut y = if args.one {
            self.rows as i32 - D51HEIGHT
        } else {
            (self.rows as i32) / 2 - 5
        };
        let mut dy = 0;

        if args.fly {
            y = (x / 7) + (self.rows as i32) - (self.cols as i32 / 7) - D51HEIGHT;
            dy = 1;
        }

        let ptn = ((D51LENGTH + x) as usize) % D51PATTERNS;

        // Render train body + animated wheels below the body
        self.draw_lines(y, x, &D51_BODY)?;
        let height = D51_BODY.len() as i32;
        self.draw_lines(y + height, x, &D51_WHEELS[ptn])?;
        let height = height + D51_WHEELS[0].len() as i32;
        self.draw_text(y + height, x, WHEELS_ERASER)?;

        // Render the attached coal car
        let car_x_offset = 53;
        self.draw_lines(y + dy, x + car_x_offset, &D51_COAL)?;

        if args.accident {
            self.add_man(y + 2, x + 43, D51LENGTH)?;
            self.add_man(y + 2, x + 47, D51LENGTH)?;
        }
        env.add_smoke(self, y - 1, x + D51FUNNEL)?;
        Ok(true)
    }

    fn draw_lines(&mut self, y: i32, x: i32, lines: &[&str]) -> Result<()> {
        for (i, line) in lines.iter().enumerate() {
            self.draw_text(y + i as i32, x, line)?;
        }
        Ok(())
    }

    fn render_c51(&mut self, env: &mut SmokePlume, args: &Config, x: i32) -> Result<bool> {
        if x < -C51LENGTH {
            return Ok(false);
        }
        let mut y = if args.one {
            self.rows as i32 - C51HEIGHT
        } else {
            (self.rows as i32) / 2 - 5
        };
        let mut dy = 0;

        if args.fly {
            y = (x / 7) + (self.rows as i32) - (self.cols as i32 / 7) - C51HEIGHT;
            dy = 1;
        }

        let ptn = ((C51LENGTH + x) as usize) % C51PATTERNS;

        // Render train body + animated wheels below the body
        self.draw_lines(y, x, &C51_BODY)?;
        let height = C51_BODY.len() as i32;
        self.draw_lines(y + height, x, &C51_WHEELS[ptn])?;
        let height = height + C51_WHEELS[0].len() as i32;
        self.draw_text(y + height, x, WHEELS_ERASER)?;

        // Render the attached coal car
        let car_x_offset = 55;
        self.draw_lines(y + dy, x + car_x_offset, &C51_COAL)?;

        if args.accident {
            self.add_man(y + 3, x + 45, C51LENGTH)?;
            self.add_man(y + 3, x + 49, C51LENGTH)?;
        }
        env.add_smoke(self, y - 1, x + C51FUNNEL)?;
        Ok(true)
    }
}

fn main() -> Result<()> {
    let args = Config::parse();
    let mut tui = Tui::new()?;

    let mut x = tui.cols as i32 - 1;
    let mut plume = SmokePlume::new();

    while tui.render_train(&mut plume, &args, x)? {
        // Check for interrupt signal (Ctrl+C) early exit
        const FRAME_DURATION: Duration = Duration::from_millis(40);
        if poll(FRAME_DURATION)?
            && let Event::Key(event) = read()?
            && event.code == KeyCode::Char('c')
            && event.modifiers.contains(KeyModifiers::CONTROL)
        {
            break;
        }
        x -= 1;
    }
    Ok(())
}
