mod state;

fn main() {
    println!("Not yet implemented");
}

// use std::collections::{HashMap, VecDeque};
// use state::{State, ZERO_STATE};

// fn main() {
//     let initial_state: State = State {
//         faces: [
//             0,0,2,0,0,0,0,0,
//             1,1,1,1,1,1,1,2,
//             2,2,3,2,2,3,2,2,
//             0,3,3,5,3,5,3,3,
//             4,4,4,4,4,4,4,4,
//             5,5,5,5,3,5,5,1
//         ]
//     };

//     println!("Initial state is: {:?}", initial_state);
//     println!("Target state is: {:?}", ZERO_STATE);

//     let path = search(initial_state, ZERO_STATE);

//     for mv in path {
//         if *mv < 6 {
//             println!("{} CW", mv)
//         } else {
//             println!("{} CCW", *mv % 6)
//         }
//     }
// }

// fn search(initial_state: State, target_state: State) -> &'static [u8] {
//     //value is a turn 0..5 - cw, 6..11 - ccw
//     let mut forward_paths: HashMap<u128, Vec<u8>> = HashMap::new();
//     forward_paths.insert(initial_state.hash(), Vec::new());
//     let mut forward_queue: VecDeque<State> = VecDeque::new();
//     forward_queue.push_back(initial_state);

//     let mut back_paths: HashMap<u128, Vec<u8>> = HashMap::new();
//     back_paths.insert(target_state.hash(), Vec::new());
//     let mut back_queue: VecDeque<State> = VecDeque::new();
//     back_queue.push_back(target_state);

//     while !forward_queue.is_empty() && !back_queue.is_empty() {
//         let forward_state = forward_queue.pop_front().unwrap();
//         let forward_state_hash = forward_state.hash();
//         if forward_paths.contains_key(&forward_state_hash) && back_paths.contains_key(&forward_state_hash) {
//             println!("Forward {:?}", forward_paths.get(&forward_state_hash));
//             println!("Back {:?}", back_paths.get(&forward_state_hash));
//             return &[];
//         } else {

//             for turn in 0..12 {
//                 let turned_state = forward_state.turn(turn % 6, turn < 6);
//                 let turned_state_hash = turned_state.hash();
//                 if forward_paths.contains_key(&turned_state_hash) && back_paths.contains_key(&turned_state_hash) {
//                     println!("Forward {:?}", forward_paths.get(&turned_state_hash));
//                     println!("Back {:?}", back_paths.get(&turned_state_hash));
//                     return &[];
//                 }
//                 if !forward_paths.contains_key(&turned_state_hash) {
//                     let mut path = Vec::new();
//                     let base_path = forward_paths.get(&forward_state_hash).unwrap();
//                     println!("Base path: {}", base_path.len());
//                     for x in base_path {
//                         path.push(*x);
//                     }
//                     path.push(turn);

//                     forward_paths.insert(turned_state.hash(), path);
//                     forward_queue.push_back(turned_state);
//                 }
//             }
//         }

//         let back_state = back_queue.pop_front().unwrap();
//         let back_state_hash = back_state.hash();
//         if forward_paths.contains_key(&back_state_hash) && back_paths.contains_key(&back_state_hash) {
//             println!("Forward {:?}", forward_paths.get(&back_state_hash));
//             println!("Back {:?}", back_paths.get(&back_state_hash));
//             return &[];
//         } else {

//             for turn in 0..12 {
//                 let turned_state = back_state.turn(turn % 6, turn < 6);
//                 let turned_state_hash = turned_state.hash();
//                 if forward_paths.contains_key(&turned_state_hash) && back_paths.contains_key(&turned_state_hash) {
//                     println!("Forward {:?}", forward_paths.get(&turned_state_hash));
//                     println!("Back {:?}", back_paths.get(&turned_state_hash));
//                     return &[];
//                 }
//                 if !back_paths.contains_key(&turned_state_hash) {
//                     let mut path = Vec::new();
//                     let base_path = back_paths.get(&back_state_hash).unwrap();
//                     println!("Base path: {}", base_path.len());
//                     for x in base_path {
//                         path.push(*x);
//                     }
//                     path.push(turn);

//                     back_paths.insert(turned_state.hash(), path);
//                     back_queue.push_back(turned_state);
//                 }
//             }
//         }


//     }

//     return &[];
// }
