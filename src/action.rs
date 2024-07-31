use std::ops::{Deref, DerefMut};
/// This struct allows execution of specified function before the value of given variable is
/// changed. Basically it is executing your function on mutable deffer
pub struct ActionObserver<'a, T: std::clone::Clone> {
    value: T,
    func: &'a mut dyn FnMut(&mut T) -> (),
}
impl<'a, T: std::clone::Clone> ActionObserver<'a, T> {
    /// Possible usecases are for debug logging when you need to see variable value every time
    /// before it is changed or changing the value if needed
    /// ```rust
    /// use observer::action::ActionObserver;
    /// static mut logging_fn: fn(&mut i32)->() = |val: &mut i32|{
    ///     println!("Current value is:{}",val);
    /// };
    /// let mut val = ActionObserver::new(1, unsafe{ &mut logging_fn});
    /// //now we can modify the value as we wish
    /// *val = 5;
    /// // you should see 'Current value is:1' in your terminal
    /// ```
    pub fn new(value: T, handle_function: &'static mut dyn FnMut(&mut T) -> ()) -> Self {
        ActionObserver {
            value: value.clone(),
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
        (self.func)(&mut self.value);
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use std::i32;

    use super::*;

    #[test]
    fn deref_test() {
        static mut OUTSIDE_THE_SCOPE_VAR: i32 = 0;
        static mut FUNCTION: fn(&mut i32) -> () = |curr: &mut i32| unsafe {
            OUTSIDE_THE_SCOPE_VAR = *curr;
        };
        let mut val = ActionObserver::new(5, unsafe { &mut FUNCTION });
        assert_eq!(*val, 5);
        unsafe {
            assert_eq!(OUTSIDE_THE_SCOPE_VAR, 0);
        }
        *val = 3;
        unsafe { assert_eq!(OUTSIDE_THE_SCOPE_VAR, 5) }
        assert_eq!(*val, 3);
    }
}
