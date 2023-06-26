use crate::imports::imports_acmd::*;

#[acmd_script( agent = "sonic", scripts = [
    "effect_jumpfront","effect_jumpfrontmini","effect_jumpaerialfront"
    ], category = ACMD_EFFECT)]
unsafe fn effect_jumpf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }

    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sonic_spinblur_max"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinblur_max"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_RATE(agent,1.2);
        
        BURN_COLOR(agent,0.3, 0.8, 10.5, 0.3);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sonic_spinblur_max"),false,false);
        COL_NORMAL(agent);
    }
}

#[acmd_script( agent = "sonic", scripts = [
    "effect_jumpback","effect_jumpbackmini","effect_jumpaerialback"
    ], category = ACMD_EFFECT)]
unsafe fn effect_jumpb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }

    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sonic_spinblur_max"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinblur_max"), Hash40::new("top"), 0, 5, 0, 0, 180, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_RATE(agent,1.2);
        
        BURN_COLOR(agent,0.3, 0.8, 10.5, 0.3);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sonic_spinblur_max"),false,false);
        COL_NORMAL(agent);
    }
}

pub fn install() {
    install_acmd_scripts!(
        effect_jumpf,
        effect_jumpb
    );
}