use amethyst::prelude::*;

pub struct Partie;

impl SimpleState for Partie{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let LaSalleDuTarot = data.world;
    
        for i in 0..5 {
            let joueur1 = LaSalleDuTarot
            .create_entity()
            .build();
            LaSalleDuTarot.insert(joueur1);
        }
    }
}

