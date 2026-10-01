pub trait Character{ 
    fn attack(&self, target: &mut dyn Character); 
    fn is_alive(&self) -> bool; 
    fn take_damage(&mut self, amount: i32);
}