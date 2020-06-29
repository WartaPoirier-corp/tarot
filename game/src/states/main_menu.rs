use amethyst::prelude::*;
use amethyst::ui::UiCreator;

pub struct MenuPrincipal;

impl SimpleState for MenuPrincipal {
    fn on_start(&mut self, data: StateData<GameData>) {
        let mut world = data.world;
        
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/menu.ron", ())
        });
    }
}
