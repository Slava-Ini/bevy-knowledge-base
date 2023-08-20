use basics::basics_app;
use bundles_and_resources::bundles_and_resources_app;
use project::project_app;

pub(crate) mod basics;
pub(crate) mod bundles_and_resources;
pub(crate) mod project;

fn main() {
    // basics_app();
    // bundles_and_resources_app();
    project_app();
}
