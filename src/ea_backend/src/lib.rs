pub mod certificate;

use crate::certificate::mint;
use candid::{CandidType, Nat, Principal};
use certificate::types::{GenericValue, NftError};
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::management_canister::main::raw_rand;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::{
    cell::RefCell,
    sync::atomic::{AtomicU64, Ordering},
};

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;
type CourseStore = BTreeMap<String, Course>;
type JobStore = BTreeMap<String, Jobs>;

thread_local! {
    static CHECK_USER_STORE: RefCell<Vec<CheckUser>> = RefCell::default();
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    static COURSE_STORE : RefCell<CourseStore> = RefCell::default();
    static JOB_STORE : RefCell<JobStore> = RefCell::default();
}

// ==================================================================================================
// Stable Storage
// ==================================================================================================

//  Making the stable storage upgradeable
#[pre_upgrade]
fn pre_upgrade() {
    // Serializing and saving all the stores
    let serialized_check_user_store =
        serde_cbor::to_vec(&CHECK_USER_STORE.with(|store| store.borrow().clone()))
            .expect("Failed to serialize check_user_store");
    let serialized_profile_store =
        serde_cbor::to_vec(&PROFILE_STORE.with(|store| store.borrow().clone()))
            .expect("Failed to serialize profile_store");
    let serialized_id_store = serde_cbor::to_vec(&ID_STORE.with(|store| store.borrow().clone()))
        .expect("Failed to serialize id_store");
    let serialized_course_store =
        serde_cbor::to_vec(&COURSE_STORE.with(|store| store.borrow().clone()))
            .expect("Failed to serialize course_store");
    let serialized_job_store = serde_cbor::to_vec(&JOB_STORE.with(|store| store.borrow().clone()))
        .expect("Failed to serialize job_store");

    // Saving the serialized data to stable storage
    ic_cdk::storage::stable_save((
        serialized_check_user_store,
        serialized_profile_store,
        serialized_id_store,
        serialized_course_store,
        serialized_job_store,
    ))
    .expect("Failed to save to stable storage");
}

#[post_upgrade]
fn post_upgrade() {
    // Restoring the serializing data from stable storage
    let (
        serialized_check_user_store,
        serialized_profile_store,
        serialized_id_store,
        serialized_course_store,
        serialized_job_store,
    ): (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) = match ic_cdk::storage::stable_restore() {
        Ok(data) => data,
        Err(e) => {
            ic_cdk::api::print(format!("Failed to restore from stable storage: {:?}", e));
            return;
        }
    };

    // Deserializing the data and populating the stores
    let check_user_store: Vec<CheckUser> =
        serde_cbor::from_slice(&serialized_check_user_store).unwrap_or_else(|_| Vec::new());
    let profile_store: ProfileStore =
        serde_cbor::from_slice(&serialized_profile_store).unwrap_or_else(|_| BTreeMap::new());
    let id_store: IdStore =
        serde_cbor::from_slice(&serialized_id_store).unwrap_or_else(|_| BTreeMap::new());
    let course_store: CourseStore =
        serde_cbor::from_slice(&serialized_course_store).unwrap_or_else(|_| BTreeMap::new());
    let job_store: JobStore =
        serde_cbor::from_slice(&serialized_job_store).unwrap_or_else(|_| BTreeMap::new());

    CHECK_USER_STORE.with(|store| *store.borrow_mut() = check_user_store);
    PROFILE_STORE.with(|store| *store.borrow_mut() = profile_store);
    ID_STORE.with(|store| *store.borrow_mut() = id_store);
    COURSE_STORE.with(|store| *store.borrow_mut() = course_store);
    JOB_STORE.with(|store| *store.borrow_mut() = job_store);
}

// ==================================================================================================
// Structs
// ==================================================================================================

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Reviews{
pub ratings : u8,// to be translated to stars in the app max of 5 ,min of 1
pub reviewer : String,
pub title : String,
pub image : Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Profile {
    pub id: String,
    pub principal_id: Principal,
    pub fullname: String,
    pub email: String,
    pub occupation: String,
    pub organization: String,
    pub location: String,
    pub resume: Vec<u8>,
    pub role: Roles,
    pub description: String,
    pub keywords: Vec<String>,
    pub skills: Vec<String>,
    pub token_ids: Vec<(Nat, String)>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: Default::default(),
            principal_id: ic_cdk::api::caller(),
            fullname: Default::default(),
            email: Default::default(),
            occupation: Default::default(),
            organization: Default::default(),
            location: Default::default(),
            resume: Default::default(),
            role: Roles::ADMIN,
            description: Default::default(),
            keywords: Default::default(),
            skills: Default::default(),
            token_ids: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Course {
    pub id: String,
    pub title: String,
    pub creator: Principal,
    pub creator_fullname: String,
    pub applicants: Vec<Principal>,
}
impl Default for Course {
    fn default() -> Self {
        Self {
            id: Default::default(),
            title: Default::default(),
            creator: ic_cdk::api::caller(),
            creator_fullname: Default::default(),
            applicants: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Jobs {
    pub id: String,
    pub title: String,
    pub creator: Principal,
    pub creator_fullname: String,
    pub applicants: Vec<Principal>,
}
impl Default for Jobs {
    fn default() -> Self {
        Self {
            id: Default::default(),
            title: Default::default(),
            creator: ic_cdk::api::caller(),
            creator_fullname: Default::default(),
            applicants: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct CheckUser {
    pub user: Principal,
}
impl Default for CheckUser {
    fn default() -> Self {
        Self {
            user: ic_cdk::api::caller(),
        }
    }
}

// Role enum
#[derive(CandidType, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub enum Roles {
    FREELANCER,
    #[default]
    EMPLOYER,
    TRAINER,
    ADMIN,
}
impl Roles {
    pub fn from_str(el: &str) -> Roles {
        match el.to_lowercase().as_str() {
            "trainee" => Roles::FREELANCER,
            "employer" => Roles::EMPLOYER,
            "trainer" => Roles::TRAINER,
            _ => Roles::ADMIN,
        }
    }
}

// ==================================================================================================
// User Functions
// ==================================================================================================

// Check if the principal exists in the vector
#[update]
async fn check_user() -> bool {
    let user = ic_cdk::api::caller();
    let exists = CHECK_USER_STORE.with(|check_user_store| {
        check_user_store
            .borrow()
            .iter()
            .any(|check_user| check_user.user == user)
    });

    // If the principal does not exist, store it in the vector
    if !exists {
        CHECK_USER_STORE.with(|check_user_store| {
            check_user_store.borrow_mut().push(CheckUser { user });
        });
        false
    } else {
        true
    }
}

// Create a new profile
#[update]
async fn create_user(fullname: String, email: String, role: String) -> Profile {
    let principal_id = ic_cdk::api::caller();

    let uid = raw_rand().await.unwrap().0;
    let uid = format!("{:x}", Sha256::digest(&uid));
    let f = uid.clone();
    let id = uid.clone().to_string();
    let m = fullname.clone();
    let e = email.clone();
    ID_STORE.with(|el| el.borrow_mut().insert(uid.to_string(), principal_id));
    PROFILE_STORE.with(|el| {
        el.borrow_mut().insert(
            principal_id,
            Profile {
                id: f.to_string(),
                principal_id: principal_id,
                fullname,
                email,
                role: Roles::from_str(&role),
                ..Default::default()
            },
        )
    });

    Profile {
        id,
        principal_id,
        fullname: m,
        email: e,
        role: Roles::from_str(&role),
        ..Default::default()
    }
}

// Get full name of the user
#[query]
fn get_full_name() -> String {
    let principal_id = ic_cdk::api::caller();
    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&principal_id)
            .map(|profile| profile.fullname.clone())
            .unwrap_or_default()
    })
}

// Get role of the user
#[query]
fn get_role() -> String {
    let principal_id = ic_cdk::api::caller();
    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&principal_id)
            .map(|profile| format!("{:?}", profile.role))
            .unwrap_or_default()
    })
}

#[query]
fn get_self() -> Profile {
    let id = ic_cdk::api::caller();
    PROFILE_STORE.with(|profile_store| profile_store.borrow().get(&id).cloned().unwrap_or_default())
}

#[query]
fn get(uid: String) -> Profile {
    ID_STORE.with(|id_store| {
        PROFILE_STORE.with(|profile_store| {
            id_store
                .borrow()
                .get(&uid)
                .and_then(|id| profile_store.borrow().get(id).cloned())
                .unwrap_or_default()
        })
    })
}

#[update]
fn update(profile: Profile) {
    let principal_id = ic_cdk::api::caller();

    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow_mut()
            .entry(principal_id)
            .and_modify(|el| {
                *el = Profile { ..profile };
            });
    });
}

#[query(manual_reply = true)]
fn search(text: String) -> ManualReply<Option<Profile>> {
    let text = text.to_lowercase();
    PROFILE_STORE.with(|profile_store| {
        for (_, p) in profile_store.borrow().iter() {
            if p.fullname.to_lowercase().contains(&text)
                || p.description.to_lowercase().contains(&text)
            {
                return ManualReply::one(Some(p));
            }

            for x in p.keywords.iter() {
                if x.to_lowercase() == text {
                    return ManualReply::one(Some(p));
                }
            }
        }
        ManualReply::one(None::<Profile>)
    })
}

// Getting all freelancers
#[query]
fn get_all_freelancers() -> Vec<Profile> {
    let mut freelancers = Vec::new();
    let principal_id = ic_cdk::api::caller();
    let mut m: Profile = Default::default();
    PROFILE_STORE.with(|el| m = el.borrow().get(&principal_id).unwrap().clone());
    assert!(m.role == Roles::EMPLOYER);

    PROFILE_STORE.with(|store| {
        let store_borrowed = store.borrow();
        for profile in store_borrowed.values() {
            if profile.role == Roles::ADMIN {
                freelancers.push(profile.clone());
            }
        }
    });

    freelancers
}

// ==================================================================================================
// Course related functions
// ==================================================================================================

// Create a new Course
#[update]
async fn create_course(title: String) -> Course {
    let principal_id = ic_cdk::api::caller();
    let mut m: Profile = Default::default();
    PROFILE_STORE.with(|el| m = el.borrow().get(&principal_id).unwrap().clone());
    assert!(m.role == Roles::TRAINER);
    let uid = raw_rand().await.unwrap().0;
    let uid = format!("{:x}", Sha256::digest(&uid));

    let c = title.clone();
    let d = uid.clone();
    let e = uid.clone();

    COURSE_STORE.with(|el| {
        el.borrow_mut().insert(
            uid,
            Course {
                id: d,
                title: title.to_string(),
                creator: principal_id,
                creator_fullname: m.fullname.to_string(),
                applicants: vec![],
            },
        )
    });

    Course {
        id: e,
        title: title.to_string(),
        creator: principal_id,
        creator_fullname: m.fullname.to_string(),
        applicants: vec![],
    }
}

// Get all the courses by the creator
#[update]
fn get_courses_by_creator() -> Vec<Course> {
    let principal_id = ic_cdk::api::caller();
    let mut courses_by_creator = Vec::new();

    COURSE_STORE.with(|store| {
        let store_borrowed = store.borrow();
        for course in store_borrowed.values() {
            if course.creator == principal_id {
                courses_by_creator.push(course.clone());
            }
        }
    });

    courses_by_creator.reverse();

    courses_by_creator
}

// Get a specific course by id
#[query]
fn get_course(id: String) -> Course {
    COURSE_STORE.with(|el| el.borrow().get(&id).cloned().unwrap())
}

// Get all courses
#[query]
fn get_all_courses() -> Vec<Course> {
    let mut courses = Vec::new();
    COURSE_STORE.with(|store| {
        let store_borrowed = store.borrow();
        for course in store_borrowed.values() {
            courses.push(course.clone());
        }
    });

    courses.reverse();

    courses
}

#[update]
fn apply_course(id: String) -> Option<Course> {
    let principal_id = ic_cdk::api::caller();
    let mut course_opt = None;

    COURSE_STORE.with(|el| {
        let mut store = el.borrow_mut();
        if let Some(course) = store.get_mut(&id) {
            if !course.applicants.contains(&principal_id) {
                course.applicants.push(principal_id);
            }
            course_opt = Some(course.clone());
        }
    });

    course_opt
}

// Check if the user has applied for the course
#[query]
fn check_applied_course(course_id: String) -> bool {
    let principal_id = ic_cdk::api::caller();

    let result = COURSE_STORE.with(|store| {
        store
            .borrow()
            .get(&course_id)
            .map_or(false, |course| course.applicants.contains(&principal_id))
    });

    result
}

// Get registered courses for the user
#[update]
fn get_courses_registered_by_user() -> Vec<Course> {
    let principal_id = ic_cdk::api::caller();
    let mut registered_courses = Vec::new();

    COURSE_STORE.with(|store| {
        let store_borrowed = store.borrow();
        for course in store_borrowed.values() {
            if course.applicants.contains(&principal_id) {
                registered_courses.push(course.clone());
            }
        }
    });

    registered_courses.reverse();

    registered_courses
}

// Getting course registered students
#[query]
fn get_course_applicants(course_id: String) -> Vec<Profile> {
    let mut applicants = Vec::new();
    COURSE_STORE.with(|store| {
        let store_borrowed = store.borrow();
        if let Some(course) = store_borrowed.get(&course_id) {
            for applicant in &course.applicants {
                if let Some(profile) = PROFILE_STORE
                    .with(|profile_store| profile_store.borrow().get(applicant).cloned())
                {
                    applicants.push(profile);
                }
            }
        }
    });

    applicants
}

// ==================================================================================================
// Job related functions
// ==================================================================================================

#[update]
fn apply_jobs(id: String) -> Option<Jobs> {
    let principal_id = ic_cdk::api::caller();
    let mut job_opt = None;

    JOB_STORE.with(|el| {
        let mut store = el.borrow_mut();
        if let Some(job) = store.get_mut(&id) {
            if !job.applicants.contains(&principal_id) {
                job.applicants.push(principal_id);
            }
            job_opt = Some(job.clone());
        }
    });

    job_opt
}

// Create a new Job
#[update]
async fn create_job(title: String) -> Jobs {
    let principal_id = ic_cdk::api::caller();
    let mut m: Profile = Default::default();
    PROFILE_STORE.with(|el| m = el.borrow().get(&principal_id).unwrap().clone());
    assert!(m.role == Roles::EMPLOYER);
    let uid = raw_rand().await.unwrap().0;
    let uid = format!("{:x}", Sha256::digest(&uid));

    let c = title.clone();
    let d = uid.clone();
    let e = uid.clone();

    JOB_STORE.with(|el| {
        el.borrow_mut().insert(
            uid,
            Jobs {
                id: d,
                title: title.to_string(),
                creator: principal_id,
                creator_fullname: m.fullname.to_string(),
                applicants: vec![],
            },
        )
    });

    Jobs {
        id: e,
        title: title.to_string(),
        creator: principal_id,
        creator_fullname: m.fullname.to_string(),
        applicants: vec![],
    }
}

// Get all jobs
#[query]
fn get_all_jobs() -> Vec<Jobs> {
    let mut jobs = Vec::new();
    JOB_STORE.with(|store| {
        let store_borrowed = store.borrow();
        for job in store_borrowed.values() {
            jobs.push(job.clone());
        }
    });

    jobs.reverse();

    jobs
}

// Get all jobs by the creator
#[update]
fn get_jobs_by_creator() -> Vec<Jobs> {
    let principal_id = ic_cdk::api::caller();
    let mut jobs_by_creator = Vec::new();

    JOB_STORE.with(|store| {
        let store_borrowed = store.borrow();
        for jobs in store_borrowed.values() {
            if jobs.creator == principal_id {
                jobs_by_creator.push(jobs.clone());
            }
        }
    });

    jobs_by_creator.reverse();

    jobs_by_creator
}

// Check if the user has applied for the job
#[query]
fn check_applied_job(job_id: String) -> bool {
    let principal_id = ic_cdk::api::caller();

    let result = JOB_STORE.with(|store| {
        store
            .borrow()
            .get(&job_id)
            .map_or(false, |job| job.applicants.contains(&principal_id))
    });

    result
}

// Get job count for the user
#[query]
fn get_jobs_applied_count() -> u32 {
    let principal_id = ic_cdk::api::caller();
    let mut jobs_applied_count = 0;

    JOB_STORE.with(|store| {
        let store_borrowed = store.borrow();
        for job in store_borrowed.values() {
            if job.applicants.contains(&principal_id) {
                jobs_applied_count += 1;
            }
        }
    });

    jobs_applied_count
}

// Getting job applicants
#[query]
fn get_job_applicants(job_id: String) -> Vec<Profile> {
    let mut applicants = Vec::new();
    JOB_STORE.with(|store| {
        let store_borrowed = store.borrow();
        if let Some(job) = store_borrowed.get(&job_id) {
            for applicant in &job.applicants {
                if let Some(profile) = PROFILE_STORE
                    .with(|profile_store| profile_store.borrow().get(applicant).cloned())
                {
                    applicants.push(profile);
                }
            }
        }
    });

    applicants
}

// ==================================================================================================
// Rating and reviews
// ==================================================================================================

// #[update]
// fn addReviews(ratings : u8  ,title: String, image : Option<Vec<u8>>,profile_reviewed : String){
//  let mut profile = get(profile_reviewed);
//  let  me = get_self();
//  if let Some (el) = image {
//     let r = Reviews{ ratings, reviewer:  me.fullname, title, image: el };
//     profile.reviews.push(r);

   
//  }else {
//     let r = Reviews{ ratings, reviewer: me.fullname, title, image : vec![0] };
//     profile.reviews.push(r);
//  }

// }

// #[update]
// fn get_all_reviews() -> Vec<Reviews>{
// get_self().reviews
// }

// ==================================================================================================
// NFT
// ==================================================================================================

// // ======================
// //      UPDATE CALLS
// // ======================

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn generate_unique_nat() -> Nat {
    // Increment the counter safely
    let counter_value = COUNTER.fetch_add(1, Ordering::SeqCst);
    // Convert the counter value directly to Nat
    Nat::from(counter_value)
}

// Minting certificate to the students
#[update]
async fn mint_certificate(
    to: String,
    description: String,
    tag: String,
    course_id: String,
    certificate: String,
) -> Result<Nat, NftError> {
    let to = Principal::from_text(to).unwrap();

    let token_identifier = generate_unique_nat();

    let token_id_tuple = (token_identifier.clone(), course_id.clone());

    PROFILE_STORE.with(|store| {
        if let Some(profile) = store.borrow_mut().get_mut(&to) { 
            if !profile.token_ids.contains(&token_id_tuple) {
                profile.token_ids.push(token_id_tuple);
            }
        }
    });

    // Assigning metadata values
    let properties = vec![
        (
            "description".to_string(),
            GenericValue::TextContent(description.to_string()),
        ),
        (
            "tag".to_string(),
            GenericValue::TextContent(tag.to_string()),
        ),
        (
            "course_id".to_string(),
            GenericValue::TextContent(course_id.to_string()),
        ),
        (
            "certificate".to_string(),
            GenericValue::TextContent(certificate.to_string()),
        ),
    ];

    mint(to, token_identifier, properties)
}

ic_cdk::export_candid!();
