use crate::prelude::*;

// Wolf Flash SFX
#[acmd_script( agent = "wolf", scripts = ["sound_specialsend","sound_specialairsend"], category = ACMD_SOUND )]
unsafe fn wolf_sound_specialsend_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        // Dtaunt on hit for "AWOO!"
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            macros::PLAY_SE(fighter,Hash40::new("vc_wolf_appeal01"));
            println!("Awoo");
        }
    }
}

// Wolf Side B no Freefall
#[acmd_script( agent = "wolf", scripts = ["game_specialsend","game_specialairsend"], category = ACMD_GAME )]
unsafe fn wolf_specialsend_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 270, 100, 0, 20, 4.0, 0.0, 5.5, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 28, 85, 0, 30, 7.0, 0.0, 5.5, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true)
    }

    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
        frame(fighter.lua_state_agent, 42.0);
        if is_excute(fighter) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            println!("Side B Fall on-hit");
        }
    }
}

// Jump cancel Shine
#[fighter_frame( agent = FIGHTER_KIND_WOLF )]
fn wolf_jumpshine(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let is_status = [
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END
            ].contains(&status_kind) || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && MotionModule::frame(boma) >= 6.0);
        let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let jump_count_max = WorkModule::get_int(boma,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        if is_status 
        && jump_count < jump_count_max 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
}
    
pub fn install() {

    smashline::install_acmd_scripts!(
        wolf_sound_specialsend_smash_script,
        wolf_specialsend_smash_script
    );
    smashline::install_agent_frames!(
        wolf_jumpshine
    );
}
