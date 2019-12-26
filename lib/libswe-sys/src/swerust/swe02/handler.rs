/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane Bressani (s.bressani@bluewin.ch)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an MIT License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to use astro_compute_swisseph in your commercial
 * projects, you must adhere to the GPL license or buy a Swiss Ephemeris
 * commercial license.
 */
use crate::raw;
use std::ffi::CString;
use std::os::raw::c_char;

/*
 * 2. The Ephemeris file related functions
 */
/// 2.1
/// Set the path of ephemeris
pub fn set_ephe_path(path: &str) {
    if path.len() > 255 {
        panic!("swe 2.1 -> set_ephe_path -> path to long");
    }
    let c_str = CString::new(path).unwrap();
    let path_final: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        raw::swe_set_ephe_path(path_final);
    }
}
