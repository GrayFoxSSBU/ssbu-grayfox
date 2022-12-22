use crate::prelude::*;



// Down taunt super spike
#[acmd_script( agent = "luigi", scripts = ["game_appeallwr", "game_appeallwl"], category = ACMD_GAME )]
unsafe fn luigi_appeal_smash_script(fighter: &mut L2CAgentBase) {
    
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        
        macros::ATTACK(fighter,0, 0, Hash40::new("top"), 2.0, 270, 130, 180, 0, 3.0, 0.0, -1.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_KICK);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);
    
    }
    sv_animcmd::frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Side B grab edge
#[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
fn luigi_missile_ledge(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let lua_state = fighter.lua_state_agent;
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let is_status = [
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM,
        *FIGHTER_STATUS_KIND_SPECIAL_S
        ].contains(&status_kind);        
        if is_status {
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        else {};
    }
}


pub fn install() {

    smashline::install_acmd_scripts!(
        luigi_appeal_smash_script,
    );
    smashline::install_agent_frames!(
        luigi_missile_ledge
    );
}
