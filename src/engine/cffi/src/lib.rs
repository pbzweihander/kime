#![no_std]

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[link(name = "kime_engine", kind = "dylib")]
extern "C" {}

pub use ffi::{
    InputCategory, InputResult, InputResult_CONSUMED, InputResult_HAS_COMMIT,
    InputResult_HAS_PREEDIT, InputResult_LANGUAGE_CHANGED, ModifierState, ModifierState_ALT,
    ModifierState_CONTROL, ModifierState_SHIFT, ModifierState_SUPER, KIME_API_VERSION,
};

pub fn check_api_version() -> bool {
    unsafe { ffi::kime_api_version() == ffi::KIME_API_VERSION }
}

pub struct InputEngine {
    engine: *mut ffi::InputEngine,
}

impl InputEngine {
    pub fn new(config: &Config) -> Self {
        Self {
            engine: unsafe { ffi::kime_engine_new(config.config) },
        }
    }

    pub fn update_layout_state(&self) {
        unsafe { ffi::kime_engine_update_layout_state(self.engine) }
    }

    pub fn set_input_category(&mut self, category: InputCategory) {
        unsafe { ffi::kime_engine_set_input_category(self.engine, category) };
    }

    pub fn press_key(
        &mut self,
        config: &Config,
        hardware_code: u16,
        state: ModifierState,
    ) -> InputResult {
        unsafe { ffi::kime_engine_press_key(self.engine, config.config, hardware_code, state) }
    }

    pub fn preedit_str(&mut self) -> &str {
        unsafe {
            let s = ffi::kime_engine_preedit_str(self.engine);
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(s.ptr, s.len))
        }
    }

    pub fn commit_str(&self) -> &str {
        unsafe {
            let s = ffi::kime_engine_commit_str(self.engine);
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(s.ptr, s.len))
        }
    }

    pub fn clear_commit(&mut self) {
        unsafe {
            ffi::kime_engine_clear_commit(self.engine);
        }
    }

    pub fn clear_preedit(&mut self) {
        unsafe {
            ffi::kime_engine_clear_preedit(self.engine);
        }
    }

    pub fn remove_preedit(&mut self) {
        unsafe {
            ffi::kime_engine_remove_preedit(self.engine);
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            ffi::kime_engine_reset(self.engine);
        }
    }
}

impl Drop for InputEngine {
    fn drop(&mut self) {
        unsafe {
            ffi::kime_engine_delete(self.engine);
        }
    }
}

pub struct Config {
    config: *mut ffi::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config: unsafe { ffi::kime_config_default() },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self {
            config: unsafe { ffi::kime_config_load() },
        }
    }

    pub fn xim_font(&self) -> (&str, f64) {
        unsafe {
            let font = ffi::kime_config_xim_preedit_font(self.config);

            (
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    font.name.ptr,
                    font.name.len,
                )),
                font.size,
            )
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            ffi::kime_config_delete(self.config);
        }
    }
}
