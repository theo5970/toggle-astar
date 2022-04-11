
use super::utils::BitArray;

//
// Button Function
//
#[derive(Debug, Clone, Copy)]
pub enum ButtonFunction {
    Nothing,
    OneArrow(OneArrow),
    TwoArrow(TwoArrow),
    FourArrow,
    Rotate(bool),
    Symmetry(bool),
    Shift(bool),
    AroundEight,
}

#[derive(Debug, Clone, Copy)]
pub enum OneArrow {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

#[derive(Debug, Clone, Copy)]
pub enum TwoArrow {
    BothHorizontal,
    BothVertical,
    LeftUpRightDown,
    LeftDownRightUp,
}

//
// Coordinate
//
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}

//
// Button
//
#[derive(Debug)]
pub struct Button {
    pub coord: Coordinate,
    pub is_on: bool,
    pub func: ButtonFunction,
}

impl Button {
    fn new(x: i32, y: i32) -> Button {
        Button {
            coord: Coordinate::new(x, y),
            is_on: false,
            func: ButtonFunction::Nothing,
        }
    }

    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}

//
// Grid
//
#[derive(Debug)]
pub struct Grid {
    buttons: Vec<Button>,
    width: i32,
    height: i32,
}

fn convert_subtype_to_func(subtype: u8) -> ButtonFunction {
    match subtype {
        // One Arrow Linear
        1 => ButtonFunction::OneArrow(OneArrow::Up),
        2 => ButtonFunction::OneArrow(OneArrow::Down),
        3 => ButtonFunction::OneArrow(OneArrow::Left),
        4 => ButtonFunction::OneArrow(OneArrow::Right),

        // One Arrow Diagonal
        5 => ButtonFunction::OneArrow(OneArrow::LeftUp),
        6 => ButtonFunction::OneArrow(OneArrow::RightUp),
        7 => ButtonFunction::OneArrow(OneArrow::LeftDown),
        8 => ButtonFunction::OneArrow(OneArrow::RightDown),

        // Two Arrow Linear
        9 => ButtonFunction::TwoArrow(TwoArrow::BothHorizontal),
        10 => ButtonFunction::TwoArrow(TwoArrow::BothVertical),

        // Four Arrow
        11 => ButtonFunction::FourArrow,

        // Two Arrow Diagonal
        12 => ButtonFunction::TwoArrow(TwoArrow::LeftUpRightDown),
        13 => ButtonFunction::TwoArrow(TwoArrow::LeftDownRightUp),

        // Rotate
        14 => ButtonFunction::Rotate(true),  // Clockwise
        15 => ButtonFunction::Rotate(false), // Counter-Clockwise

        // Symmetry
        16 => ButtonFunction::Symmetry(true),  // Horizontal
        17 => ButtonFunction::Symmetry(false), // Vertical

        // Shift
        18 => ButtonFunction::Shift(false), // Left
        19 => ButtonFunction::Shift(true),  // Right

        // Around Eight
        20 => ButtonFunction::AroundEight,

        // Unknown
        _ => ButtonFunction::Nothing,
    }
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Grid {
        let total_buttons = width * height;
        let mut result = Grid {
            buttons: Vec::with_capacity(total_buttons as usize),
            width,
            height,
        };

        for y in 0..height {
            for x in 0..width {
                result.buttons.push(Button::new(x, y))
            }
        }

        result
    }

    pub fn from_level(level: &ToggleLevel) -> Grid {
        let mut result = Grid::new(
            level.width.try_into().unwrap(),
            level.height.try_into().unwrap(),
        );

        for y in 0..level.height {
            for x in 0..level.width {
                let state_index = usize::try_from(x * level.height + y).unwrap();
                let subtype_index = usize::try_from(y * level.width + x).unwrap();

                let btn = result.at_mut(x as i32, y as i32).unwrap();

                btn.is_on = level.states.get(state_index);
                btn.func = convert_subtype_to_func(level.subtypes[subtype_index]);
            }
        }
        result
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&Button> {
        let index = y * self.width + x;
        self.buttons.get(index as usize)
    }

    pub fn at_mut(&mut self, x: i32, y: i32) -> Option<&mut Button> {
        let index = y * self.width + x;
        self.buttons.get_mut(index as usize)
    }

    pub fn click(&mut self, x: i32, y: i32) {
        self.click_button(x, y)
    }

    pub fn check_range(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.width) && (y >= 0 && y < self.height)
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let inv_y = self.height - y - 1;
                let btn = self.at(x, inv_y).unwrap();

                print!("{} ", if btn.is_on { 'O' } else { '.' });
            }
            println!();
        }
    }

    pub fn get_states(&self) -> BitArray {
        let mut result = BitArray::new((self.width * self.height).try_into().unwrap());

        let mut i = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let btn = self.at(x, y).unwrap();
                result.set(i, btn.is_on);
                i+= 1;
            }
        }

        result
    }

    pub fn set_states(&mut self, b: &BitArray) {
        let mut i = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let btn = self.at_mut(x, y).unwrap();
                btn.is_on = b.get(i);
                i+= 1;
            }
        }
    }

    pub fn set_all_state(&mut self, state: bool) {
        for i in 0..self.buttons.len() {
            self.buttons[i].is_on = state;
        }
    }

    // Clicks an button
    fn click_button(&mut self, x: i32, y: i32) {
        let btn = self.at_mut(x, y).unwrap();

        match btn.func {
            ButtonFunction::Nothing => {}
            ButtonFunction::OneArrow(dir) => {
                self.solve_onearrow(x, y, dir);
            }
            ButtonFunction::TwoArrow(dir) => {
                self.solve_twoarrow(x, y, dir);
            }
            ButtonFunction::FourArrow => {
                self.solve_fourarrow(x, y);
            }
            ButtonFunction::Rotate(is_clockwise) => {
                self.solve_rotate(x, y, is_clockwise);
            }
            ButtonFunction::Shift(is_right) => {
                self.solve_shift(y, is_right);
            }
            ButtonFunction::Symmetry(is_horizontal) => {
                self.solve_symmetry(x, y, is_horizontal);
            }
            ButtonFunction::AroundEight => {
                self.solve_aroundeight(x, y);
            }
        }
    }

    fn solve_onearrow(&mut self, x: i32, y: i32, dir: OneArrow) {
        let mut dx = 0;
        let mut dy = 0;

        match dir {
            OneArrow::Up => dy = 1,
            OneArrow::Down => dy = -1,
            OneArrow::Left => dx = -1,
            OneArrow::Right => dx = 1,
            OneArrow::LeftUp => {
                dx = -1;
                dy = 1
            }
            OneArrow::RightUp => {
                dx = 1;
                dy = 1
            }
            OneArrow::LeftDown => {
                dx = -1;
                dy = -1
            }
            OneArrow::RightDown => {
                dx = 1;
                dy = -1
            }
        }

        self.toggle_follow_direction(x, y, dx, dy);
    }

    fn solve_twoarrow(&mut self, x: i32, y: i32, dir: TwoArrow) {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;

        match dir {
            TwoArrow::BothHorizontal => {
                dx = 1;
            },
            TwoArrow::BothVertical => {
                dy = 1;
            },
            TwoArrow::LeftUpRightDown => {
                dx = -1;
                dy = 1;
            }
            TwoArrow::LeftDownRightUp => {
                dx = -1;
                dy = -1;
            }
        }

        let btn = self.at_mut(x, y).unwrap();
        btn.toggle();

        self.toggle_follow_direction(x + dx, y + dy, dx, dy);
        self.toggle_follow_direction(x - dx, y - dy, -dx, -dy);
    }

    fn toggle_follow_direction(&mut self, start_x: i32, start_y: i32, dx: i32, dy: i32) {
        let mut x2 = start_x;
        let mut y2 = start_y;

        while self.check_range(x2, y2) {
            let btn = self.at_mut(x2, y2).unwrap();
            btn.toggle();

            x2 += dx;
            y2 += dy;
        }
    }

    fn solve_fourarrow(&mut self, x: i32, y: i32) {
        let btn = self.at_mut(x, y).unwrap();
        btn.toggle();

        for x2 in 0..self.width() {
            if x == x2 {
                continue;
            }

            let btn = self.at_mut(x2, y).unwrap();
            btn.toggle();
        }

        for y2 in 0..self.height() {
            if y == y2 {
                continue;
            }
            let btn = self.at_mut(x, y2).unwrap();
            btn.toggle();
        }
    }

    fn solve_rotate(&mut self, x: i32, y: i32, is_clockwise: bool) {
        let directions = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let btn = self.at_mut(x, y).unwrap();
        btn.toggle();

        let mut old_states: [bool; 8] = [false; 8];
        let mut new_states: [bool; 8] = [false; 8];

        for i in 0..8 {
            let x2 = x + directions[i].0;
            let y2 = y + directions[i].1;

            if self.check_range(x2, y2) {
                let btn = self.at(x2, y2).unwrap();
                old_states[i] = btn.is_on;
            }
        }

        if is_clockwise {
            new_states[0] = old_states[7];

            for i in 1..8 {
                new_states[i] = old_states[i - 1];
            }
        } else {
            new_states[7] = old_states[0];

            for i in 0..7 {
                new_states[i] = old_states[i + 1];
            }
        }

        for i in 0..8 {
            let x2 = x + directions[i].0;
            let y2 = y + directions[i].1;

            if self.check_range(x2, y2) {
                let btn = self.at_mut(x2, y2).unwrap();
                btn.is_on = new_states[i];
            }
        }
    }

    fn solve_shift(&mut self, y: i32, is_right: bool) {
        let mut old_states: [bool; 16] = [false; 16];
        let mut new_states: [bool; 16] = [false; 16];

        for i in 0..self.width {
            let btn = self.at(i, y).unwrap();
            old_states[i as usize] = btn.is_on;
        }

        if is_right {
            new_states[0] = old_states[(self.width - 1) as usize];
            for i in 1..self.width {
                new_states[i as usize] = old_states[(i - 1) as usize];
            }
        } else {
            new_states[(self.width - 1) as usize] = old_states[0];
            for i in 1..(self.width - 1) {
                new_states[i as usize] = old_states[(i + 1) as usize];
            }
        }

        for i in 0..self.width {
            let btn = self.at_mut(i, y).unwrap();
            btn.is_on = new_states[i as usize];
        }
    }

    fn solve_symmetry(&mut self, x: i32, y: i32, is_horizontal: bool) {
        let btn = self.at_mut(x, y).unwrap();
        btn.toggle();

        if is_horizontal {
            let left_x = x - 1;
            let right_x = x + 1;

            let exists_left = self.check_range(left_x, y);
            let exists_right = self.check_range(right_x, y);
            for y2 in 0..self.height {

                if exists_left && exists_right {
                    let left_state = self.at(left_x, y2).unwrap().is_on;
                    let right_state = self.at(right_x, y2).unwrap().is_on;

                    self.at_mut(right_x, y2).unwrap().is_on = left_state;
                    self.at_mut(left_x, y2).unwrap().is_on = right_state;

                } else if exists_left {
                    self.at_mut(left_x, y2).unwrap().is_on = false;

                } else if exists_right {
                    self.at_mut(right_x, y2).unwrap().is_on = false;
                }
            }
        } else {
            let up_y = y + 1;
            let down_y = y - 1;

            let exists_up = self.check_range(x, up_y);
            let exists_down = self.check_range(x, down_y);

            for x2 in 0..self.width {
                if exists_up && exists_down {
                    let up_state = self.at(x2, up_y).unwrap().is_on;
                    let down_state = self.at(x2, down_y).unwrap().is_on;

                    self.at_mut(x2, up_y).unwrap().is_on = down_state;
                    self.at_mut(x2, down_y).unwrap().is_on = up_state;
                } else if exists_up {
                    self.at_mut(x2, up_y).unwrap().is_on = false;

                } else if exists_down {
                    self.at_mut(x2, down_y).unwrap().is_on = false;
                }
            }
        }
    }

    fn solve_aroundeight(&mut self, x: i32, y: i32) {
        let directions = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let btn = self.at_mut(x, y).unwrap();
        btn.toggle();

        for i in 0..8 {
            let x2 = x + directions[i].0;
            let y2 = y + directions[i].1;

            if self.check_range(x2, y2) {
                let btn = self.at_mut(x2, y2).unwrap();
                btn.toggle();
            }
        }
    }
}

//
// Toggle Level
//
#[derive(Debug)]
pub struct ToggleLevel {
    pub width: u32,
    pub height: u32,
    pub subtypes: Vec<u8>,
    pub states: BitArray,
    pub min_clicks: u32,
}

impl ToggleLevel {
    pub fn new() -> ToggleLevel {
        ToggleLevel {
            width: 0,
            height: 0,
            min_clicks: 0,
            subtypes: Vec::new(),
            states: BitArray::new(0),
        }
    }
}