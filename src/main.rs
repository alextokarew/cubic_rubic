use Face::*;


fn main() {

    let initial_state: State = State {
        faces: [
            W,W,W,W,W,W,W,W,
            R,R,R,R,R,R,R,R,
            G,G,G,G,G,G,G,G,
            O,O,O,O,O,O,O,O,
            B,B,B,B,B,B,B,B,
            Y,Y,Y,Y,Y,Y,Y,Y
        ]
    };

    println!("Side w is: {:?}", W);
    println!("Side w+1 is: {:?}", W + 1);
    println!("Side w-1 is: {:?}", W - 1);
    println!("Side y+1 is: {:?}", Y + 1);
    println!("Side y-1 is: {:?}", Y - 1);

    println!("Initial state is: {:?}", initial_state);
    println!("New state is: {:?}", turn(initial_state))

}

fn turn(state: State) -> State {
    
    state
}

#[derive(Debug)]
enum Face {
    W, R, G, O, B, Y
}

impl std::ops::Add<i32> for Face {
    type Output = Face;

    fn add(self, _num: i32) -> Face {
        W        
    }
}

impl std::ops::Sub<i32> for Face {
    type Output = Face;

    fn sub(self, _num: i32) -> Face {
        self + (- _num)
    }
}


struct State {
    faces: [Face; 48]
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
                "\n\
                 {:?}{:?}{:?}\n\
                 {:?}{:?}{:?}\n\
                 {:?}{:?}{:?}\n\
                 \n\
                 {:?}{:?}{:?} {:?}{:?}{:?} {:?}{:?}{:?} {:?}{:?}{:?}\n\
                 {:?}{:?}{:?} {:?}{:?}{:?} {:?}{:?}{:?} {:?}{:?}{:?}\n\
                 {:?}{:?}{:?} {:?}{:?}{:?} {:?}{:?}{:?} {:?}{:?}{:?}\n\
                 \n\
                 {:?}{:?}{:?}\n\
                 {:?}{:?}{:?}\n\
                 {:?}{:?}{:?}", 
                self.faces[0], self.faces[1], self.faces[2], 
                self.faces[3],             W, self.faces[4], 
                self.faces[5], self.faces[6], self.faces[7],

                self.faces[8], self.faces[9], self.faces[10], 
                self.faces[16], self.faces[17], self.faces[18], 
                self.faces[24], self.faces[25], self.faces[26],
                self.faces[32], self.faces[33], self.faces[34], 
                
                self.faces[11],             R, self.faces[12], 
                self.faces[19],             G, self.faces[20], 
                self.faces[27],             O, self.faces[28], 
                self.faces[35],             B, self.faces[36], 
                
                self.faces[13], self.faces[14], self.faces[15],
                self.faces[21], self.faces[22], self.faces[23],
                self.faces[29], self.faces[30], self.faces[31],
                self.faces[37], self.faces[38], self.faces[39],

                self.faces[40], self.faces[41], self.faces[42], 
                self.faces[43],             Y, self.faces[44], 
                self.faces[45], self.faces[46], self.faces[47],
            )
    }
}

