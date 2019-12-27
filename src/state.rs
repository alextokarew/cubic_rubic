pub struct State {
    pub faces: [usize; 48]
}

impl State {
    pub fn turn(&self, face: u8, cw: bool) -> State {
        let mut new_faces = [0; 48];
        new_faces.copy_from_slice(&self.faces);

        let start_index: usize = (face*12).into();
        let indices = &INDICES_FROM[start_index..start_index+12];
        let shift = if cw { 15 } else { 9 };

        for i in 0..indices.len() {
            let from_index = indices[i];
            let to_index = indices[(i + shift) % 12 ];
            new_faces[to_index] = self.faces[from_index];
        }

        State {
            faces: new_faces
        }
    }
}

const FACE_NAMES: [&str; 6]  = ["W", "R", "G", "O", "B", "Y"];

const INDICES_FROM: [usize; 72] = [
     8,  9, 10, 32, 33, 34, 24, 25, 26, 16, 17, 18,
     5,  6,  7, 16, 19, 21, 42, 41, 40, 39, 36, 34,
     7,  4,  2, 24, 27, 29, 47, 44, 42, 15, 12, 10,
     2,  1,  0, 32, 35, 37, 47, 46, 45, 23, 20, 18,
     0,  3,  5,  8, 11, 13, 40, 43, 45, 31, 28, 26,
    13, 14, 15, 21, 22, 23, 29, 30, 31, 37, 38, 39
];

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
               FACE_NAMES[self.faces[0]], FACE_NAMES[self.faces[1]], FACE_NAMES[self.faces[2]],
               FACE_NAMES[self.faces[3]], FACE_NAMES[0], FACE_NAMES[self.faces[4]],
               FACE_NAMES[self.faces[5]], FACE_NAMES[self.faces[6]], FACE_NAMES[self.faces[7]],

               FACE_NAMES[self.faces[8]], FACE_NAMES[self.faces[9]], FACE_NAMES[self.faces[10]],
               FACE_NAMES[self.faces[16]], FACE_NAMES[self.faces[17]], FACE_NAMES[self.faces[18]],
               FACE_NAMES[self.faces[24]], FACE_NAMES[self.faces[25]], FACE_NAMES[self.faces[26]],
               FACE_NAMES[self.faces[32]], FACE_NAMES[self.faces[33]], FACE_NAMES[self.faces[34]],

               FACE_NAMES[self.faces[11]],FACE_NAMES[1], FACE_NAMES[self.faces[12]],
               FACE_NAMES[self.faces[19]],FACE_NAMES[2], FACE_NAMES[self.faces[20]],
               FACE_NAMES[self.faces[27]],FACE_NAMES[3], FACE_NAMES[self.faces[28]],
               FACE_NAMES[self.faces[35]],FACE_NAMES[4], FACE_NAMES[self.faces[36]],

               FACE_NAMES[self.faces[13]], FACE_NAMES[self.faces[14]], FACE_NAMES[self.faces[15]],
               FACE_NAMES[self.faces[21]], FACE_NAMES[self.faces[22]], FACE_NAMES[self.faces[23]],
               FACE_NAMES[self.faces[29]], FACE_NAMES[self.faces[30]], FACE_NAMES[self.faces[31]],
               FACE_NAMES[self.faces[37]], FACE_NAMES[self.faces[38]], FACE_NAMES[self.faces[39]],

               FACE_NAMES[self.faces[40]], FACE_NAMES[self.faces[41]], FACE_NAMES[self.faces[42]],
               FACE_NAMES[self.faces[43]], FACE_NAMES[5], FACE_NAMES[self.faces[44]],
               FACE_NAMES[self.faces[45]], FACE_NAMES[self.faces[46]], FACE_NAMES[self.faces[47]]
        )
    }
}
