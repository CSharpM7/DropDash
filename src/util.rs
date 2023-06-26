use super::*;

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

pub unsafe fn get_grabbed_opponent_boma(attacker: *mut BattleObjectModuleAccessor) -> &'static mut BattleObjectModuleAccessor {
    let opponent_id = LinkModule::get_node_object_id(attacker, *LINK_NO_CAPTURE) as u32;
    let opponent_object = get_battle_object_from_id(opponent_id);
    return &mut *(*opponent_object).module_accessor
}
pub unsafe fn get_entry_from_boma(boma: *mut BattleObjectModuleAccessor) -> u32 {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32
}
pub unsafe fn get_entry(fighter: &mut L2CAgentBase) -> u32 {
    return get_entry_from_boma(fighter.module_accessor);
}