pub use {
    project_decmac::{
        min,
        max,
        min_max,
        pipeline,
        returns,
    },
};

#[inline(always)]
pub const fn returns<T>(value: T) -> impl FnOnce() -> T {
    returns!(value)
}

#[inline(always)]
pub const fn returns_clone<T: Clone>(value: T) -> impl Fn() -> T {
    returns!(value.clone())
}

#[inline(always)]
pub const fn returns_copy<T: Copy>(value: T) -> impl Fn() -> T {
    returns!(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn history_test() {
        #[derive(Debug, Clone)]
        struct Data {
            pub name: &'static str,
            pub age: u64,
        }
        impl Data {
            pub const fn new(name: &'static str, age: u64) -> Self {
                Self { name, age }
            }
        }
        
        struct History<T> {
            stack: Vec<Box<dyn Fn() -> T + 'static>>,
        }
        
        impl<T> History<T> {
            #[must_use]
            #[inline(always)]
            pub const fn new() -> Self {
                Self { stack: Vec::new() }
            }
            
            pub fn push<F: Fn() -> T + 'static>(&mut self, f: F) {
                self.stack.push(Box::new(f));
            }
            
            pub fn last(&self) -> Option<T> {
                self.stack.last().map(|callback| (callback)())
            }
            
            pub fn pop(&mut self) -> Option<T> {
                self.stack.pop().map(|callback| (callback)())
            }
        }
        let mut history = History::new();
        history.push(returns_clone(Data::new("Ada", 32)));
        history.push(returns_clone(Data::new("Ada", 32)));
        history.push(returns_clone(Data::new("Ada", 32)));
        if let Some(data) = history.pop() {
            assert_eq!((data.name, data.age), ("Ada", 32));
        }
        let mut index = 0;
        while let Some(top) = history.last() && index < 50 {
            assert_eq!((top.name, top.age), ("Ada", 32));
            index += 1;
        }
        assert_eq!(index, 50);
    }
}