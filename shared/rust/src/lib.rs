//! Shared types for ji cloud project.

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]
#![warn(clippy::pedantic)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::future_not_send)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::string_lit_as_bytes)]
#![warn(clippy::use_self)]
#![warn(clippy::useless_let_if_seq)]
#![allow(
    clippy::option_option,
    clippy::module_name_repetitions,
    clippy::default_trait_access
)]
#![allow(warnings)]

// use std::fmt::Display;

// use miniserde::{Deserialize, Serialize};

use std::{str::FromStr, fmt::Display};

pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod media;

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// pub struct Uuid {
//     pub a: crate::Uuid,
// }

// impl Display for Uuid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// impl Serialize for Uuid {
//     fn begin(&self) -> miniserde::ser::Fragment {
//         todo!()
//     }
// }

// impl Deserialize for Uuid {
//     fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
//         todo!()
//     }
// }

// impl From<crate::Uuid> for Uuid {
//     fn from(a: crate::Uuid) -> Self {
//         Self {
//             a
//         }
//     }
// }

///fdsa
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DateTime<T: chrono::TimeZone + Copy>(pub chrono::DateTime<T>);

impl Default for DateTime<Utc> {
    fn default() -> Self {
        let n = chrono::DateTime::default();
        let g = Utc(n.timezone());
        let pp = n.with_timezone(&g);
        DateTime(pp)
    }
}

impl std::fmt::Display for Utc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UTC")
    }
}

// impl<T: Copy + chrono::TimeZone> Copy for DateTime<T> {

// }

impl<T: chrono::TimeZone + miniserde::Deserialize + Copy> miniserde::Deserialize for DateTime<T> {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

impl<T: chrono::TimeZone + miniserde::Serialize + Copy> miniserde::Serialize for DateTime<T> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        todo!()
    }
}

impl<T: chrono::TimeZone + Copy> serde::Serialize for DateTime<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: chrono::TimeZone + Copy> serde::Deserialize<'de> for DateTime<T> {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        DateTime::deserialize(deserializer)
    }
}


impl<T: chrono::TimeZone + Copy> std::ops::Deref for DateTime<T> {
    type Target = chrono::DateTime<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


///fdsa
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Utc(pub chrono::Utc);

impl miniserde::Serialize for Utc {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        todo!()
    }
}

impl miniserde::Deserialize for Utc {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

impl std::ops::Deref for Utc {
    type Target = chrono::Utc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl chrono::Offset for Utc {
    fn fix(&self) -> chrono::FixedOffset {
        chrono::FixedOffset::east_opt(0).unwrap()
    }
}
impl chrono::TimeZone for Utc {
    type Offset = Utc;

    fn from_offset(_offset: &Self::Offset) -> Self {
        Utc(chrono::Utc)
    }

    fn offset_from_local_date(&self, _local: &chrono::NaiveDate) -> chrono::LocalResult<Self::Offset> {
        chrono::LocalResult::Single(Utc(chrono::Utc))
    }

    fn offset_from_local_datetime(&self, _local: &chrono::NaiveDateTime) -> chrono::LocalResult<Self::Offset> {
        chrono::LocalResult::Single(Utc(chrono::Utc))
    }

    fn offset_from_utc_date(&self, _utc: &chrono::NaiveDate) -> Self::Offset {
        Utc(chrono::Utc)
    }

    fn offset_from_utc_datetime(&self, _utc: &chrono::NaiveDateTime) -> Self::Offset {
        Utc(chrono::Utc)
    }
}
impl Utc {
    /// fdsa
    pub fn now() -> DateTime<Utc> {
        // pub struct DateTime<T: chrono::TimeZone + Copy>(pub chrono::DateTime<T>);

        let n = chrono::Utc::now();
        let g = Utc(n.timezone());
        let pp = n.with_timezone(&g);


        // let pp = chrono::DateTime(Utc(g));
        // let pp = chrono::DateTime::offset(g);

        DateTime(pp)
    }
}
// impl chrono::TimeZone for Utc {
//     type Offset = Utc;

//     fn from_offset(_state: &Utc) -> Utc {
//         Utc
//     }

//     fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<Utc> {
//         LocalResult::Single(Utc)
//     }
//     fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<Utc> {
//         LocalResult::Single(Utc)
//     }

//     fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Utc {
//         Utc
//     }
//     fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Utc {
//         Utc
//     }
// }










///fdsa
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NaiveDate(pub chrono::NaiveDate);

// impl miniserde::Serialize for NaiveDate {
//     fn begin(&self) -> miniserde::ser::Fragment<'_> {
//         self.0.to_string().begin()
//     }
// }
#[allow(non_upper_case_globals)]
const _IMPL_MINISERIALIZE_FOR_NaiveDate: () = {
    impl miniserde::Serialize for NaiveDate {
        fn begin(&self) -> miniserde::ser::Fragment {
            miniserde::ser::Fragment::Seq(miniserde::__private::Box::new(__Seq {
                data: self.0.to_string(),
                state: 0,
            }))
        }
    }
    struct __Seq {
        data: String,
        state: miniserde::__private::usize,
    }
    impl miniserde::ser::Seq for __Seq {
        fn next(&mut self) -> miniserde::__private::Option<&dyn miniserde::Serialize> {
            let __state = self.state;
            self.state = __state + 1;
            match __state {
                0usize => miniserde::__private::Some(&self.data),
                _ => miniserde::__private::None,
            }
        }
    }
};

impl miniserde::de::Visitor for Place<NaiveDate> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        self.out = Some(NaiveDate(chrono::NaiveDate::from_str(s).unwrap()));
        Ok(())
    }
}
impl miniserde::Deserialize for NaiveDate {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        Place::new(out)
    }
}
impl std::ops::Deref for NaiveDate {
    type Target = chrono::NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}









///fdsa
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Tz(pub chrono_tz::Tz);

// impl miniserde::Serialize for Tz {
//     fn begin(&self) -> miniserde::ser::Fragment<'_> {
//         self.0.to_string().begin()
//     }
// }
#[allow(non_upper_case_globals)]
const _IMPL_MINISERIALIZE_FOR_Tz: () = {
    impl miniserde::Serialize for Tz {
        fn begin(&self) -> miniserde::ser::Fragment {
            miniserde::ser::Fragment::Seq(miniserde::__private::Box::new(__Seq {
                data: (),
                state: 0,
            }))
        }
    }
    struct __Seq {
        data: (),
        state: miniserde::__private::usize,
    }
    impl miniserde::ser::Seq for __Seq {
        fn next(&mut self) -> miniserde::__private::Option<&dyn miniserde::Serialize> {
            let __state = self.state;
            self.state = __state + 1;
            match __state {
                0usize => miniserde::__private::Some(&self.data),
                _ => miniserde::__private::None,
            }
        }
    }
};

impl miniserde::de::Visitor for Place<Tz> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        self.out = Some(Tz(chrono_tz::Tz::from_str(s).unwrap()));
        Ok(())
    }
}
impl miniserde::Deserialize for Tz {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        Place::new(out)
    }
}
impl std::ops::Deref for Tz {
    type Target = chrono_tz::Tz;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}












miniserde::make_place!(Place);


///fdsa
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub struct Uuid(pub uuid::Uuid);

impl Uuid {
    fn from_u128(u: u128) -> Self {
        Uuid(uuid::Uuid::from_u128(u))
    }
}
impl FromStr for Uuid {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uuid(uuid::Uuid::from_str(s)?))
    } 
}
impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// impl miniserde::Serialize for Uuid {
//     fn begin(&self) -> miniserde::ser::Fragment<'_> {
//         // miniserde::Serialize(self.0)
//         let g = self.0.as_u64_pair();
//         let g = g.0.begin();
//         g
//     }
// }
#[allow(non_upper_case_globals)]
const _IMPL_MINISERIALIZE_FOR_Uuid: () = {
    impl miniserde::Serialize for Uuid {
        fn begin(&self) -> miniserde::ser::Fragment {
            miniserde::ser::Fragment::Seq(miniserde::__private::Box::new(__Seq {
                data: self.as_u64_pair(),
                state: 0,
            }))
        }
    }
    struct __Seq {
        data: (u64, u64),
        state: miniserde::__private::usize,
    }
    impl miniserde::ser::Seq for __Seq {
        fn next(&mut self) -> miniserde::__private::Option<&dyn miniserde::Serialize> {
            let __state = self.state;
            self.state = __state + 1;
            match __state {
                0usize => miniserde::__private::Some(&self.data),
                _ => miniserde::__private::None,
            }
        }
    }
};

impl miniserde::de::Visitor for Place<Uuid> {
    fn nonnegative(&mut self, num: u64) -> miniserde::Result<()> {
        self.out = Some(Uuid(uuid::Uuid::from_u128(num as u128)));
        Ok(())
    }
}
impl miniserde::Deserialize for Uuid {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        Place::new(out)
    }
}
impl std::ops::Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}




///fdsa
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Url(pub url::Url);

impl miniserde::Serialize for Url {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        // miniserde::Serialize(self.0)
        self.0.as_str().begin()
    }
}
impl miniserde::de::Visitor for Place<Url> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        self.out = Some(Url(url::Url::from_str(s).unwrap()));
        Ok(())
    }
}
impl miniserde::Deserialize for Url {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        Place::new(out)
    }
}
impl std::ops::Deref for Url {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}




#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
///fdsa
pub struct RGBA<T: Copy>(pub rgb::RGBA<T>);

impl<T: Copy> miniserde::Deserialize for RGBA<T> {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

impl<T: Copy> miniserde::Serialize for RGBA<T> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        todo!()
    }
}
///fdsa
pub type RGBA8 = RGBA<u8>;







///fdsa
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Duration(pub std::time::Duration);

#[allow(non_upper_case_globals)]
const _IMPL_MINISERIALIZE_FOR_Duration: () = {
    impl miniserde::Serialize for Duration {
        fn begin(&self) -> miniserde::ser::Fragment {
            miniserde::ser::Fragment::Seq(miniserde::__private::Box::new(__Seq {
                data: self.as_secs(),
                state: 0,
            }))
        }
    }
    struct __Seq {
        data: u64,
        state: miniserde::__private::usize,
    }
    impl miniserde::ser::Seq for __Seq {
        fn next(&mut self) -> miniserde::__private::Option<&dyn miniserde::Serialize> {
            let __state = self.state;
            self.state = __state + 1;
            match __state {
                0usize => miniserde::__private::Some(&self.data),
                _ => miniserde::__private::None,
            }
        }
    }
};



impl miniserde::de::Visitor for Place<Duration> {
    fn nonnegative(&mut self, s: u64) -> miniserde::Result<()> {
        self.out = Some(Duration(std::time::Duration::new(s, 0)));
        Ok(())
    }
}
impl miniserde::Deserialize for Duration {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        Place::new(out)
    }
}
impl std::ops::Deref for Duration {
    type Target = std::time::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}





// anyhow::Error
// Tz
