use crate::linked_list::LinkedList;

pub mod linked_list;

fn main() {
    // Edge Case 1: Empty List
    let mut empty_list: LinkedList<i32> = LinkedList::new();
    println!("Empty list iter_mut:");
    for item in empty_list.iter_mut() {
        println!("{item}"); // Should not print anything
    }

    // Edge Case 2: Single Element List
    let mut single = LinkedList::new();
    single.push_front(42);
    println!("Single element list:");
    for val in single.iter_mut() {
        println!("Before: {val}");
        *val *= 2;
        println!("After: {val}");
    }

    // Edge Case 3: Multiple Elements
    let mut list = LinkedList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Original list:");
    for val in list.iter_mut() {
        print!("{val} ");
    }
    println!();

    // Mutate values
    for val in list.iter_mut() {
        *val += 1;
    }

    println!("After +1 mutation:");
    for val in list.iter_mut() {
        print!("{val} ");
    }
    println!();

    // Further mutation
    for val in list.iter_mut() {
        *val *= 10;
    }

    println!("After *10 mutation:");
    for val in list.iter_mut() {
        print!("{val} ");
    }
    println!();
}
