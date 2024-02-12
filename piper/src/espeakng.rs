#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/* automatically generated by rust-bindgen 0.59.2 */
/* hand crafted to work around some errors  */

pub type espeak_ERROR = ::std::os::raw::c_int;
pub const espeak_ERROR_EE_OK: espeak_ERROR = 0;

pub type espeak_AUDIO_OUTPUT = ::std::os::raw::c_int;
pub const espeak_AUDIO_OUTPUT_AUDIO_OUTPUT_RETRIEVAL: espeak_AUDIO_OUTPUT = 1;

pub const espeakINITIALIZE_DONT_EXIT: u32 = 32768;
pub const espeakINITIALIZE_PHONEME_IPA: u32 = 2;
pub const espeakCHARS_UTF8: u32 = 1;

extern "C" {
    pub fn espeak_SetVoiceByName(name: *const ::std::os::raw::c_char) -> espeak_ERROR;

    pub fn espeak_Initialize(
        output: espeak_AUDIO_OUTPUT,
        buflength: ::std::os::raw::c_int,
        path: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[allow(dead_code)]
    pub fn espeak_TextToPhonemes(
        textptr: *mut *const ::std::os::raw::c_char,
        textmode: ::std::os::raw::c_int,
        phonememode: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;

    pub fn espeak_TextToPhonemesWithTerminator(
        textptr: *mut *const ::std::os::raw::c_char,
        textmode: ::std::os::raw::c_int,
        phonememode: ::std::os::raw::c_int,
        terminator: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}