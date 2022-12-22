use crate::prelude::*;
use crate::custom::get_player_number;

//fix jab 1 and 2 so they actually link properly, 
#[acmd_script( agent = "reflet", script = "game_attack11", category = ACMD_GAME )]
unsafe fn reflet_attackh11_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.8, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.8, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 15, 0, 20, 1.8, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 1.8, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 4, 0, Hash40::new("sword"), 2.0, 361, 25, 0, 25, 3.0, 2.0, 0.0, -2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 5, 0, Hash40::new("sword"), 2.0, 361, 25, 0, 25, 3.5, 3.0, 4.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        AttackModule::set_add_reaction_frame(fighter.module_accessor,0, 4.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor,1, 4.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor,2, 4.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor,3, 4.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor,4, 4.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor,5, 4.0, false);
    }    

    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
        
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

// dtilt pop up
#[acmd_script( agent = "reflet", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn reflet_attackhlw3_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor,3.5, 4.0);
    }
    
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor,0.0, 5.0);
    }
    
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 88, 90, 0, 40, 3.5, 0.0, 3.0, 12.0, Some(0.0), Some(5.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }    

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor,5.0, 5.0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// give utilt earlier hitboxes so it scoops better, 
#[acmd_script( agent = "reflet", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn reflet_attackhi3_smash_script(fighter: &mut L2CAgentBase) {
    
    
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.0, 96, 80, 0, 50, 3.5, 0.0, 5.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 6.0, 96, 80, 0, 60, 3.5, 0.0, 1.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.0, 96, 80, 0, 70, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }    

    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// upsmash launcher *may be too complex for me
#[acmd_script( agent = "reflet", scripts = ["game_attackhi4","game_attackhi42"], category = ACMD_GAME )]
unsafe fn reflet_attackhi4_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }

    // Levin Sword Check
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        // Levin Sword
        WorkModule::is_flag(fighter.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);

        // If Levin Sword is available
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == true {

            AttackModule::clear_all(fighter.module_accessor);
            
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 15.0, 92, 86, 0, 55, 4.3, 0.0, 7.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);

            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 92, 86, 0, 55, 4.5, 0.0, 17.0, 4.0, Some(0.0), Some(12.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);

            // expand sword hitbox and clear body hitbox
            wait(fighter.lua_state_agent, 1.0);
            if is_excute(fighter) {
                AttackModule::set_size(fighter.module_accessor,0, 5.8);
                AttackModule::clear(fighter.module_accessor,1,false);
                AttackModule::clear(fighter.module_accessor,2,false);
            }

            // weak spinning hitbox
            wait(fighter.lua_state_agent, 4.0);
            if is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 10.0, 70, 80, 0, 60, 2.5, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            }

            // Clear hitbox
            frame(fighter.lua_state_agent, 33.0);
            if is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
            }
            
            frame(fighter.lua_state_agent, 39.0);
            // Check if Levin sword ran out of points
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0  {
                // If Levin sword is equipped
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == true {
                    if is_excute(fighter) {
                        // Turn Sword back to Bronze?
                        VisibilityModule::set_int64(fighter.module_accessor,hash40("sword") as i64, hash40("sword_normal")as i64);
                        // Turn off Levin Sword
                        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);

                        
                        }
                    }
                }
        }  
    }
    // Else for Bronze sword damage
    else {
        if is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 9.0, 55, 84, 0, 45, 4.2, 0.0, 5.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        // Expand hitbox
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor,0, 5.8);
            AttackModule::clear(fighter.module_accessor,1,false);
        }
        // Clear hitbox
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
        }
    }

    

}


//make nos not put into freefall
#[acmd_script( agent = "reflet", script = "game_specialairlwend", category = ACMD_GAME )]
unsafe fn reflet_specialairlwend_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }    

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.7);
    }

    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    }
}


// make elwind a 2 input action where elwind 1 does not put into freefall
#[acmd_script( agent = "reflet", scripts = ["game_specialhi","game_specialairhi"], category = ACMD_GAME, low_priority)]
unsafe fn reflet_specialhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
       ArticleModule::generate_article(module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
       WorkModule::on_flag(module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }

    frame(fighter.lua_state_agent, 12.0);
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if is_excute(fighter) {
            WorkModule::on_flag(module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
    }

    frame(lua_state, 24.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
    }
}

// Start with levin sword
#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_REFLET {
            return;
        }
        WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
    }
}

// One Up B per airtime

pub static mut UPB_CHECK : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_REFLET)]
fn upb_reflet(fighter: &mut L2CFighterCommon) {
    unsafe{
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let is_status = status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if [*FIGHTER_STATUS_KIND_DAMAGE, 
            *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
            *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
            ].contains(&status_kind) 
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        && UPB_CHECK[get_player_number(boma)] {
            UPB_CHECK[get_player_number(boma)] = false;
        }
        if is_status {
            UPB_CHECK[get_player_number(boma)] = true;
        }
    }
}


#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
term: i32
) -> bool {
    let fighter_kind = smash::app::utility::get_kind(boma);
    let ret = original!()(boma, term);
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
            if fighter_kind == FIGHTER_KIND_REFLET || fighter_kind == FIGHTER_KIND_DUCKHUNT {
                if UPB_CHECK[get_player_number(boma)] {
                    return false;
                }
                else {
                    return ret;
                }
            }
        }
return ret;
}

pub fn install() {
    smashline::install_acmd_scripts!(
        reflet_attackh11_smash_script,
        reflet_attackhlw3_smash_script,
        reflet_attackhi3_smash_script,
        reflet_attackhi4_smash_script,
        reflet_specialairlwend_smash_script,
        reflet_specialhi2
    );
    install_agent_init_callbacks!(
        agent_init
    );
    smashline::install_agent_frames!(
        upb_reflet
    );

    skyline::install_hook!(
        is_enable_transition_term_replace
    );
}
