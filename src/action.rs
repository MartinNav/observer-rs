
 pub struct ActionObserver<T:std::clone::Clone>{
    pub value:T,
    previous:T,
    func: fn(&T,&mut T)->(), 
}
impl<T:std::clone::Clone> ActionObserver<T> {
     fn new(value:T,handle_function:fn(&T,&mut T)->())->Self {
         ActionObserver{
             value:value.clone(),
             previous:value,
             func:handle_function,
         }
     }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
    }
}
