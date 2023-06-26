use crate::imports::imports_agent::*;

#[fighter_frame( agent = FIGHTER_KIND_SONIC )]
fn sonic_update(fighter: &mut L2CFighterCommon) {
    unsafe {
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        sonic_update
    );
}