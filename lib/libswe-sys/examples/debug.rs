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
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;

use strum::{AsStaticRef, IntoEnumIterator};

//use libswe_sys::sweconst::{Bodies, Calandar, HouseSystem};
use libswe_sys::sweconst::{
    Bodies, Calandar, House, Object, ObjectType, OptionalFlag,
};
use libswe_sys::swerust::{
    handler_swe02,
    handler_swe03,
    handler_swe07,
    handler_swe08,
    handler_swe14,
    //    handler_swe14::HousesResult,
};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Data {
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
    lat: f64,
    lng: f64,
}

fn main() {
    println!("Swissephem C -> Rust");
    // let swe02_path_final = "/src/swisseph/sweph";
    // let swe02_path: String =
    //    env::var("CARGO_MANIFEST_DIR").unwrap() + swe02_path_final;
    let swe02_path: &str = "/Users/stephanebressani/Code/Rust/astro_compute_swisseph/lib/libswe-sys/src/swisseph/sweph/";
    println!("Set the path of ephemeris to: {}", swe02_path);
    handler_swe02::set_ephe_path(&swe02_path);
    println!("Version swephem: {}", handler_swe02::version());
    println!("Get path of library: {}", handler_swe02::get_library_path());

    const PATH: &str = "examples/data.json";
    let mut s = String::new();
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push(PATH);
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let data: Data = serde_json::from_str(&s).unwrap();
    println!("Data: {:?}", data);
    let julday: f64 = handler_swe08::julday(
        data.year,
        data.month,
        data.day,
        data.hour,
        Calandar::Gregorian,
    );
    println!("Get julday: {:?}", julday);

    let mut object: Vec<Object> = Vec::new();
    let mut calc: handler_swe03::CalcUtResult;
    for bodies in Bodies::iter() {
        if bodies.clone().object_type() == ObjectType::PlanetOrStar {
            calc = handler_swe03::calc_ut(
                julday,
                bodies.clone(),
                OptionalFlag::Speed as i32,
            );
            object.push(Object::new(
                bodies.clone().as_static(),
                bodies.clone().object_type(),
                calc.longitude,
                calc.latitude,
            ));
        }
    }

    for o in object {
        println!("{:?}", o);
    }

    let pheno_ut: handler_swe07::PhenoUtResult = handler_swe07::pheno_ut(
        julday,
        Bodies::Sun,
        OptionalFlag::Speed as i32,
    );
    println!("PhenoUt: {:?}", pheno_ut);

    // let hsys = HouseSystem::Placidus;
    let name = handler_swe14::house_name('P');
    println!("Hsys: {}", name);

    let result = handler_swe14::houses(julday, data.lng, data.lat, 'P');
    //println!("House object: {:?}", result);
    let mut house: Vec<House> = Vec::new();
    for (i, res) in result.clone().cusps.iter().enumerate() {
        if i > 0 {
            house.push(House::new(i as i32, res.clone()));
            if i + 1 > 12 {
                break;
            }
        }
    }

    for h in house {
        println!("{:?}", h);
    }
    println!("House: {:?}", result.clone());
    /*
     * #[derive(Debug)]
    pub struct HousesResult {
        // cusps: [f64; 37], // Limtation to 32 ->
        // /* array for 13 (or 37 for system G) doubles */
        cusps: Vec<f64>,
        ascmc: [f64; 10],
        result: i32,
    }
    */

    println!("Exit and free memory swephem");
    handler_swe02::close();
}
