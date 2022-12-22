use crate::prelude::*;


// Dtilt pops up
#[acmd_script( agent = "chrom", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn chrom_attacklw3_smash_script(fighter: &mut L2CAgentBase) {
        
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 75, 30, 0, 95, 2.7, 0.0, 3.5, 13.0, Some(0.0), Some(4.1), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 80, 30, 0, 95, 2.7, 0.0, 0.0, 6.7, Some(0.0), Some(0.0), Some(4.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Dash attack sends back
#[acmd_script( agent = "chrom", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn chrom_attackdash_smash_script(fighter: &mut L2CAgentBase) {
        
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 128, 85, 0, 60, 4.4, 0.0, 6.5, 12.5, Some(0.0), Some(6.5), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 128, 85, 0, 60, 4.4, 0.0, 6.5, 18.5, Some(0.0), Some(6.5), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }

    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

}

// New Ground Down B
#[acmd_script( agent = "chrom", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn chrom_speciallw_smash_script(fighter: &mut L2CAgentBase) {
        
    println!("Counter");

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }


    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        macros::FT_MOTION_RATE(fighter, 2.2);
    }
}
// SFX
#[acmd_script( agent = "chrom", script = "sound_speciallw", category = ACMD_SOUND )]
unsafe fn chrom_sound_speciallw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_chrom_jump02"));
    }
    
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_chrom_attackair_f01"));
    }
}

// VFX
#[acmd_script( agent = "chrom", script = "effect_speciallw", category = ACMD_EFFECT )]
unsafe fn chrom_effect_speciallw_smash_script(fighter: &mut L2CAgentBase) {


    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::FLASH(fighter, 1.0, 1.0, 1.0, 0.75)
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        for _ in 0..2 {
            if is_excute(fighter) {
                macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5)
            }
            wait(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
                macros::FLASH(fighter, 0.67, 0.0, 0.78, 0.31)
            }
            wait(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
                macros::COL_NORMAL(fighter)
            }
            wait(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
            }
            }
    }

}

// New Aerial Down B
#[acmd_script( agent = "chrom", script = "game_specialairlw", category = ACMD_GAME )]
unsafe fn chrom_specialairlw_smash_script(fighter: &mut L2CAgentBase) {
        
    println!("Air Counter");

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {   
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);

        //KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }


    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }


    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        macros::FT_MOTION_RATE(fighter, 3.0);
    }
}

// SFX
#[acmd_script( agent = "chrom", script = "sound_specialairlw", category = ACMD_SOUND )]
unsafe fn chrom_sound_specialairlw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_chrom_jump02"));
    }
    
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_chrom_attackair_f01"));
    }
}

// VFX
#[acmd_script( agent = "chrom", script = "effect_specialairlw", category = ACMD_EFFECT )]
unsafe fn chrom_effect_specialairlw_smash_script(fighter: &mut L2CAgentBase) {


    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::FLASH(fighter, 1.0, 1.0, 1.0, 0.75)
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        for _ in 0..2 {
            if is_excute(fighter) {
                macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5)
            }
            wait(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
                macros::FLASH(fighter, 0.67, 0.0, 0.78, 0.31)
            }
            wait(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
                macros::COL_NORMAL(fighter)
            }
            wait(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
            }
            }
    }

}

pub fn install() {

    smashline::install_acmd_scripts!(
        chrom_attacklw3_smash_script,
        chrom_attackdash_smash_script,
        chrom_speciallw_smash_script,
        chrom_sound_speciallw_smash_script,
        chrom_effect_speciallw_smash_script,
        chrom_specialairlw_smash_script,
        chrom_sound_specialairlw_smash_script,
        chrom_effect_specialairlw_smash_script
    );
}
