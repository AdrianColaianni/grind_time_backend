use crate::db;
use crate::error_handler::CustomError;
use crate::schema::entries;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug)]
enum Location {
    BarreHall, BrackettHall, ChandlerHall, CollegeOfBusiness, CooperLibrary, CribbHall,
    DanielHall, DesChampsHall, EarleHall, EdwardsHall, FikeRecCenter, FreemanHall,
    GressetteHall, HardinHall, Hendrix, HumanitiesHall, LehotskyHall, LongHall, LowryHall,
    MartinHall, McAlisterHall, MichelHall, NewmanHall, PoolCenter, RiggsHall, SikesHall,
    SirrineHall, VickeryHall, Watt,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, PartialEq, Debug)]
#[table_name = "entries"]
pub struct Entry {
    pub user_name: String,
    pub minutes: i32,
    pub location: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "entries"]
pub struct Entries {
    pub id: i32,
    pub user_name: String,
    pub minutes: i32,
    pub location: i32,
}

impl Entries {
    pub fn example() -> Self {
        Entries {
            id: 0,
            user_name: String::from("JohnFortnite"),
            location: Location::LongHall as i32,
            minutes: 274,   // 4 hrs 34 minutes
        }
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let entries = entries::table.load::<Entries>(&conn)?;
        Ok(entries)
    }

    pub fn find_location(location: i32) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let entries = entries::table.filter(entries::location.eq(location)).load::<Entries>(&conn)?;
        Ok(entries)
    }

    pub fn find_id(id: i32) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let entries = entries::table.filter(entries::id.eq(id)).load::<Entries>(&conn)?;
        Ok(entries)
    }

    // Get entry with longest duration
    // pub fn longest(location: i32) -> Result<Vec<Self>, CustomError> {
    //     let conn = db::connection()?;
    //     let entries = entries::table.filter(entries::location.eq(location));
    // }

    pub fn create(entry: Entry) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let entry = Entry::from(entry);
        let entry = diesel::insert_into(entries::table)
            .values(entry)
            .get_result(&conn)?;
        Ok(entry)
    }

    pub fn update(id: i32, entry: Entry) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let entry = diesel::update(entries::table)
            .filter(entries::id.eq(id))
            .set(entry)
            .get_result(&conn)?;
        Ok(entry)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(entries::table.filter(entries::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Entry {
    fn from(entry: Entry) -> Entry {
        Entry {
            user_name: entry.user_name,
            minutes: entry.minutes,
            location: entry.location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Entry, Location};

    #[test]
    fn entry_eq() {
        let entry1 = Entry {
            user_name: String::from("JohnFortnite"),
            location: Location::LongHall as i32,
            minutes: 274,   // 4 hrs 34 minutes
        };
        let entry2 = Entry {
            user_name: String::from("JohnFortnite"),
            location: Location::LongHall as i32,
            minutes: 274,   // 4 hrs 34 minutes
        };

        assert!(entry1.eq(&entry2));
    }

    #[test]
    fn entry_ne() {
        let entry1 = Entry {
            user_name: String::from("JohnFortnite"),
            location: Location::LongHall as i32,
            minutes: 274,   // 4 hrs 34 minutes
        };
        let entry2 = Entry {
            user_name: String::from("JoeFortnite"),
            location: Location::LongHall as i32,
            minutes: 274,   // 4 hrs 34 minutes
        };

        assert!(!entry1.eq(&entry2));
    }

    #[test]
    fn entry_display() {
        assert_eq!("CooperLibrary", format!("{:?}", Location::CooperLibrary));
    }
}
