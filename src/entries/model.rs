use crate::db;
use crate::error_handler::CustomError;
use crate::schema::entries;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::naive::NaiveTime;

enum Location {
    AdministrativeServicesBuilding, AgriculturalServiceCenter, AlumniCenter, BarnettHall, BarreHall,
    BenetHall, BiologicalSciencesFieldStation, BiosystemsResearchComplex, BotanicalGarden,  BowenHall,
    BrackettHall, BradleyHall, BrooksCenterforthePerformingArts, ByrnesHall, CalhounCourtsApartments,
    CalhounsOffice, CampbellCarriageHouseCoffeeandGiftShop, CampbellGeologyMuseum,
    CampbellMuseumofNaturalHistory, CarillonGarden,
    CenterfortheImprovementofConstructionandManagementProcesses, CentralEnergyFacilities, Classof1957Rotunda,
    ClemsonHouse, ClemsonMemorialStadiumandFrankHowardField, ClemsonStatue,
    ClemsonUniversityFoundationShirleyCenterforPhilanthropy, CookAgriculturalServiceLaboratory,
    CookEngineeringLaboratory, CooperLibrary, CopeHall, CoxPlaza, DanielHall, DillardBuilding, DonaldsonHall,
    EarleHall, EdwardsHall, EndocrinePhysiologyLaboratory, FernowStreetCafe, FikeRecreationCenter,
    FireStation, FluorDanielEngineeringInnovationBuilding, FortHill, FranHansonDiscoveryCenter, FreemanHall,
    GanttCircle, GeerHall, GentryHall, GodfreyHall, GodleySnellResearchCenter, GreenhouseComplex,
    HanoverHouse, HarcombeDiningHall, HardinHall, HaydenConferenceCenter, HendrixStudentCenter, HolmesHall,
    HoltzendorffHall, Hopewell, HousingMaintenanceFacility, HoustonCenter, HunterChemistryLaboratory,
    IptayTicketOffice, IndoorTrack, InternalAuditingOffice, JerveyAthleticCenter, JohnstoneHall, JordanHall,
    KinardLaboratoryPhysics, KiteHillRecyclingCenter, LaMasterDairyCenter, LeeHall, LehotskyHall, LeverHall,
    LightseyBridgeIApartments, LightseyBridgeIIApartments, LittlejohnColiseum, LittlejohnHouse, LongHall,
    LowryHall, MadrenCenter, MaintenanceStores, ManningHall, MartinHall, MauldinHall, McAdamsHall,
    McCabeHall, McFaddenBuilding, MellHall, MilitaryHeritagePlaza, MoormanHouse, MorganPoultryCenter,
    MotorPool, NationalDropoutPreventionCenter, NewmanHall, NormanTrack, NorrisHall, OlinHall,
    OutdoorTheatre, PlantGermplasmResearchLaboratory, PoliceDepartment, PooleAgriculturalCenter,
    PresidentsHome, RedfernHealthCenter, ReunionSquare, RhodesAnnex, RhodesEngineeringResearchCenter,
    RiggsField, RiggsHall, RoderickInternationalHouse, FoundationSeed, SandersHall, SchilletterDiningHall,
    ScrollofHonorMemorial, SearsHouse, SheepBarn, SikesHall, SimpsonHallNorth, SimpsonHallSouth, SirrineHall,
    SloanTennisCenter, SmithBuildingforSonocoInstituteofPackagingDesignandGraphics, SmithHall,
    StadiumResidenceHall, StrodeTower, ThornhillVillageApartments, ThurmondInstitute, TigerField,
    TillmanHall, TrusteeHouse, UniversityFacilitiesEnvironmentalHealthandSafety,
    UniversityFacilitiesOperations, UniversityUnion, EdgarBrown, VickeryHall, VisitorsCenter, Classof1944,
    WalkerGolfCourseClubhouse, WannamakerHall, WomensRowingBoathouses, WoodlandCemetery, YoungHall
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "entries"]
pub struct Entry {
    pub user_name: String,
    pub duration: NaiveTime,
    pub location: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "entries"]
pub struct Entries {
    pub id: i32,
    pub user_name: String,
    pub duration: NaiveTime,
    pub location: i32,
}

impl Entries {
    pub fn example() -> Self {
        Entries {
            id: 0,
            user_name: String::from("JohnFortnite"),
            location: 6,
            duration: chrono::NaiveTime::from_hms_opt(4, 34, 15).unwrap()
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
            duration: entry.duration,
            location: entry.location,
        }
    }
}
