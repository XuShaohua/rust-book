#![allow(dead_code)]

use std::mem::{align_of, size_of};

#[repr(u8)]
#[derive(Debug)]
pub enum Enum {
    Uint = 3,
    Tuple(u32) = 2,
    Struct { a: u8, b: u16 } = 1,
}

impl Enum {
    pub fn tag(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    pub fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    pub fn singular(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "second",
            TimeUnit::Minutes => "minute",
            TimeUnit::Hours => "hour",
            TimeUnit::Days => "day",
            TimeUnit::Months => "month",
            TimeUnit::Years => "year",
        }
    }

    pub fn tag(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    IntTheFuture(TimeUnit, u32),
}

pub enum LargeEnums {
    Enum0,
    Enum1,
    Enum2,
    Enum3,
    Enum4,
    Enum5,
    Enum6,
    Enum7,
    Enum8,
    Enum9,
    Enum10,
    Enum11,
    Enum12,
    Enum13,
    Enum14,
    Enum15,
    Enum16,
    Enum17,
    Enum18,
    Enum19,
    Enum20,
    Enum21,
    Enum22,
    Enum23,
    Enum24,
    Enum25,
    Enum26,
    Enum27,
    Enum28,
    Enum29,
    Enum30,
    Enum31,
    Enum32,
    Enum33,
    Enum34,
    Enum35,
    Enum36,
    Enum37,
    Enum38,
    Enum39,
    Enum40,
    Enum41,
    Enum42,
    Enum43,
    Enum44,
    Enum45,
    Enum46,
    Enum47,
    Enum48,
    Enum49,
    Enum50,
    Enum51,
    Enum52,
    Enum53,
    Enum54,
    Enum55,
    Enum56,
    Enum57,
    Enum58,
    Enum59,
    Enum60,
    Enum61,
    Enum62,
    Enum63,
    Enum64,
    Enum65,
    Enum66,
    Enum67,
    Enum68,
    Enum69,
    Enum70,
    Enum71,
    Enum72,
    Enum73,
    Enum74,
    Enum75,
    Enum76,
    Enum77,
    Enum78,
    Enum79,
    Enum80,
    Enum81,
    Enum82,
    Enum83,
    Enum84,
    Enum85,
    Enum86,
    Enum87,
    Enum88,
    Enum89,
    Enum90,
    Enum91,
    Enum92,
    Enum93,
    Enum94,
    Enum95,
    Enum96,
    Enum97,
    Enum98,
    Enum99,
    Enum100,
    Enum101,
    Enum102,
    Enum103,
    Enum104,
    Enum105,
    Enum106,
    Enum107,
    Enum108,
    Enum109,
    Enum110,
    Enum111,
    Enum112,
    Enum113,
    Enum114,
    Enum115,
    Enum116,
    Enum117,
    Enum118,
    Enum119,
    Enum120,
    Enum121,
    Enum122,
    Enum123,
    Enum124,
    Enum125,
    Enum126,
    Enum127,
    Enum128,
    Enum129,
    Enum130,
    Enum131,
    Enum132,
    Enum133,
    Enum134,
    Enum135,
    Enum136,
    Enum137,
    Enum138,
    Enum139,
    Enum140,
    Enum141,
    Enum142,
    Enum143,
    Enum144,
    Enum145,
    Enum146,
    Enum147,
    Enum148,
    Enum149,
    Enum150,
    Enum151,
    Enum152,
    Enum153,
    Enum154,
    Enum155,
    Enum156,
    Enum157,
    Enum158,
    Enum159,
    Enum160,
    Enum161,
    Enum162,
    Enum163,
    Enum164,
    Enum165,
    Enum166,
    Enum167,
    Enum168,
    Enum169,
    Enum170,
    Enum171,
    Enum172,
    Enum173,
    Enum174,
    Enum175,
    Enum176,
    Enum177,
    Enum178,
    Enum179,
    Enum180,
    Enum181,
    Enum182,
    Enum183,
    Enum184,
    Enum185,
    Enum186,
    Enum187,
    Enum188,
    Enum189,
    Enum190,
    Enum191,
    Enum192,
    Enum193,
    Enum194,
    Enum195,
    Enum196,
    Enum197,
    Enum198,
    Enum199,
    Enum200,
    Enum201,
    Enum202,
    Enum203,
    Enum204,
    Enum205,
    Enum206,
    Enum207,
    Enum208,
    Enum209,
    Enum210,
    Enum211,
    Enum212,
    Enum213,
    Enum214,
    Enum215,
    Enum216,
    Enum217,
    Enum218,
    Enum219,
    Enum220,
    Enum221,
    Enum222,
    Enum223,
    Enum224,
    Enum225,
    Enum226,
    Enum227,
    Enum228,
    Enum229,
    Enum230,
    Enum231,
    Enum232,
    Enum233,
    Enum234,
    Enum235,
    Enum236,
    Enum237,
    Enum238,
    Enum239,
    Enum240,
    Enum241,
    Enum242,
    Enum243,
    Enum244,
    Enum245,
    Enum246,
    Enum247,
    Enum248,
    Enum249,
    Enum250,
    Enum251,
    Enum252,
    Enum253,
    Enum254,
    Enum255,
    Enum256,
    Enum257,
    Enum258,
    Enum259,
    Enum260,
    Enum261,
    Enum262,
    Enum263,
    Enum264,
    Enum265,
    Enum266,
    Enum267,
    Enum268,
    Enum269,
    Enum270,
    Enum271,
    Enum272,
    Enum273,
    Enum274,
    Enum275,
    Enum276,
    Enum277,
    Enum278,
    Enum279,
    Enum280,
    Enum281,
    Enum282,
    Enum283,
    Enum284,
    Enum285,
    Enum286,
    Enum287,
    Enum288,
    Enum289,
    Enum290,
    Enum291,
    Enum292,
    Enum293,
    Enum294,
    Enum295,
    Enum296,
    Enum297,
    Enum298,
    Enum299,
    Enum300,
    Enum301,
    Enum302,
    Enum303,
    Enum304,
    Enum305,
    Enum306,
    Enum307,
    Enum308,
    Enum309,
    Enum310,
    Enum311,
    Enum312,
    Enum313,
    Enum314,
    Enum315,
    Enum316,
    Enum317,
    Enum318,
    Enum319,
    Enum320,
    Enum321,
    Enum322,
    Enum323,
    Enum324,
    Enum325,
    Enum326,
    Enum327,
    Enum328,
    Enum329,
    Enum330,
    Enum331,
    Enum332,
    Enum333,
    Enum334,
    Enum335,
    Enum336,
    Enum337,
    Enum338,
    Enum339,
    Enum340,
    Enum341,
    Enum342,
    Enum343,
    Enum344,
    Enum345,
    Enum346,
    Enum347,
    Enum348,
    Enum349,
    Enum350,
    Enum351,
    Enum352,
    Enum353,
    Enum354,
    Enum355,
    Enum356,
    Enum357,
    Enum358,
    Enum359,
    Enum360,
    Enum361,
    Enum362,
    Enum363,
    Enum364,
    Enum365,
    Enum366,
    Enum367,
    Enum368,
    Enum369,
    Enum370,
    Enum371,
    Enum372,
    Enum373,
    Enum374,
    Enum375,
    Enum376,
    Enum377,
    Enum378,
    Enum379,
    Enum380,
    Enum381,
    Enum382,
    Enum383,
    Enum384,
    Enum385,
    Enum386,
    Enum387,
    Enum388,
    Enum389,
    Enum390,
    Enum391,
    Enum392,
    Enum393,
    Enum394,
    Enum395,
    Enum396,
    Enum397,
    Enum398,
    Enum399,
    Enum400,
    Enum401,
    Enum402,
    Enum403,
    Enum404,
    Enum405,
    Enum406,
    Enum407,
    Enum408,
    Enum409,
    Enum410,
    Enum411,
    Enum412,
    Enum413,
    Enum414,
    Enum415,
    Enum416,
    Enum417,
    Enum418,
    Enum419,
    Enum420,
    Enum421,
    Enum422,
    Enum423,
    Enum424,
    Enum425,
    Enum426,
    Enum427,
    Enum428,
    Enum429,
    Enum430,
    Enum431,
    Enum432,
    Enum433,
    Enum434,
    Enum435,
    Enum436,
    Enum437,
    Enum438,
    Enum439,
    Enum440,
    Enum441,
    Enum442,
    Enum443,
    Enum444,
    Enum445,
    Enum446,
    Enum447,
    Enum448,
    Enum449,
    Enum450,
    Enum451,
    Enum452,
    Enum453,
    Enum454,
    Enum455,
    Enum456,
    Enum457,
    Enum458,
    Enum459,
    Enum460,
    Enum461,
    Enum462,
    Enum463,
    Enum464,
    Enum465,
    Enum466,
    Enum467,
    Enum468,
    Enum469,
    Enum470,
    Enum471,
    Enum472,
    Enum473,
    Enum474,
    Enum475,
    Enum476,
    Enum477,
    Enum478,
    Enum479,
    Enum480,
    Enum481,
    Enum482,
    Enum483,
    Enum484,
    Enum485,
    Enum486,
    Enum487,
    Enum488,
    Enum489,
    Enum490,
    Enum491,
    Enum492,
    Enum493,
    Enum494,
    Enum495,
    Enum496,
    Enum497,
    Enum498,
    Enum499,
    Enum500,
    Enum501,
    Enum502,
    Enum503,
    Enum504,
    Enum505,
    Enum506,
    Enum507,
    Enum508,
    Enum509,
    Enum510,
    Enum511,
    Enum512,
}

enum BaseClass {
    Unit,
    Struct,
    Tuple,
}

fn main() {
    assert_eq!(1, Enum::Struct { a: 7, b: 11 }.tag());
    assert_eq!(2, Enum::Tuple(5).tag());
    assert_eq!(3, Enum::Uint.tag());
    println!("sizeof enum: {}", size_of::<Enum>());

    println!("align of TimeUnit: {}", align_of::<TimeUnit>());
    println!("size of TimeUnit: {}", size_of::<TimeUnit>());
    println!("align of u32: {}", align_of::<u32>());
    println!("size of RoughTime: {}", size_of::<RoughTime>());
    println!("tag of TimeUnit::Seconds {}", TimeUnit::Seconds.tag());

    println!("size of string: {}", size_of::<String>());
    println!("size of Vec<String>: {}", size_of::<Vec<String>>());

    println!("align of large enum: {}", align_of::<LargeEnums>());
    println!("size of large enum: {}", size_of::<LargeEnums>());

    println!("size of box: {}", size_of::<Box<u32>>());

    println!("size of base class: {}", size_of::<BaseClass>());
    println!("size of boolean: {}", size_of::<bool>());
}
