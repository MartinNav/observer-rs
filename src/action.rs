use std::ops::{Deref, DerefMut};


 pub struct ActionObserver<'a,T:std::clone::Clone>{
    pub value:T,
    previous:T,
    func:&'a mut dyn FnMut(&T,&mut T)->(), 
}
impl<'a,T:std::clone::Clone> ActionObserver<'a,T> {
     fn new(value:T,handle_function:&'a mut dyn FnMut(&T,&mut T)->())->Self {
         ActionObserver{
             value:value.clone(),
             previous:value,
             func:handle_function,
         }
     }
}
impl<'a,T: std::clone::Clone> Deref for ActionObserver<'a,T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<'a,T: std::clone::Clone+std::cmp::PartialEq> DerefMut for ActionObserver<'a,T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.value!=self.previous {
            (self.func)(&self.previous,&mut self.value);
        }
        self.previous= self.value.clone();
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use std::i32;

    use super::*;

    #[test]
    fn test1() {
         let mut outside_the_scope_var = 0;
         let mut function = |prev:&i32,curr:&mut i32|{
             outside_the_scope_var = *prev;
         };
         let mut val = ActionObserver::new(5, &mut function);
        *val=3;
        assert_eq!(outside_the_scope_var,5);


    }
}
