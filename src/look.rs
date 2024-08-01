use std::ops::Deref;

pub struct LookObserver<'a,T>{
    val:T,
    func: &'a dyn Fn(&'a T)->(),
}
impl<'a,T> LookObserver<'a,T> {
    pub fn new(value:T,on_look_fn: &'a dyn Fn(&'a T)->())->Self{
        LookObserver{
            val:value,
            func:on_look_fn,
        }
    }
    
}

impl<'a,'b,T> Deref for LookObserver<'a,T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        (self.func)(&self.val);
        &self.val
        
    }
    
}
