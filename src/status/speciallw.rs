use crate::imports::imports_agent::*;


#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn sonic_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        return original!(fighter);
    }
    else{
        StatusModule::init_settings(
            fighter.module_accessor,
            SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_UNIQ,
            *GROUND_CORRECT_KIND_KEEP as u32,
            GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_NO_REAC,
            false,
            false,
            false,
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
            0
        );
        0.into()
    }
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        return original!(fighter);
    }
    else{
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);

        let start_rot = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_start_rot_speed_air"));
        WorkModule::set_float(fighter.module_accessor, start_rot,*FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_FLOAT_ROT_SPEED);

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_set_ground_correct_by_situation(true.into());

        fighter.sub_shift_status_main(L2CValue::Ptr(sonic_specialairlw_main_loop as *const () as _))
    }
}
unsafe extern "C" fn sonic_specialairlw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return dropdash_main_loop(fighter,true);
}

#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn sonic_speciallw_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        return 0.into();
    }
    else{
        return 0.into();
    }
}


#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_speciallw_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        return original!(fighter);
    }
    else{
        println!("Drop Dash Start");
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_set_ground_correct_by_situation(true.into());

        fighter.sub_shift_status_main(L2CValue::Ptr(sonic_specialairlw_hold_main_loop as *const () as _))
    }
}
unsafe extern "C" fn sonic_specialairlw_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return dropdash_main_loop(fighter,false);
}
unsafe extern "C" fn dropdash_main_loop(fighter: &mut L2CFighterCommon, startup: bool) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            if startup {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, false);
            }
            else{
                WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, false);
            }
        }
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    if startup{
        if MotionModule::is_end(fighter.module_accessor) {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, false);
            }
            else {
                fighter.change_status_by_situation(FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END.into(), FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD.into(), false.into());
            }
        }
        return 0.into();
    }
    
    //let brake = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_rot_brake"));
    let brake = 1.25;
    let factor = 1.5;
    let rot_step = if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {-brake*factor} else {factor};
    let rot_init = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_start_rot_speed_air"));
    let rot_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_end_rot_speed"));
    let rot_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_rot_speed_max"));
    //let charge_1 = rot_max*0.33;
    //let charge_2 = rot_max*0.66;
    let charge_1 = rot_max*0.425;
    let charge_2 = rot_max*0.75;

    let rot_current = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_FLOAT_ROT_SPEED)+rot_step;
    WorkModule::set_float(fighter.module_accessor, rot_current.clamp(rot_min, rot_max),*FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_FLOAT_ROT_SPEED);
    
    let rot_end = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_end_rot_speed"));
    if (rot_current <= rot_end){
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, false);
    }

    let mut new_charge = 0;
    if (rot_current > charge_2){
        new_charge = 2;
    }
    else if (rot_current > charge_1){
        new_charge = 1;
    }
    else if (rot_current > rot_min){
        new_charge = 0;
    }
    else {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, false);
        return 0.into()
    }
    let current_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);
    //println!("Current Charge: {current_charge} Current Rot: {rot_current} Step: {rot_step}");
    if current_charge != new_charge{
        match new_charge {
            0 => {
                MotionAnimcmdModule::call_script_single(
                    fighter.module_accessor,
                    *FIGHTER_ANIMCMD_EFFECT,
                    Hash40::new_raw(0x204be4482f),
                    -1
                );
            },
            1 => {
                MotionAnimcmdModule::call_script_single(
                    fighter.module_accessor,
                    *FIGHTER_ANIMCMD_EFFECT,
                    Hash40::new_raw(0x1c2f32b79c),
                    -1
                );
            },
            _ => {
                MotionAnimcmdModule::call_script_single(
                    fighter.module_accessor,
                    *FIGHTER_ANIMCMD_EFFECT,
                    Hash40::new_raw(0x1cb1514ffb),
                    -1
                );
            }
        }
    }
    WorkModule::set_int(fighter.module_accessor, new_charge, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);    

    0.into()
}

pub fn install() {
    install_status_scripts!(
        sonic_speciallw_pre,
        sonic_speciallw_main,
        sonic_speciallw_hold_main,
        sonic_speciallw_hold_exec,
    );
}