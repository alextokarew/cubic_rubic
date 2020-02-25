
//Face is represented as 8-digit hexadecimal number, where each digit is equal to 0..5, corresponding to face name
//Digits are placed clockwise, starting from top left corner.
pub type Face = u32;

pub type Faces = [Face; 6];

pub struct State {
    pub faces: Faces
}

impl State {
    pub const fn new(faces: Faces) -> State {
        State {
            faces: faces
        }
    }

    pub fn turn(&self, face: usize, cw: bool) -> State {
        let mut new_faces = self.faces.clone();
        let sides = AFFECTED_SIDES[face];
        let shift = if cw { 1 } else { 3 };
        for i in 0..sides.len() {
            let (from_face, from_digit_start) = sides[i];
            let (to_face, to_digit_start) = sides[(i + shift) % 4];

            let edge = if from_digit_start == 6 {
                self.faces[from_face] << 24 | self.faces[from_face] >> 8
            } else {
                self.faces[from_face] << from_digit_start * 4
            } & 0xFFF00000;

            new_faces[to_face] = if to_digit_start == 6 {
                (self.faces[to_face] & 0x0FFFFF00) | edge >> 24 | edge << 8
            } else {
                let to_shift = to_digit_start * 4;
                (self.faces[to_face] & !(0xFFF00000 >> to_shift)) | edge >> to_shift
            };
        }

        let new_face_value: Face = if cw {
            self.faces[face] >> 8 | self.faces[face] << 24
        } else {
            self.faces[face] << 8 | self.faces[face] >> 24
        };

        new_faces[face] = new_face_value;

        State::new(new_faces)
    }

    pub fn hash(&self) -> u128 {
        self.faces.iter().fold(0, |acc, x| acc*6 + (*x as u128))
    }

    pub fn color(&self, index: usize) -> &str {
        let face_index = index / 8;
        let shift = (7 - index % 8) * 4;
        let face = self.faces[face_index];

        let color_code: usize = (((face >> shift) & 0xF) as u8).into();

        FACE_COLORS[color_code]
    }
}

const FACE_COLORS: [&str; 6]  = [
    "\x1b[48;2;255;255;255;30mW\x1b[0m",
    "\x1b[48;2;255;0;0;30mR\x1b[0m",
    "\x1b[48;2;0;0;255;30mB\x1b[0m",
    "\x1b[48;2;255;127;0;30mO\x1b[0m",
    "\x1b[48;2;0;255;0;30mG\x1b[0m",
    "\x1b[48;2;255;255;0;30mY\x1b[0m"
];

const AFFECTED_SIDES: [[(usize, u8); 4]; 6] = [
    [(3, 0), (2, 0), (1, 0), (4, 0)],
    [(0, 4), (2, 6), (5, 0), (4, 2)],
    [(0, 2), (3, 6), (5, 2), (1, 2)],
    [(0, 0), (4, 6), (5, 4), (2, 2)],
    [(0, 6), (1, 6), (5, 6), (3, 2)],
    [(1, 4), (2, 4), (3, 4), (4, 4)]
];

pub const ZERO_STATE: State = State::new([
    0x00000000,
    0x11111111,
    0x22222222,
    0x33333333,
    0x44444444,
    0x55555555
]);

impl std::fmt::Debug for State {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "\n\
                 {}{}{}\n\
                 {}{}{}\n\
                 {}{}{}\n\
                 \n\
                 {}{}{} {}{}{} {}{}{} {}{}{}\n\
                 {}{}{} {}{}{} {}{}{} {}{}{}\n\
                 {}{}{} {}{}{} {}{}{} {}{}{}\n\
                 \n\
                 {}{}{}\n\
                 {}{}{}\n\
                 {}{}{}",
               self.color(0),  self.color(1), self.color(2),
               self.color(7), FACE_COLORS[0], self.color(3),
               self.color(6),  self.color(5), self.color(4),

               self.color(8),   self.color(9), self.color(10),
               self.color(16), self.color(17), self.color(18),
               self.color(24), self.color(25), self.color(26),
               self.color(32), self.color(33), self.color(34),

               self.color(15), FACE_COLORS[1], self.color(11),
               self.color(23), FACE_COLORS[2], self.color(19),
               self.color(31), FACE_COLORS[3], self.color(27),
               self.color(39), FACE_COLORS[4], self.color(35),

               self.color(14), self.color(13), self.color(12),
               self.color(22), self.color(21), self.color(20),
               self.color(30), self.color(29), self.color(28),
               self.color(38), self.color(37), self.color(36),

               self.color(40), self.color(41), self.color(42),
               self.color(47), FACE_COLORS[5], self.color(43),
               self.color(46), self.color(45), self.color(44)
        )
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_debug() {
        let result = format!("{:?}", ZERO_STATE);
        let expected = "\n\
        WWW\n\
        WWW\n\
        WWW\n\
        \n\
        RRR BBB OOO GGG\n\
        RRR BBB OOO GGG\n\
        RRR BBB OOO GGG\n\
        \n\
        YYY\n\
        YYY\n\
        YYY";
        assert_eq!(expected, result);
    }

    // #[test]
    // fn test_hash() {
    //     assert_eq!(0, ZERO_STATE.hash());
    // }

    #[test]
    fn test_eq() {
        let another_zero = State::new([
            0x00000000,
            0x11111111,
            0x22222222,
            0x33333333,
            0x44444444,
            0x55555555
        ]);

        assert_eq!(ZERO_STATE, another_zero);
    }

    #[test]
    fn test_turn_top_right() {
        let turned = ZERO_STATE.turn(0, true);
        println!("Turned top right: {:?}", turned);

        let expected = State::new([
            0x00000000,
            0x22211111,
            0x33322222,
            0x44433333,
            0x11144444,
            0x55555555
        ]);

        assert_eq!(expected, turned);
    }

    #[test]
    fn test_turn_top_left() {
        let turned = ZERO_STATE.turn(0, false);
        println!("Turned top left: {:?}", turned);

        let expected = State::new([
            0x00000000,
            0x44411111,
            0x11122222,
            0x22233333,
            0x33344444,
            0x55555555
        ]);

        assert_eq!(expected, turned);
    }

    #[test]
    fn test_turn_front_right() {
        let turned = ZERO_STATE.turn(1, true);
        println!("Turned front right: {:?}", turned);

        let expected = State::new([
            0x00004440,
            0x11111111,
            0x02222200,
            0x33333333,
            0x44555444,
            0x22255555
        ]);

        assert_eq!(expected, turned);
    }

    #[test]
    fn test_turn_front_left() {
        let turned = ZERO_STATE.turn(1, false);
        println!("Turned front left: {:?}", turned);
        
        let expected = State::new([
            0x00002220,
            0x11111111,
            0x52222255,
            0x33333333,
            0x44000444,
            0x44455555
        ]);

        assert_eq!(expected, turned);
    }

    #[test]
    fn test_turn_bottom_right() {
        let turned = ZERO_STATE.turn(5, true);
        println!("Turned front right: {:?}", turned);

        let expected = State::new([
            0x00000000,
            0x11114441,
            0x22221112,
            0x33332223,
            0x44443334,
            0x55555555
        ]);

        assert_eq!(expected, turned);
    }

    #[test]
    fn test_turn_bottom_left() {
        let turned = ZERO_STATE.turn(5, false);
        println!("Turned front left: {:?}", turned);
        
        let expected = State::new([
            0x00000000,
            0x11112221,
            0x22223332,
            0x33334443,
            0x44441114,
            0x55555555
        ]);

        assert_eq!(expected, turned);
    }

    #[test]
    fn test_series_of_100_random_turns() {
        extern crate rand;
        use rand::prelude::*;
        
        let turns: Vec<(usize, bool)> = {
            let mut vec = Vec::new();
            let mut rng = thread_rng();
            for _ in 0..100 {
                let turn = (rng.gen_range(0, 6), rng.gen());
                vec.push(turn);
            }
            vec
        };

        println!("{} turns are: {:?}", turns.len(), turns);

        let mut turned = ZERO_STATE;
        for (face, cw) in &turns {
            turned = turned.turn(*face, *cw);
            println!("Turn {} - {}:{:?}\n", face, cw, turned);

        }

        for (face, cw) in turns.iter().rev() {
            turned = turned.turn(*face, !cw);
            println!("Turn {} - {}:{:?}\n", face, !cw, turned);

        }
        assert_eq!(ZERO_STATE, turned);
    }
}