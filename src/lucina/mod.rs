use crate::prelude::*;



// New Ground Down B
#[acmd_script( agent = "lucina", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn lucina_speciallw_smash_script(fighter: &mut L2CAgentBase) {
        
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::SET_SPEED_EX(fighter,3.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

// SFX
#[acmd_script( agent = "lucina", script = "sound_speciallw", category = ACMD_SOUND )]
unsafe fn lucina_sound_speciallw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_jump02"));
    }
    
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_attackair_f01"));
    }
}

// VFX
#[acmd_script( agent = "lucina", script = "effect_speciallw", category = ACMD_EFFECT )]
unsafe fn lucina_effect_speciallw_smash_script(fighter: &mut L2CAgentBase) {


    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::FLASH(fighter, 1.0, 1.0, 1.0, 0.75)
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        for _ in 0..2 {
            if is_excute(fighter) {
                macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
                macros::EFFECT(fighter,Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
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
#[acmd_script( agent = "lucina", script = "game_specialairlw", category = ACMD_GAME )]
unsafe fn lucina_specialairlw_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {   
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {   
        macros::SET_SPEED_EX(fighter,3.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {   
        macros::SET_SPEED_EX(fighter,2.4, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {   
        macros::SET_SPEED_EX(fighter,1.8, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {   
        macros::SET_SPEED_EX(fighter,1.2, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {   
        macros::SET_SPEED_EX(fighter,0.6, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        KineticModule::clear_speed_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {   
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        macros::SET_SPEED_EX(fighter,0.0, -0.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

// SFX
#[acmd_script( agent = "lucina", script = "sound_specialairlw", category = ACMD_SOUND )]
unsafe fn lucina_sound_specialairlw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_jump02"));
    }
    
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_attackair_f01"));
    }
}

// VFX
#[acmd_script( agent = "lucina", script = "effect_specialairlw", category = ACMD_EFFECT )]
unsafe fn lucina_effect_specialairlw_smash_script(fighter: &mut L2CAgentBase) {


    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::FLASH(fighter, 1.0, 1.0, 1.0, 0.75)
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        for _ in 0..2 {
            if is_excute(fighter) {
                macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
                macros::EFFECT(fighter,Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
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


#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_speedbreaker(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let is_status = status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX && MotionModule::frame(boma) == 3.0;
        if is_status
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND 
        {
            println!("Max Charge f5");
            macros::SET_SPEED_EX(fighter,5.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }        
}
pub fn install() {

    smashline::install_acmd_scripts!(
        lucina_speciallw_smash_script,
        lucina_sound_speciallw_smash_script,
        lucina_effect_speciallw_smash_script,
        lucina_specialairlw_smash_script,
        lucina_sound_specialairlw_smash_script,
        lucina_effect_specialairlw_smash_script
    );
    smashline::install_agent_frames!(
        lucina_speedbreaker
    );
}
