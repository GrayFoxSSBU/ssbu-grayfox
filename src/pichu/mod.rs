use crate::prelude::*;


// Stronger ftilt
#[acmd_script( agent = "pichu", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn pichu_attacks3_smash_script(fighter: &mut L2CAgentBase) {
    

    
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {

        DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
        
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 8.0, 366, 150, 0, 5, 3.5, 0.0, 0.0, 0.0, Some(0.0),Some(-4.5),Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
 
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 366, 150, 0, 5, 3.0, 0.0, 0.0, 0.0, Some(0.0),Some(-3.7),Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor,0,false);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

}

pub fn install() {

    smashline::install_acmd_scripts!(
        pichu_attacks3_smash_script

    );
}
