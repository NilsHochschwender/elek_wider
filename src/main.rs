#[macro_use]
extern crate lazy_static; //for reducing generating expensive objects more than once
#[macro_use]
extern crate text_io; // todo: remove code
extern crate regex; // for the regular expressions used to convert the user input

use regex::Regex; // namespace for not typing regex::Regex every time again
use std::f64; //a namespace too
use std::io; //a namespace for the io functions
#[lang = "char"]
impl char{
    fn is_ascii_float(&self) -> bool {
        let possible_chars: Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9','e','-','.'];
        possible_chars.contains(&self)
    }
}

enum PreUnits {
    P,
    T,
    G,
    M,
    k,
    No,
    d,
    z,
    m,
    my,
    n,
}
impl PreUnits{
    fn return_pow(&self) -> i16 {
        match self{
            PreUnits::P => 15,
            PreUnits::T => 12,
            PreUnits::G => 9,
            PreUnits::M => 6,
            PreUnits::k => 3,
            PreUnits::No => 0,
            PreUnits::d => -1,
            PreUnits::z => -2,
            PreUnits::m => -3,
            PreUnits::my => -6,
            PreUnits::n => -12,
        }
    }
    fn convert_pre_unit(&self, num: f64, goal_pre_unit: PreUnits) -> (f64, PreUnits) {
        let mut pow: i16 = self.return_pow();
        pow = pow - goal_pre_units.return_pow();
        if pow != 0 {
            (num / pow as f64, goal_pre_unit)
        } else {
            (num, goal_pre_unit)
        }
    }
    fn new(pre_unit: String) -> PreUnits {
        match pre_unit {
            "P" => PreUnits::P,
            "T" => PreUnits::T,
            "G" => PreUnits::G,
            "M" => PreUnits::M,
            "k" => PreUnits::k,
            ""  => PreUnits::No,
            "d" => PreUnits::d,
            "z" => PreUnits::z,
            "m" => PreUnits::m,
            "µ" | "my" => PreUnits::my,
            "n" => PreUnits::n,
            _   => PreUnits::No,
        }
    }
}

// an enum for the different materials
enum RhoMaterial {
    Copper,
    Iron,
    Aluminium,
    Gold,
    Graphit,
    Silver,
    Platinum,
    Lead,
    Tungsten,
}
//selecting the different possibilities for the materials with the correct rho values
fn select_rho(rho: RhoMaterial) -> f64 {
    match rho { 
        // using match for the selection. different from c, c++,java case switch statement. 
        // this type is common to the ML-language family(functional programming).
        // rust is partially a functional language too
        RhoMaterial::Copper => 1.721e-2f64,
        RhoMaterial::Iron => 1e-1f64,
        RhoMaterial::Aluminium => 2.65e-2f64,
        RhoMaterial::Gold => 2.214e-2f64,
        RhoMaterial::Graphit => 8f64,
        RhoMaterial::Silver => 1.587e-2f64,
        RhoMaterial::Platinum => 1.05e-1f64,
        RhoMaterial::Lead => 2.08e-1f64,
        RhoMaterial::Tungsten => 6.03e-2f64,
    }
}
#[derive(Clone,Debug)]
struct Voltage{
    value: f64,
    pre_unit: PreUnits,
}
impl Voltage{
    fn new(input: String) -> Result<Voltage, &'static str> {
        let split_point = input.find(|c: char| !c.is_ascii_float()).unwrap_or(input.len());
        let mut input_clone = input.clone();
        let num_str: String = input_clone.drain(..split_point).collect();
        let split_point = input_clone.find("V").unwrap_or(input_clone.len());
        let pre_unit: String = input_clone.drain(..split_point).collect();
        if let Ok(num) = num_str.parse::<f64>() {
            Ok(Voltage{
                value: num,
                pre_unit: PreUnits::new(pre_unit),
            })
        } else {
            Err("number not parseable")
        }
    }
}

#[derive(Clone,Debug)]
struct Current{
    value: f64,
    pre_unit: PreUnits,
}
impl Current{
    fn new(input: String) -> Result<Current, &'static str> {
        let split_point = input.find(|c: char| !c.is_ascii_float()).unwrap_or(input.len());
        let mut input_clone = input.clone();
        let num_str: String = input_clone.drain(..split_point).collect();
        let split_point = input_clone.find("A").unwrap_or(input_clone.len());
        let pre_unit: String = input_clone.drain(..split_point).collect();
        if let Ok(num) = num_str.parse::<f64>() {
            Ok(Current{
                value: num,
                pre_unit: PreUnits::new(pre_unit),
            })
        } else {
            Err("number not parseable")
        }
    }

}
// derive is a macro for the default implementation of traits. traits are feature class without
// specifing any data beside of types that have to be present to complete the task. they don't have
// any data type in the trait

//structs are used for data and cannot contain any code
#[derive(Clone, Default, Debug)]
struct Area {
    d: Option<f64>,
    a: Option<f64>,
}
// impl is the part where specific code for a struct or enum is defined. traits are implemented
// with impl Trait for ...
impl Area {
    fn calc_area(&mut self) { // here we implement a methode for the calculation of the circle area
        // here we use a so called left handed expression and the if let statement to match if the
        // cloned, because rust won't give the variable out of the borrow, which causes an error,
        // value contains something we can work with and giving back an Option if it contains
        // something or not. we don't use an semicolon here in rust so that the compiler knows that
        // the value should be assigent to the left-handed expression. if we would create a new
        // variable we wouldn't be able to assign it to our struct because of rusts lifetime. every
        // curvey bracket is a so called scope where every variable and every reference is dropped
        // out of lifetime(like an automatic free in c) and therefore not available to us anymore.
        // for the mutable variable we have to use let mut for a defenition and for the self a
        // mutable borrow of the object &mut self
        // let pid2 = |d| 3.141f64 * d * d;
        self.a = if let Some(d) = self.d.clone() {
            Some(3.141f64 * (d * d) / 4f64)
        } else {
            None
        }
    }
    fn calc_d(&mut self) {
        // here we can see how the type of a variable is defined in rust. unlike in c and others,
        // rust again uses the ML-annotion with variablename: type
        let d: f64 = if let Some(a) = self.a.clone() {
            (a * 4f64) / 3.141f64
        } else {
            -1f64
        };
        self.d = if d > 0f64 { Some(d.sqrt()) } else { None };
    }
}

#[derive(Clone, Default, Debug)]
struct Elect {
    resistor: Option<f64>, //ohm
    rho: Option<f64>, //ohm*mm2/m
    length: Option<f64>, //m
    area: Option<Area>, //d:mm A:mm2
}
impl Elect {
    // here is somewhat of a kind of constructor known from c++ and java.
    // also we are using somekind of errorhandling which is more like what is used in c
    fn new(s: Vec<String>) -> Result<Elect, &'static str> {
        let mut elect: Elect = Default::default();
        for i in s.iter() {
            // lazy_static! is a macro which helps to reduce the defenition of expensive
            // decleration in loops. 
            // with the following regex we try to capture the values the string vec is giving us 
            lazy_static!  {
                static ref RENUM: Regex = Regex::new(r"([0-9\.]+)(ohmxmm2/m|ohm|mm2|mm|m)").unwrap();
                static ref REMAT: Regex = Regex::new(r"([[:alpha:]]+)").unwrap();
            }
            // here happens the actual capture
            let capsnum = RENUM.captures(&i);
            let capsmat = REMAT.captures(&i);
            // some non initialized unmutable variable where we can assign a value once
            let num: String;
            let einheit: String;
            // here we look which of the to possible regex has matched
            if let Some(capsnum) = capsnum {
                // we take the first match from the capture. if it doesn't exist we take an empty
                // string. somewhat of an exception handling
                num = String::from(capsnum.get(1).map_or("", |m| m.as_str()));
                einheit = String::from(capsnum.get(2).map_or("", |m| m.as_str())).to_lowercase();
            } else {
                // if it doesn't match we assign empty strings to variable to prevent a crash
                num = "".to_string();
                einheit = "".to_string();
            }
            // here we check if the second possible input of the material type matched
            let material = if let Some(capsmat) = capsmat {
                String::from(capsmat.get(1).map_or("", |m| m.as_str())).to_lowercase()
            } else {
                "".to_string()
            };
            // for an easier match expression we use str instead of string. if we reassign a
            // variable with let variable_name we can alter immutable variables and their type
            let einheit = einheit.as_str();
            let material = material.as_str();
            // to check if the first regex has matched
            if num != "" && einheit != "" {
                // then we match the str einheiten against there possible ones to assign the value
                // the right field in the struct
                match einheit {
                    "ohm" => {
                        elect.resistor = if let Ok(i) = num.parse::<f64>() {
                            Some(i)
                        } else {
                            None
                        };
                    }
                    "ohmxmm2/m" => {
                        elect.rho = if let Ok(i) = num.parse::<f64>() {
                            Some(i)
                        } else {
                            None
                        }
                    }
                    "m" => {
                        elect.length = if let Ok(i) = num.parse::<f64>() {
                            Some(i)
                        } else {
                            None
                        }
                    }
                    "mm" => {
                        elect.area = if let Ok(i) = num.parse::<f64>() {
                            Some({
                                let mut area = Area {
                                    d: Some(i),
                                    a: None,
                                };
                                area.calc_area();
                                area
                            })
                        } else {
                            None
                        }
                    }
                    "mm2" => {
                        elect.area = if let Ok(i) = num.parse::<f64>() {
                            Some({
                                let mut area = Area {
                                    d: None,
                                    a: Some(i),
                                };
                                area.calc_d();
                                area
                            })
                        } else {
                            None
                        }
                    }
                    _ => return Err("Some wrong input"),
                }
            } else if num == "" && einheit == "" && material != "" {
                let mat = match material {
                    "iron" | "eisen" => RhoMaterial::Iron,
                    "copper" | "kupfer" => RhoMaterial::Copper,
                    "aluminium" | "alu" => RhoMaterial::Aluminium,
                    "gold" => RhoMaterial::Gold,
                    "graphit" => RhoMaterial::Graphit,
                    "silver" | "silber" => RhoMaterial::Silver,
                    "platinum" | "platin" => RhoMaterial::Platinum,
                    "lead" | "blei" => RhoMaterial::Lead,
                    "tungsten" | "nickel" => RhoMaterial::Tungsten,
                    _ => return Err("material not found"),
                };
                elect.rho = Some(select_rho(mat));
            }
        }
        Ok(elect)
    }
    fn calc(&mut self, should_print: bool) -> i8 {
        let counter = 0i8;
        let mut boolarr: [bool; 4] = [false; 4];
        let counter = if self.resistor.is_some() {
            boolarr[0] = true;
            counter + 1
        } else {
            counter
        };
        let counter = if self.rho.is_some() {
            boolarr[1] = true;
            counter + 1
        } else {
            counter
        };
        let counter = if self.length.is_some() {
            boolarr[2] = true;
            counter + 1
        } else {
            counter
        };
        let counter = if self.area.is_some() {
            boolarr[3] = true;
            counter + 1
        } else {
            counter
        };
        let mut num: i8 = -1i8;
        if counter == 3 {
            for i in 0..4 {
                if boolarr[i] {
                    match i {
                        0 => {
                            self.calc_resistor();
                            num = 0i8;
                        }
                        1 => {
                            self.calc_rho();
                            num = 1i8
                        }
                        2 => {
                            self.calc_length();
                            num = 2i8
                        }
                        3 => {
                            self.calc_area();
                            num = 3i8
                        }
                        _ => num = -1i8,
                    }
                }
            }
        } else {
            num = -1i8
        };

        if should_print {
            match num.clone() {
                0 => self.print_res(0i8),
                1 => self.print_res(1i8),
                2 => self.print_res(2i8),
                3 => self.print_res(3i8),
                _ => {}
            }
        }
        num
    }
    fn print_res(&mut self, num: i8) {
        let a: f64 = if let Some(i) = self.area.clone() {
            if let Some(a) = i.a { a } else { 0f64 }
        } else {
            0f64
        };
        let d: f64 = if let Some(i) = self.area.clone() {
            if let Some(d) = i.d { d } else { 0f64 }
        } else {
            0f64
        };
        let rho: f64 = if let Some(i) = self.rho.clone() {
            i
        } else {
            0f64
        };
        let res: f64 = if let Some(i) = self.resistor.clone() {
            i
        } else {
            0f64
        };
        let len: f64 = if let Some(i) = self.length.clone() {
            i
        } else {
            0f64
        };
        match num {
            0 => {
                println!(
                    "The resistenz of the {} m long cable with the area of {} mm² and the diameter {} mm with the specific resistenz of {} Ω·mm²/m is: {} Ω",
                    len,
                    a,
                    d,
                    rho,
                    res
                )
            }
            1 => {
                println!(
                    "The specific resistenz of the {} m long cable with the area of {} mm² and the diameter {} mm for the resistenz of {} Ω is: {} Ω·mm²/m",
                    len,
                    a,
                    d,
                    res,
                    rho
                )
            }
            2 => {
                println!(
                    "The length of the cable with the area of {} mm² and the diameter of {} mm for the resistenz of {} Ω and the specific resistenz of {} Ω·mm²/m is: {} m",
                    a,
                    d,
                    res,
                    rho,
                    len
                )
            }
            3 => {
                println!(
                    "The area and the diameter for the cable with the length of {} m for the resistenz of {} Ω and the specific resistenz of {} Ω·mm²/m is: {} mm² and {} mm",
                    len,
                    res,
                    rho,
                    a,
                    d
                )
            }
            _ => {}
        }
    }
    fn calc_resistor(&mut self) {
        let a: f64 = if let Some(mut area) = self.area.clone() {
            if let Some(a) = area.a {
                a
            } else {
                area.calc_area();
                if let Some(a) = area.a { a } else { f64::NAN }
            }
        } else {
            f64::NAN
        };
        let rho: f64 = if let Some(rho) = self.rho.clone() {
            rho
        } else {
            f64::NAN
        };
        let length: f64 = if let Some(len) = self.length.clone() {
            len
        } else {
            f64::NAN
        };
        self.resistor = if !a.is_nan() && !rho.is_nan() && !length.is_nan() {
            if a < -0.000000001f64 || a > 0.000000001f64 {
                Some(rho * (length / a))
            } else {
                None
            }
        } else {
            None
        };
    }
    fn calc_rho(&mut self) {
        let a: f64 = if let Some(mut area) = self.area.clone() {
            if let Some(a) = area.a {
                a
            } else {
                area.calc_area();
                if let Some(a) = area.a { a } else { f64::NAN }
            }
        } else {
            f64::NAN
        };
        let length: f64 = if let Some(len) = self.length.clone() {
            len
        } else {
            f64::NAN
        };
        let resistor: f64 = if let Some(res) = self.resistor.clone() {
            res
        } else {
            f64::NAN
        };
        self.rho = if !a.is_nan() && !length.is_nan() && !resistor.is_nan() {
            if length < -0.000000001f64 || length > 0.000000001f64 {
                Some((resistor * a) / length)
            } else {
                None
            }
        } else {
            None
        };
    }
    fn calc_length(&mut self) {
        let a: f64 = if let Some(mut area) = self.area.clone() {
            if let Some(a) = area.a {
                a
            } else {
                area.calc_area();
                if let Some(a) = area.a { a } else { f64::NAN }
            }
        } else {
            f64::NAN
        };
        let rho: f64 = if let Some(rho) = self.rho.clone() {
            rho
        } else {
            f64::NAN
        };
        let resistor: f64 = if let Some(res) = self.resistor.clone() {
            res
        } else {
            f64::NAN
        };
        self.length = if !a.is_nan() && !rho.is_nan() && !resistor.is_nan() {
            if rho < -0.000000001f64 || rho > 0.000000001f64 {
                Some((resistor * a) / rho)
            } else {
                None
            }
        } else {
            None
        };
    }
    fn calc_area(&mut self) {
        let rho: f64 = if let Some(rho) = self.rho.clone() {
            rho
        } else {
            f64::NAN
        };
        let length: f64 = if let Some(len) = self.length.clone() {
            len
        } else {
            f64::NAN
        };
        let resistor: f64 = if let Some(res) = self.resistor.clone() {
            res
        } else {
            f64::NAN
        };
        self.area = if !rho.is_nan() && !length.is_nan() && !resistor.is_nan() {
            if resistor < -0.000000001f64 || resistor > 0.000000001f64 {
                Some(Area {
                    d: None,
                    a: Some((rho * length) / resistor),
                })
            } else {
                None
            }
        } else {
            None
        };
    }
}
#[derive(Clone, Debug)]


struct URI {
    u: Option<f64>,
    i: Option<f64>,
    r: Option<f64>,
}
impl URI {
    fn new(s: Vec<String>) -> Result<URI, &'static str> {
        let mut uri = URI {
            u: None,
            i: None,
            r: None,
        };
        for i in s.iter() {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"([0-9\.]+)(ohm|[Aa]|[Vv])").unwrap();
            }
            let caps = RE.captures(&i);
            let num: String;
            let einheit: String;
            if let Some(caps) = caps {
                num = String::from(caps.get(1).map_or("", |m| m.as_str()));
                einheit = String::from(caps.get(2).map_or("", |m| m.as_str())).to_lowercase();
            } else {
                num = "".to_string();
                einheit = "".to_string();
            }
            let einheit = einheit.as_str();
            match einheit {
                "ohm" => {
                    uri.r = if let Ok(i) = num.parse::<f64>() {
                        Some(i)
                    } else {
                        None
                    }
                }
                "v" => {
                    uri.u = if let Ok(i) = num.parse::<f64>() {
                        Some(i)
                    } else {
                        None
                    }
                }
                "a" => {
                    uri.i = if let Ok(i) = num.parse::<f64>() {
                        Some(i)
                    } else {
                        None
                    }
                }
                _ => return Err("input is strange"),
            }
        }
        Ok(uri)
    }
    fn calc(&mut self, should_print: bool) {
        let counter: i8 = 0;
        let mut boolarr: [bool; 3] = [false; 3];
        let counter = if self.u.is_some() {
            boolarr[0] = true;
            counter + 1
        } else {
            counter
        };
        let counter = if self.i.is_some() {
            boolarr[1] = true;
            counter + 1
        } else {
            counter
        };
        let counter = if self.r.is_some() {
            boolarr[2] = true;
            counter + 1
        } else {
            counter
        };
        let mut num: i8 = -1i8;
        if counter == 2 {
            for i in 0..3 {
                if !boolarr[i] {
                    match i {
                        0 => {
                            self.calc_u();
                            num = 0i8
                        }
                        1 => {
                            self.calc_i();
                            num = 1i8
                        }
                        2 => {
                            self.calc_r();
                            num = 2i8
                        }
                        _ => num = -1i8,
                    }
                }
            }
        } else {
            num = -1i8
        }
        if should_print {
            self.print_res(num)
        }
    }
    fn print_res(&self, num: i8) {
        let r: f64 = if let Some(i) = self.r.clone() {
            i
        } else {
            0f64
        };
        let u: f64 = if let Some(i) = self.u.clone() {
            i
        } else {
            0f64
        };
        let i: f64 = if let Some(i) = self.i.clone() {
            i
        } else {
            0f64
        };
        match num {
            0 => {
                println!(
                    "The voltage for the current of {} A and the resistenz of {} Ω is: {} V",
                    i,
                    r,
                    u
                )
            }
            1 => {
                println!(
                    "The current for the voltage of {} V and the resistenz of {} Ω is: {} A",
                    u,
                    r,
                    i
                )
            }
            2 => {
                println!(
                    "The resistenz for the current of {} A and the voltage of {} V is: {} Ω",
                    i,
                    u,
                    r
                )
            }
            _ => {}
        }
    }
    fn calc_u(&mut self) {
        let r: f64 = if let Some(i) = self.r.clone() {
            i
        } else {
            f64::NAN
        };
        let i: f64 = if let Some(i) = self.i.clone() {
            i
        } else {
            f64::NAN
        };
        println!("{} * {} = {}", r, i, r * i);
        self.u = if r > 0f64 && i > 0f64 {
            Some(r * i)
        } else {
            None
        };
    }
    fn calc_i(&mut self) {
        let r: f64 = if let Some(i) = self.r.clone() {
            i
        } else {
            f64::NAN
        };
        let u: f64 = if let Some(i) = self.u.clone() {
            i
        } else {
            f64::NAN
        };
        self.i = if r > 0f64 && u > 0f64 {
            Some(u / r)
        } else {
            None
        };
    }
    fn calc_r(&mut self) {
        let u: f64 = if let Some(i) = self.u.clone() {
            i
        } else {
            f64::NAN
        };
        let i: f64 = if let Some(i) = self.i.clone() {
            i
        } else {
            f64::NAN
        };
        self.r = if u > 0f64 && i > 0f64 {
            Some(u / i)
        } else {
            None
        };
    }
}
//todo implement elect_dual
/*
#[derive(Clone, Debug)]
struct ElectDual {
    first: Elect,
    second: Elect,
}
impl elect_dual {
    fn new(s1:Vec<String>,s2:Vec<String>) -> Result<ElectDual, &'static str>{
        let mut elect_dual = ElectDual{first: Default::default(),second:Default::default()};
        if let Ok(elect) = Elect::new(s1) {elect_dual.first = elect.clone()}else{return Err("strange input")}
        if let Ok(elect) = Elect::new(s2) {elect_dual.second = elect.clone()}else{return Err("strange input")}

    }
}
*/
fn main() {
/*    println!(
        "What do you want to calculate:\n1 for wire resistens,\n2 for uri,\n3 for two different wires."
    );
    let decision: char = read!();
    match decision {
        '1' => {
            println!("please give me three strings");
            let mut inputstrings: Vec<String> = Vec::new();
            for _i in 0..3 {
                let mut buf_string = String::new();
                match io::stdin().read_line(&mut buf_string) {
                    Ok(_n) => {
                        inputstrings.push(String::from(buf_string.trim().clone()).replace("*", "x"))
                    }
                    Err(error) => {
                        println!("error: {}", error);
                    }
                }
            }
            let mut elect = match Elect::new(inputstrings) {
                Ok(n) => n,
                Err(error) => {
                    println!("error: {}!", error);
                    panic!("cannot continue");
                }
            };
            elect.calc(true);
        }
        '2' => {
            println!("please give me two strings");
            let mut inputstrings: Vec<String> = Vec::new();
            for _i in 0..2 {
                let mut buf_string = String::new();
                match io::stdin().read_line(&mut buf_string) {
                    Ok(_) => inputstrings.push(String::from(buf_string.trim().clone())),
                    Err(error) => {
                        println!("error: {}", error);
                    }
                }
            }
            let mut uri = match URI::new(inputstrings){
                Ok(n) => n,
                Err(error) => {
                    println!("error: {}!", error);
                    panic!("cannot continue");
                }
            };
            uri.calc(true);
        }
        _ => {}
    }
*/
    let (given,searched) : (String, String) = {
        let mut given_option:Option<String> = None;
        let mut searched_option:Option<String> = None;
        while given_option.is_none() && searched_option.is_none(){
            println!("give my an input for the given values with semicolon as separator(;) and the units with an colon(:)");
            println!("You can give me the material name in english or german with material=material name.");
            given_option = {
                let mut buf_string = String::new();
                match io::stdin().read_line(&mut buf_string) {
                    Ok(_) => Some(String::from(buf_string.trim().clone())),
                    Err(error) => None,
                }
            };
            println!("And now please enter the variable with the unit we are searching for");
            searched_option = {
                let mut buf_string = String::new();
                match io::stdin().read_line(&mut buf_string){
                    Ok(_) => Some(String::from(buf_string.trim().clone())),
                    Err(error) => None,
                }
            };
        }
        (given_option.unwrap(),searched_option.unwrap())
    };
}
