mod state;

use state::{State, ZERO_STATE};
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::spawn;

fn main() {
    let cpus = num_cpus::get();
    println!("Number of CPUs: {}", cpus);

    let initial_state: State = State::new([
        0x00532000,
        0x11311111,
        0x02321202,
        0x23333320,
        0x44444444,
        0x55255555
    ]);

    println!("Initial state is: {:?}", initial_state);
    println!("Target state is: {:?}", ZERO_STATE);

    // for num_cores in 1..=35 {
    //     analyse_cores(num_cores, &initial_state);
    //     analyse_cores(num_cores, &ZERO_STATE);
    // }

    let path = search(&initial_state, &ZERO_STATE);

    let mut current_state = initial_state;

    for turn in path {
        println!("Turn {} {}", turn % 6, turn < 6);
        current_state = current_state.turn(turn % 6, turn < 6);
        println!("{:?}", current_state);
    }

    assert_eq!(current_state, ZERO_STATE);
}

fn analyse_cores(num_cores: u128, initial_state: &State) {
    let mut rem_counter: Vec<u32> = Vec::new();
    for i in 0..num_cores {
        rem_counter.push(0);
    }

    // println!("Initial state reminder: {}", initial_state.hash % num_cores);
    rem_counter[(initial_state.hash % num_cores) as usize] += 1;
    
    for i in 0..12 {
        let turn1 = initial_state.turn(i % 6, i < 6);
        // println!("Turn: {}, Reminder: {}", i, turn1.hash % num_cores);
        rem_counter[(turn1.hash % num_cores) as usize] += 1;

        for j in 0..12 {
            let turn2 = turn1.turn(j % 6, j < 6);
            // println!("Turn: {} - {}, Reminder: {}", i, j, turn2.hash % num_cores);
            rem_counter[(turn2.hash % num_cores) as usize] += 1;
        }
    }

    println!("For {} cores remainders count are {:?}", num_cores, rem_counter);
}

fn search(initial_state: &State, target_state: &State) -> Vec<usize> {
    //value is a turn 0..5 - cw, 6..11 - ccw
    let mut forward_paths: HashMap<u128, usize> = HashMap::new();
    forward_paths.insert(initial_state.hash, 255);
    let mut forward_queue: VecDeque<u128> = VecDeque::new();
    forward_queue.push_back(initial_state.hash);

    let mut back_paths: HashMap<u128, usize> = HashMap::new();
    back_paths.insert(target_state.hash, 255);
    let mut back_queue: VecDeque<u128> = VecDeque::new();
    back_queue.push_back(target_state.hash);

    while !forward_queue.is_empty() && !back_queue.is_empty() {
        let forward_state_hash = forward_queue.pop_front().unwrap();
        if forward_paths.contains_key(&forward_state_hash) && back_paths.contains_key(&forward_state_hash) {
            return create_search_result(forward_state_hash, forward_paths, back_paths);
        } else {
            process(forward_state_hash, &mut forward_paths, &mut forward_queue);
        }

        let back_state_hash = back_queue.pop_front().unwrap();
        if forward_paths.contains_key(&back_state_hash) && back_paths.contains_key(&back_state_hash) {
            return create_search_result(back_state_hash, forward_paths, back_paths);
        } else {
            process(back_state_hash, &mut back_paths, &mut back_queue);   
        }
    }

    return vec![];
}

fn create_search_result(state_hash: u128, forward_paths: HashMap<u128, usize>, back_paths: HashMap<u128, usize>) -> Vec<usize> {
    let mut result = VecDeque::<usize>::new();
    let mut state = State::from_hash(state_hash);

    while *forward_paths.get(&state.hash).unwrap() != 255 {
        let turn = forward_paths.get(&state.hash).unwrap();
        println!("Forward {}", turn);
        result.push_front(*turn);
        state = state.turn(*turn % 6, *turn >= 6);
    }

    state = State::from_hash(state_hash);

    while *back_paths.get(&state.hash).unwrap() != 255 {
        let turn = back_paths.get(&state.hash).unwrap();
        println!("Back {}", turn);
        let reversed_turn = if *turn >= 6 { *turn - 6 } else { *turn + 6 };
        result.push_back(reversed_turn);
        state = state.turn(*turn % 6, *turn >= 6);
    }
    
    result.into_iter().collect()
}

struct Moves {
    state_hash: u128,
    steps: Vec<u8>
}

fn start(num_cores: u32) {
    let mut senders = Vec::<Sender<Moves>>::new();

    for thread_number in 0..num_cores {
        let (sender, receiver) = channel::<Moves>();
        senders.push(sender);


        spawn(move || {
            process_thread(thread_number, receiver);
        });
    }
}

fn process_thread(thread_number: u32, receiver: Receiver<Moves>) {
    let mut forward_paths: HashMap<u128, Vec<u8>> = HashMap::new();
    let mut back_paths: HashMap<u128, Vec<u8>> = HashMap::new();
}

fn select_thread(state_hash: u128) -> usize {
    (state_hash % 8) as usize
}

fn process(state_hash: u128, paths: &mut HashMap<u128, usize>, queue: &mut VecDeque<u128>) {
    let state = State::from_hash(state_hash);
    for turn in 0..12 {
        let turned_state = state.turn(turn % 6, turn < 6);
        let turned_state_hash = turned_state.hash;
        if !paths.contains_key(&turned_state_hash) {
            paths.insert(turned_state.hash, turn);
            queue.push_back(turned_state.hash);
        }
    }
}