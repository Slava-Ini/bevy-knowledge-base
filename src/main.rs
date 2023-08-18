use basics::basics_app;
use bundles_and_resources::bundles_and_resources_app;
use player_inputs::player_inputs_app;

pub(crate) mod basics;
pub(crate) mod bundles_and_resources;
pub(crate) mod player_inputs;

fn main() {
    // basics_app();
    // bundles_and_resources_app();
    player_inputs_app();
}
