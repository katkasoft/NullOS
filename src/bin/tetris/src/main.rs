use crossterm::{
    cursor, event, execute, terminal,
    style::{self, Color, Stylize},
};
use rand::Rng;
use std::io::{self, Write};
use std::time::{Duration, Instant};

const ROWS: usize = 20;
const COLS: usize = 10;

const SHAPES: [[(i32, i32); 4]; 7] = [
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 1), (1, 0), (1, 1), (1, 2)],
    [(0, 0), (1, 0), (1, 1), (1, 2)],
    [(0, 2), (1, 0), (1, 1), (1, 2)],
    [(0, 1), (0, 2), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
];

struct Game {
    board: Vec<Vec<u8>>,
    score: u32,
    is_block_flying: bool,
    is_game_over: bool,
}

impl Game {
    fn new() -> Self {
        Game {
            board: vec![vec![0; COLS]; ROWS],
            score: 0,
            is_block_flying: false,
            is_game_over: false,
        }
    }

    fn generate_block(&mut self) {
        if self.is_game_over {
            return;
        }
        let mut rng = rand::thread_rng();
        let shape = SHAPES[rng.gen_range(0..SHAPES.len())];
        let start_x = 4;
        let start_y = 0;

        let can_place = shape.iter().all(|&(dy, dx)| {
            let y = start_y + dy;
            let x = start_x + dx;
            y >= 0
                && y < ROWS as i32
                && x >= 0
                && x < COLS as i32
                && self.board[y as usize][x as usize] == 0
        });

        if !can_place {
            self.is_game_over = true;
            return;
        }

        for &(dy, dx) in &shape {
            let y = (start_y + dy) as usize;
            let x = (start_x + dx) as usize;
            self.board[y][x] = 2;
        }

        self.is_block_flying = true;
    }

    fn shift_row_down(&mut self, row_index: usize) {
        for y in (1..=row_index).rev() {
            self.board[y] = self.board[y - 1].clone();
        }
        self.board[0] = vec![0; COLS];
    }

    fn check_and_clear_lines(&mut self) {
        let mut y = ROWS - 1;
        while y > 0 {
            if self.board[y].iter().all(|&c| c == 1) {
                self.shift_row_down(y);
                self.score += 1;
            } else {
                y -= 1;
            }
        }
    }

    fn go_down(&mut self) {
        let mut active = Vec::new();
        let mut can_move = true;

        for y in 0..ROWS {
            for x in 0..COLS {
                if self.board[y][x] == 2 {
                    active.push((y, x));
                    if y + 1 >= ROWS || self.board[y + 1][x] == 1 {
                        can_move = false;
                    }
                }
            }
        }

        if can_move {
            for &(y, x) in &active {
                self.board[y][x] = 0;
            }
            for &(y, x) in &active {
                self.board[y + 1][x] = 2;
            }
        } else {
            for &(y, x) in &active {
                self.board[y][x] = 1;
            }
            self.is_block_flying = false;
            self.check_and_clear_lines();
        }
    }

    fn shift_x(&mut self, add: i32) {
        let mut active = Vec::new();
        let mut can_move = true;

        for y in 0..ROWS {
            for x in 0..COLS {
                if self.board[y][x] == 2 {
                    active.push((y, x));
                    let nx = x as i32 + add;
                    if nx < 0 || nx >= COLS as i32 || self.board[y][nx as usize] == 1 {
                        can_move = false;
                    }
                }
            }
        }

        if can_move {
            for &(y, x) in &active {
                self.board[y][x] = 0;
            }
            for &(y, x) in &active {
                let nx = (x as i32 + add) as usize;
                self.board[y][nx] = 2;
            }
        }
    }

    fn rotate(&mut self) {
        let mut active = Vec::new();
        for y in 0..ROWS {
            for x in 0..COLS {
                if self.board[y][x] == 2 {
                    active.push((y, x));
                }
            }
        }

        if active.is_empty() {
            return;
        }

        let (cy, cx) = active[0];

        let new_pos: Vec<(usize, usize)> = active
            .iter()
            .map(|&(y, x)| {
                let dy = y as i32 - cy as i32;
                let dx = x as i32 - cx as i32;
                (
                    (cy as i32 + dx) as usize,
                    (cx as i32 - dy) as usize,
                )
            })
            .collect();

        let can_rotate = new_pos.iter().all(|&(y, x)| {
            y < ROWS && x < COLS && self.board[y][x] != 1
        });

        if can_rotate {
            for &(y, x) in &active {
                self.board[y][x] = 0;
            }
            for &(y, x) in &new_pos {
                self.board[y][x] = 2;
            }
        }
    }

    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        use crossterm::style::Print;

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        execute!(stdout, Print(format!("+{}+\r\n", "-".repeat(COLS * 2))))?;

        for y in 0..ROWS {
            execute!(stdout, Print("|"))?;
            for x in 0..COLS {
                if self.board[y][x] == 0 {
                    execute!(stdout, Print("  "))?;
                } else {
                    execute!(stdout, style::PrintStyledContent("██".with(Color::Red)))?;
                }
            }
            execute!(stdout, Print("|\r\n"))?;
        }

        execute!(stdout, Print(format!("+{}+\r\n", "-".repeat(COLS * 2))))?;
        execute!(stdout, Print(format!("Score: {}\r\n", self.score)))?;

        if self.is_game_over {
            execute!(stdout, Print("GAME OVER! Press any key...\r\n"))?;
        }

        stdout.flush()?;
        Ok(())
    }

    fn game_tick(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        if !self.is_block_flying {
            self.generate_block();
        }
        self.go_down();
        self.draw(stdout)?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::DisableLineWrap,
        cursor::Hide
    )?;

    let mut game = Game::new();
    game.draw(&mut stdout)?;

    let tick = Duration::from_millis(500);
    let mut last = Instant::now();

    'main: loop {
        if event::poll(Duration::from_millis(0))? {
            if let event::Event::Key(k) = event::read()? {
                match k.code {
                    event::KeyCode::Left => game.shift_x(-1),
                    event::KeyCode::Right => game.shift_x(1),
                    event::KeyCode::Down => game.go_down(),
                    event::KeyCode::Up => game.rotate(),
                    event::KeyCode::Char(' ') => {
                        for _ in 0..ROWS {
                            game.go_down();
                        }
                    }
                    event::KeyCode::Char('q') => break 'main,
                    _ => {}
                }
                game.draw(&mut stdout)?;
            }
        }

        if last.elapsed() >= tick {
            game.game_tick(&mut stdout)?;
            last = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    execute!(
        stdout,
        terminal::EnableLineWrap,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;
    Ok(())
}