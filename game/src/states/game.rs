use amethyst::prelude::*;

pub struct Partie;

impl SimpleState for Partie{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let salle_du_tarot = data.world;
    
        for _ in 0..5 {
            let joueur1 = salle_du_tarot
                .create_entity()
                .build();
            salle_du_tarot.insert(joueur1);
        }
    }
}

