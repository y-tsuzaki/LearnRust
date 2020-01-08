use crossterm;
use crossterm::event::{poll, read, Event};
use crossterm::{
    cursor::{self, Hide},
    execute, queue,
    style::{self, Colorize, StyledContent},
    terminal,
    terminal::ClearType,
};
use std::io::{stdout, Write};
use std::thread;
use std::time;
use std::time::Duration;
use std::time::SystemTime;
use std::{env, io};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
// use rand::Rng;

enum TetriminoType {
    I,
    O,
    S, 
    Z,
    L,
    T,
    J
}

const TetriminoBlockI: [[bool; 4]; 4] = [
    [false, false, true, false],
    [false, false, true, false],
    [false, false, true, false],
    [false, false, true, false],
];

const TetriminoBlockO: [[bool; 4]; 4] = [
    [true, true, false, false],
    [true, true, false, false],
    [false, false, false, false],
    [false, false, false, false],
];

const TetriminoBlockS: [[bool; 4]; 4] = [
    [true, false, false, false],
    [true, true, false, false],
    [false, true, false, false],
    [false, false, false, false],
];

const TetriminoBlockZ: [[bool; 4]; 4] = [
    [false, true, false, false],
    [true, true, false, false],
    [true, false, false, false],
    [false, false, false, false],
];


const TetriminoBlockL: [[bool; 4]; 4] = [
    [false, true, false, false],
    [false, true, false, false],
    [false, true, true, false],
    [false, false, false, false],
];


const TetriminoBlockT: [[bool; 4]; 4] = [
    [false, true, false, false],
    [true, true, true, false],
    [false, false, false, false],
    [false, false, false, false],
];

const TetriminoBlockJ: [[bool; 4]; 4] = [
    [false, true, false, false],
    [false, true, false, false],
    [true, true, false, false],
    [false, false, false, false],
];

type FieldBlocks = [[bool; 10]; 20];
type Blocks = [[bool; 4]; 4];

type Pos = (i16, i16);

struct Tetrimino {
    mino_type: TetriminoType,
    blocks: Blocks,
    pos: Pos,
    size: u16, //ミノのサイズが3ブロックか4ブロックか
}

enum Direction4 {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Tetrimino {
    fn rotate(&mut self) {
        self.blocks = self._get_rotated_block();
    }

    fn _get_rotated_block(&self) -> Blocks {
        let mut tmp: Blocks = [[false; 4]; 4];

        for y in 0..self.size {
            for x in 0..self.size {
                tmp[x as usize][(self.size - 1 - y) as usize] = self.blocks[y as usize][x as usize];
            }
        }

        tmp
    }

    fn _has_collision(field: &FieldBlocks, blocks: &Blocks, pos: &Pos) -> bool {
        for y in 0..blocks.len() {
            for x in 0..blocks[0].len() {
                let global_x = (x as i16 + pos.0);
                let global_y = (y as i16 + pos.1);

                if blocks[y][x] == false {
                    continue;
                }

                if global_x < 0
                    || global_x >= (Stage::WIDTH) as i16
                    || global_y >= (Stage::HEIGHT) as i16
                {
                    return false;
                }
                if (global_y < 0) {
                    continue;
                }
                if field[global_y as usize][global_x as usize] {
                    return false;
                }
            }
        }

        return true;
    }

    fn fall(&mut self) {
        self.pos.1 += 1;
    }

    fn can_fall(&self, field: &FieldBlocks) -> bool {
        let new_pos = (self.pos.0, self.pos.1 + 1);
        return Tetrimino::_has_collision(field, &self.blocks, &new_pos);
    }

    fn can_move(&self, dir: Direction4, field: &FieldBlocks) -> bool {
        let new_pos: Pos = match dir {
            Direction4::UP => (self.pos.0, self.pos.1),
            Direction4::RIGHT => (self.pos.0 + 1, self.pos.1),
            Direction4::LEFT => (self.pos.0 - 1, self.pos.1),
            Direction4::DOWN => (self.pos.0, self.pos.1 + 1),
        };

        Tetrimino::_has_collision(field, &self.blocks, &new_pos)
    }

    fn move_to(&mut self, dir: Direction4) {
        let new_pos: Pos = match dir {
            Direction4::UP => (self.pos.0, self.pos.1),
            Direction4::RIGHT => (self.pos.0 + 1, self.pos.1),
            Direction4::LEFT => (self.pos.0 - 1, self.pos.1),
            Direction4::DOWN => (self.pos.0, self.pos.1 + 1),
        };

        self.pos = new_pos;
    }

    fn can_rotate(&mut self, field: &FieldBlocks) -> bool {
        let rotated_blocks = self._get_rotated_block();
        return Tetrimino::_has_collision(field, &rotated_blocks, &self.pos);
    }

    fn bake_to_field(&mut self, field: &mut FieldBlocks) {
        for y in 0..self.blocks.len() {
            for x in 0..self.blocks[0].len() {
                let global_y: i16 = y as i16 + self.pos.1;
                let global_x: i16 = x as i16 + self.pos.0;

                if !self.blocks[y as usize][x as usize] {
                    continue;
                }

                field[global_y as usize ][global_x as usize] = self.blocks[y as usize][x as usize];
            }    
        }
    }

    fn new(mino_type: TetriminoType) -> Self {
        let blocks = match mino_type {
            TetriminoType::I => TetriminoBlockI,
            TetriminoType::O => TetriminoBlockO,
            TetriminoType::S => TetriminoBlockS,
            TetriminoType::Z => TetriminoBlockZ,
            TetriminoType::L => TetriminoBlockL,
            TetriminoType::T => TetriminoBlockT,
            TetriminoType::J => TetriminoBlockJ,
        };
        let size = match mino_type {
            TetriminoType::I => 4,
            TetriminoType::O => 2,
            TetriminoType::S => 3,
            TetriminoType::Z => 3,
            TetriminoType::L => 3,
            TetriminoType::T => 3,
            TetriminoType::J => 3,
        } as u16;

        let pos = ((10 / 2 - size / 2) as i16, -(size as i16));

        Tetrimino {
            mino_type,
            blocks,
            pos,
            size,
        }
    }

    fn new_random(rnd : i32) -> Self {
        // FIXME ランダムにEnumを選択する賢い実装にしたい
        // let rnd = rand::thread_rng().gen_range(0, 3) /* TODO 暫定的にミノは3種類にしてる */;

        // ネットに繋がらないので擬似的に乱数っぽくしておく
        // let rnd = dummy_rand;

        let mino_type = match rnd {
            0 => TetriminoType::I,
            1 => TetriminoType::O,
            2 => TetriminoType::S,
            3 => TetriminoType::Z,
            4 => TetriminoType::L,
            5 => TetriminoType::T,
            6 => TetriminoType::J,
            _ => TetriminoType::I,
        };

        Self::new(mino_type)
    }
}

struct Stage {
    // falling_mino: Tetrimino,
    blocks: FieldBlocks,
}

impl Stage {
    const WIDTH: u16 = 10;
    const HEIGHT: u16 = 20;

    fn check_game_over() -> bool {
        false
    }

    fn get_line_complete() -> Vec<i32> {
        vec!()
    }
}

struct Terminal {
    stdout: io::Stdout,
}

impl Terminal {
    // FIXME: Resutが返却されるメソッド読んだあと何もしないと警告出るので一旦expectにしてる

    fn new() -> Self {
        let stdout = stdout();
        Terminal { stdout }
    }

    fn init(&mut self) {
        execute!(self.stdout, Hide);
    }

    fn clear(&mut self) {
        execute!(self.stdout, terminal::Clear(ClearType::All)).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().expect("")
    }

    // FIXME: crossterm関連は外だしたくなかったが、実装が難しいので、StyledContentを渡すようにしてる
    fn mvaddstr(&mut self, y: u16, x: u16, style: style::StyledContent<&str>) {
        queue!(
            self.stdout,
            cursor::MoveTo(x * 2, y),
            style::PrintStyledContent(style)
        )
        .unwrap();
    }
}

fn main() {
    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut terminal = Terminal::new();
    terminal.init();

    // init tetris
    init(&mut terminal);

    let mut mino = Tetrimino::new_random(4);

    let start_time = SystemTime::now();
    let mut elapsed_time = start_time.elapsed().unwrap();
    let mut before_frame_time = SystemTime::now();
    let mut last_fall_time = SystemTime::now();
    let fall_limit_time = Duration::from_millis(200);

    let mut stage = Stage {
        blocks: [[false; 10]; 20],
    };

    let mut dummy_rand : i32= 0;

    // main loop
    loop {
        elapsed_time = start_time.elapsed().unwrap();
        let delta_time = before_frame_time.elapsed().unwrap();
        before_frame_time = SystemTime::now();
        // キーボード入力
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if 'q' is pressed
                termion::event::Key::Char('q') => break,
                termion::event::Key::Esc => {}
                termion::event::Key::Left => {
                    if mino.can_move(Direction4::LEFT, &stage.blocks) {
                        mino.move_to(Direction4::LEFT);
                    }
                }
                termion::event::Key::Right => {
                    if mino.can_move(Direction4::RIGHT, &stage.blocks) {
                        mino.move_to(Direction4::RIGHT);
                    }
                }
                termion::event::Key::Up => {
                    if mino.can_rotate(&stage.blocks) {
                        mino.rotate();
                    }
                }
                termion::event::Key::Down => {
                    if mino.can_move(Direction4::DOWN, &stage.blocks) {
                        mino.move_to(Direction4::DOWN);
                    }
                }
                // Else print the pressed key
                _ => {}
            }
        }

        // update
        let last_fall_elapsed_time = last_fall_time.elapsed().unwrap();
        if (last_fall_elapsed_time > fall_limit_time) {
            if (mino.can_fall(&stage.blocks)) {
                mino.fall();
            } else {
                mino.bake_to_field(&mut stage.blocks);
                // todo: add new mino.


                dummy_rand += 1;
                dummy_rand = dummy_rand % 7;

                mino = Tetrimino::new_random(dummy_rand);
            }
            last_fall_time = SystemTime::now();
        }

        // renderering -----

        terminal.clear();
        render_wall(&mut terminal);

        render_field(&mut terminal, &stage.blocks);
        
        //render mino
        for y in 0..mino.blocks.len() {
            for x in 0..mino.blocks[0].len() {
                let global_y: i16 = y as i16 + mino.pos.1;
                let global_x: i16 = x as i16 + mino.pos.0;
                if (global_y < 0 || global_x < 0) {
                    continue;
                }
                if (mino.blocks[y as usize][x as usize]) {
                    terminal.mvaddstr(
                        global_y as u16,
                        global_x as u16 + 1, /* 左側の壁ブロック分 */
                        "██".cyan(),
                    );
                } else {
                    // terminal.mvaddstr(global_y as u16, global_x as u16, "██".grey());
                }
            }
        }

        terminal.flush();

        thread::sleep(time::Duration::from_millis(100));
    }
}

// fn print_events() -> crossterm::Result<()> {
//     loop {
//         // `poll()` waits for an `Event` for a given time period
//         if poll(Duration::from_millis(500))? {
//             // It's guaranteed that the `read()` won't block when the `poll()`
//             // function returns `true`
//             match read()? {
//                 Event::Key(event) => println!("{:?}", event),
//                 Event::Mouse(event) => println!("{:?}", event),
//                 Event::Resize(width, height) => println!("New size {}x{}", width, height),
//             }
//         } else {
//             // Timeout expired and no `Event` is available
//         }
//     }
//     Ok(())
// }

fn init(terminal: &mut Terminal) {
    terminal.clear();

    render_wall(terminal);

    terminal.flush();
}

fn main_loop(terminal: &mut Terminal) {}

fn render_wall(terminal: &mut Terminal) {
    for y in 0..(Stage::HEIGHT + 1) {
        for x in 0..(Stage::WIDTH + 2) {
            if (y == Stage::HEIGHT) || (x == 0 || x == Stage::WIDTH + 1) {
                terminal.mvaddstr(y, x, "██".magenta());
            }
        }
    }
}

fn render_field(terminal: &mut Terminal, field: &FieldBlocks) {

    for y in 0..field.len() {
        for x in 0..field[0].len() {
            if (field[y as usize][x as usize]) {
                terminal.mvaddstr(
                    y as u16,
                    x as u16 + 1, /* 左側の壁ブロック分 */
                    "██".grey(),
                );
            }
        }
    }
}
