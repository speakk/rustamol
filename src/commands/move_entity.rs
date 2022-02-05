use crate::commands::*;
use crate::components::Path;
use crate::components::TimedPath;
use crate::models::path_finding;
use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;

#[derive(Debug, Copy, Clone)]
pub struct MoveEntity {
    pub from: Coordinates,
    pub to: Coordinates,
}

impl CommandLike for MoveEntity {
    fn execute(&self, world: &mut World) -> TurnCommandResult {
        let from = self.from;
        let to = self.to;
        let result = {
            let coordinates_to_hex = world.get_resource::<CoordinatesToHex>().unwrap();
            let hex_occupants = world.get_resource::<HexOccupants>().unwrap();

            let occupants = hex_occupants.0.get(&from);

            if let Some(occupants) = occupants {
                let entity = occupants.iter().next().unwrap();
                let path = path_finding::get_path(from, to, coordinates_to_hex, hex_occupants);

                Some((path, entity.clone()))
            } else {
                None
            }
        };

        if let Some(result) = result {
            if let Some(path) = result.0 {
                world
                    .get_entity_mut(result.1)
                    .unwrap()
                    .insert_bundle(TimedPath::new(Path(path), None));

                return TurnCommandResult::Success;
            }
        }

        TurnCommandResult::Failure
    }
}
