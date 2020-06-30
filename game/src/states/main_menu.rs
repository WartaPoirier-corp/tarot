use amethyst::prelude::*;
use amethyst::ecs::Entity;
use amethyst::ui::{UiCreator, UiFinder, UiEventType};
use amethyst::input::is_key_down;
use amethyst::input::is_close_requested;
use amethyst::winit::VirtualKeyCode;

#[derive(Default)]
pub struct MainMenu {
    ui: Option<Entity>
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        
        self.ui = Some(world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/menu.ron", ())
        }));
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                if ui_event.event_type == UiEventType::Click {
                    data.world.exec(|finder: UiFinder| {
                        if let Some(bt) = finder.find("play_bt") {
                            if bt == ui_event.target {
                                return Trans::Push(Box::new(super::game::Partie));
                            }
                        }

                        Trans::None
                    })
                } else {
                    Trans::None
                }
            }
            StateEvent::Input(_) => {
                Trans::None
            }
        }
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        if let Some(ui) = self.ui {
            data.world.delete_entity(ui).ok();
        }
    }
}
