//! Module to deal with Scratch opcode

//! TODO: Add opcode for other extesion
//! Opcode is type of thing this scratch block refers to.
//! Ex.
//! - [`OpCode::motion_movesteps`] is use in [crate::block::Block] that mean this block is a "move steps" block.
//! - [`OpCode::data_listcontents`] is use in [crate::monitor::Monitor] that mean this monitor is a monitor that display content of a list.

use rs_sb3::value::OpCode;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryOpCode {
    control_forever,
    control_repeat,
    control_if,
    control_if_else,
    control_stop,
    control_wait,
    control_wait_until,
    control_repeat_until,
    control_while,
    control_for_each,
    control_start_as_clone,
    control_create_clone_of_menu,
    control_create_clone_of,
    control_delete_this_clone,
    control_get_counter,
    control_incr_counter,
    control_clear_counter,
    control_all_at_once,
    data_variable,
    data_setvariableto,
    data_changevariableby,
    data_showvariable,
    data_hidevariable,
    data_listcontents,
    data_listindexall,
    data_listindexrandom,
    data_addtolist,
    data_deleteoflist,
    data_deletealloflist,
    data_insertatlist,
    data_replaceitemoflist,
    data_itemoflist,
    data_itemnumoflist,
    data_lengthoflist,
    data_listcontainsitem,
    data_showlist,
    data_hidelist,
    event_whentouchingobject,
    event_touchingobjectmenu,
    event_whenflagclicked,
    event_whenthisspriteclicked,
    event_whenstageclicked,
    event_whenbroadcastreceived,
    event_whenbackdropswitchesto,
    event_whengreaterthan,
    event_broadcast_menu,
    event_broadcast,
    event_broadcastandwait,
    event_whenkeypressed,
    extension_pen_down,
    extension_music_drum,
    extension_wedo_motor,
    extension_wedo_hat,
    extension_wedo_boolean,
    extension_wedo_tilt_reporter,
    extension_wedo_tilt_menu,
    extension_music_reporter,
    extension_microbit_display,
    extension_music_play_note,
    looks_sayforsecs,
    looks_say,
    looks_thinkforsecs,
    looks_think,
    looks_show,
    looks_hide,
    looks_hideallsprites,
    looks_changeeffectby,
    looks_seteffectto,
    looks_cleargraphiceffects,
    looks_changesizeby,
    looks_setsizeto,
    looks_size,
    looks_changestretchby,
    looks_setstretchto,
    looks_costume,
    looks_switchcostumeto,
    looks_nextcostume,
    looks_switchbackdropto,
    looks_backdrops,
    looks_gotofrontback,
    looks_goforwardbackwardlayers,
    looks_backdropnumbername,
    looks_costumenumbername,
    looks_switchbackdroptoandwait,
    looks_nextbackdrop,
    motion_movesteps,
    motion_turnright,
    motion_turnleft,
    motion_pointindirection,
    motion_pointtowards_menu,
    motion_pointtowards,
    motion_goto_menu,
    motion_gotoxy,
    motion_goto,
    motion_glidesecstoxy,
    motion_glideto_menu,
    motion_glideto,
    motion_changexby,
    motion_setx,
    motion_changeyby,
    motion_sety,
    motion_ifonedgebounce,
    motion_setrotationstyle,
    motion_xposition,
    motion_yposition,
    motion_direction,
    motion_scroll_right,
    motion_scroll_up,
    motion_align_scene,
    motion_xscroll,
    motion_yscroll,
    operator_add,
    operator_subtract,
    operator_multiply,
    operator_divide,
    operator_random,
    operator_lt,
    operator_equals,
    operator_gt,
    operator_and,
    operator_or,
    operator_not,
    operator_join,
    operator_letter_of,
    operator_length,
    operator_contains,
    operator_mod,
    operator_round,
    operator_mathop,
    procedures_definition,
    procedures_call,
    procedures_prototype,
    procedures_declaration,
    argument_reporter_boolean,
    argument_reporter_string_number,
    argument_editor_boolean,
    argument_editor_string_number,
    sensing_touchingobject,
    sensing_touchingobjectmenu,
    sensing_touchingcolor,
    sensing_coloristouchingcolor,
    sensing_distanceto,
    sensing_distancetomenu,
    sensing_askandwait,
    sensing_answer,
    sensing_keypressed,
    sensing_keyoptions,
    sensing_mousedown,
    sensing_mousex,
    sensing_mousey,
    sensing_setdragmode,
    sensing_loudness,
    sensing_loud,
    sensing_timer,
    sensing_resettimer,
    sensing_of_object_menu,
    sensing_of,
    sensing_current,
    sensing_dayssince2000,
    sensing_username,
    sensing_userid,
    sound_sounds_menu,
    sound_play,
    sound_playuntildone,
    sound_stopallsounds,
    sound_seteffectto,
    sound_changeeffectby,
    sound_cleareffects,
    sound_changevolumeby,
    sound_setvolumeto,
    sound_volume,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenExtensionOpCode {
    pen_clear,
    pen_stamp,
    pen_penDown,
    pen_penUp,
    pen_setPenColorToColor,
    pen_changePenColorParamBy,
    pen_setPenColorParamTo,
    pen_changePenSizeBy,
    pen_setPenSizeTo,
    pen_setPenShadeToNumber,
    pen_changePenShadeBy,
    pen_setPenHueToNumber,
    pen_changePenHueBy,
}

macro_rules! impl_things {
    ($($ty:ty)*) => {
        $(
            impl Into<OpCode> for $ty {
                fn into(self) -> OpCode {
                    self.to_string()
                }
            }

            impl std::fmt::Display for $ty {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        )*
    };
}

impl_things! { PrimaryOpCode PenExtensionOpCode }
