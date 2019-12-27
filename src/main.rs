mod state;

fn main() {
    let initial_state: state::State = state::State {
        faces: [
            0,0,0,0,0,0,0,0,
            1,1,1,1,1,1,1,1,
            2,2,2,2,2,2,2,2,
            3,3,3,3,3,3,3,3,
            4,4,4,4,4,4,4,4,
            5,5,5,5,5,5,5,5
        ]
    };

    println!("Initial state is: {:?}", initial_state);
    println!("Turn 0 CW: {:?}", initial_state.turn(0, true));
    println!("Turn 1 CW: {:?}", initial_state.turn(1, true));
    println!("Turn 2 CW: {:?}", initial_state.turn(2, true));
    println!("Turn 3 CW: {:?}", initial_state.turn(3, true));
    println!("Turn 4 CW: {:?}", initial_state.turn(4, true));
    println!("Turn 5 CW: {:?}", initial_state.turn(5, true));

    println!("Turn 0 CCW: {:?}", initial_state.turn(0, false));
    println!("Turn 1 CCW: {:?}", initial_state.turn(1, false));
    println!("Turn 2 CCW: {:?}", initial_state.turn(2, false));
    println!("Turn 3 CCW: {:?}", initial_state.turn(3, false));
    println!("Turn 4 CCW: {:?}", initial_state.turn(4, false));
    println!("Turn 5 CCW: {:?}", initial_state.turn(5, false));
}

