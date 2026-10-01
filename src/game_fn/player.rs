use crate::game_fn::traits::Character;

pub struct Player{
    pub hp:i32,
    pub attack:i32
}


impl Character for Player{
    fn attack(&self, target: &mut dyn Character) {
        target.take_damage(self.attack);
    }
    fn is_alive(&self) -> bool {
        self.hp>0
    }
    fn take_damage(&mut self, amount: i32) {
        if self.hp>amount{
            self.hp-=amount;
        }else {
            self.hp=0;
        }
    }
}