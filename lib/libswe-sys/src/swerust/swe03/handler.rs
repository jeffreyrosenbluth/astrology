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
// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;
use crate::sweconst::Bodies;
/*
 * 3. The functions swe_calc_ut() and swe_calc()
 *
 * Before calling one of these functions or any other Swiss Ephemeris function,
 * it is strongly recommended to call the function swe_set_ephe_path(). Even if
 * you don’t want to set an ephemeris path and use the Moshier ephemeris, it is
 * nevertheless recommended to call swe_set_ephe_path(NULL), because this
 * function makes important initializations. If you don’t do that, the Swiss
 * Ephemeris may work but the results may be not 100% consistent.
 */
pub fn calc(tjd_ut: f64, ipl: Bodies, iflag: i32) {
    let mut xx: [f64; 6] = [0.0; 6];
    let mut serr = [0; 255];
    unsafe {
        let p_xx = xx.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        raw::swe_calc_ut(tjd_ut, ipl as i32, iflag, p_xx, p_serr);
    }
}
/*
/// 2.4
/// Get version of swiss ephemeris
pub fn version() -> String {
    // Get the version
    let mut version = [0; 255];
    let v = unsafe {
        let p = version.as_mut_ptr();
        raw::swe_version(p);
        CStr::from_ptr(p)
    };
    // Memory clean
    unsafe {
        raw::swe_close();
    }
    CString::from(v).to_str().unwrap().to_string()
}

}*/
