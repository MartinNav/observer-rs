use std::ops::{Deref, DerefMut};

pub struct ActionObserver<'a, T: std::clone::Clone> {
    pub value: T,
    previous: T,
    func: &'a mut dyn FnMut(&T, &mut T) -> (),
}
impl<'a, T: std::clone::Clone> ActionObserver<'a, T> {
    fn new(value: T, handle_function: &'static mut dyn FnMut(&T, &mut T) -> ()) -> Self {
        ActionObserver {
            value: value.clone(),
            previous: value,
            func: handle_function,
        }
    }
}
impl<'a, T: std::clone::Clone> Deref for ActionObserver<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<'a, T: std::clone::Clone + std::cmp::PartialEq> DerefMut for ActionObserver<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.value != self.previous {
            (self.func)(&self.previous, &mut self.value);
        }
        self.previous = self.value.clone();
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use std::{i32, ptr::addr_of_mut};

    use super::*;

    #[test]
    fn test1() {
        static mut outside_the_scope_var: i32 = 0;
        static mut function: fn(&i32, &mut i32) -> () = |prev: &i32, curr: &mut i32| unsafe {
            outside_the_scope_var = *prev;
        };
        let mut val = ActionObserver::new(5, unsafe { &mut function });
        *val = 3;
        unsafe {
            assert_eq!(outside_the_scope_var, 5);
        }
        assert_eq!(*val, 3);
    }
}
