use crate::prelude::*;

pub static mut TAUNTED: bool = false;


// Set Taunt thing
#[acmd_script( agent = "master", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn master_speciallw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0,3.0);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {     
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    // Taunt Check 1
    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
            TAUNTED = true;
            println!("TAUNT ON 1/2");
        }
    }

    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }

    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        
        // Taunt Check2
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
            TAUNTED = true;
            println!("TAUNT ON 2/2");
        }
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR);
        ArticleModule::set_flag(fighter.module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }

    // Slow Down and disable taunt
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
  
        if TAUNTED {
            macros::FT_MOTION_RATE(fighter, 3.0);
            TAUNTED = false;
            println!("TAUNT OFF");
        }
    }

    frame(fighter.lua_state_agent, 96.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
    frame(fighter.lua_state_agent, 117.0);
    if is_excute(fighter) {
        
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(fighter.lua_state_agent, 118.0);
    if is_excute(fighter) {
        
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE as i32, smash::app::ArticleOperationTarget(0));
    }
}

// If Taunt, people go boom
#[acmd_script( agent = "master_axe", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn master_axe_speciallw_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::set_int(fighter.module_accessor, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);

    if TAUNTED {
        frame(fighter.lua_state_agent, 61.0);
        if is_excute(fighter) {    
            // Kill
            println!("Kill1");
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 30.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
            //Turn off the Taunt
            TAUNTED = false;
            println!("TAUNT OFF AXE");
        }
    }
    else {
        frame(fighter.lua_state_agent, 61.0);
        if is_excute(fighter) {
            // no kill
            println!("Normal");
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 30.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        }
    }
    
    frame(fighter.lua_state_agent, 67.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
}


pub fn install() {

    smashline::install_acmd_scripts!(
        master_speciallw_smash_script,
        master_axe_speciallw_smash_script
    );
}
