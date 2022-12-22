use crate::prelude::*;

#[fighter_frame_callback]
pub fn djc(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
        let fighter_kind = smash::app::utility::get_kind(boma);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        /*
        Double Jump Cancels
        This says that if you're a list of certain fighters, if you're in a certain kinetic type and you enter the attack air status,
        and your in between frames 1-6, and you stop inputing the jump button, change the kinetic type to fall, which halts momentum.
        */
        if [*FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_TRAIL].contains(&fighter_kind) {
            if [*FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL].contains(&KineticModule::get_kinetic_type(boma)) 
            && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR 
            && MotionModule::frame(boma) >= 1.0 
            && MotionModule::frame(boma) <= 6.0 
            && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            };
        };
    };
}


pub unsafe fn get_player_number(boma:  &mut app::BattleObjectModuleAccessor) -> usize {
    let mut player_number = 8;
    if app::utility::get_kind(boma) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(boma, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while app::utility::get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }

    return player_number;
}

// Param Hooks
static mut INT_OFFSET : usize = 0x4ded80;
static mut FLOAT_OFFSET : usize = 0x4dedc0;

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn float_param_accessor_hook_gaogaen(
boma: u64,
param_type: u64,
param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    // Revenge speed boost
    let revenge =  WorkModule::is_flag(module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE);
    if fighter_kind == FIGHTER_KIND_GAOGAEN
    && revenge
    && param_hash == 0 {
		println!("Revenge State");
        if param_type == hash40("dash_speed") {
            println!("dash buff");
            return 2.06; //original 1.76
		}
		else if param_type == hash40("run_speed_max") {
            println!("run buff");
            return 1.78; // original 1.18
		}
		else if param_type == hash40("air_speed_x_stable") {
            println!("air buff");
            return 1.068; // original 0.88
		}
        else if param_type == hash40("air_accel_x_mul") {
            println!("accel buff");
            return 0.08; // original 0.06
		}
		else if param_type == hash40("air_speed_y_stable") {
            println!("fall buff");
            return 1.86; // original 1.76
        }
	}
    
ret
}

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn int_param_accessor_hook_toon(
boma: u64,
param_type: u64,
param_hash: u64) -> i32{
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    // Toon Link Variable Timer
    let flick =  ControlModule::get_flick_y(module_accessor);
    let stick = ControlModule::get_stick_y(module_accessor);
    if fighter_kind == FIGHTER_KIND_TOONLINK
    //&& flick <= 5 
    //&& stick <= -0.8
    && param_hash ==  hash40("bomb") {
        println!("Flick");
        if param_type == hash40("toonlinkbomb_limit_frame") {
            println!("Short Timer");
            return 60; //original 300
        }
    }
    
ret
}
 

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
]; 

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        djc
    );
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        float_param_accessor_hook_gaogaen,
        int_param_accessor_hook_toon
	);
}