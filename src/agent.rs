use crate::imports::imports_agent::*;

/* 
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    true.into()
}
*/
pub unsafe fn agent_rebirth(fighter: &mut L2CFighterCommon){
    
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    agent_rebirth(fighter);
    return original!(fighter);
}

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_SONIC {
        return;
    }
    agent_rebirth(fighter);
    //fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}


#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
    install_status_scripts!(
        rebirth_pre,
    );
}