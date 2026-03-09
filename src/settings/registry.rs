//! The registry is where all Settings types must be registered at the start of the program.
//! New settings types should be registered after old ones.

/*
struct Registry {
    types: Vec<Type>,
    ids_keys: HashMap<Id, (u64, &'static str)>,
    keys_ids: HashMap<&'static str, (u64, Id)>,
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Key(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Id(u32);

struct KeySlot {
    index: usize,
    key: Key,
}

struct IdSlot {
    index: usize,
    id: Id,
}

pub struct Registry {
    
}