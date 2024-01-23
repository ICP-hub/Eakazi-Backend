use candid::{CandidType, Principal};
use ic_cdk::api::call::ManualReply;
use ic_cdk::api::management_canister::main::raw_rand;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;
type CourseStore = BTreeMap<String, Course>;
type JobStore = BTreeMap<String, Jobs>;
use sha2::{Digest, Sha256};

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

// Struct of the data to be stored in the stable memory

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
struct Profile {
    pub id: String,
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

thread_local! {
    static CHECK_USER_STORE: RefCell<Vec<CheckUser>> = RefCell::default();
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    static COURSE_STORE : RefCell<CourseStore> = RefCell::default();
    static JOB_STORE : RefCell<JobStore> = RefCell::default();
}

// Check if the principal exists in the vector
#[update]
async fn checkUser() -> bool {
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
async fn createUser(fullname: String, email: String, role: String) -> Profile {
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
                fullname,
                email,
                role: Roles::from_str(&role),
                ..Default::default()
            },
        )
    });

    Profile {
        id,
        fullname: m,
        email: e,
        role: Roles::from_str(&role),
        ..Default::default()
    }
}

// Get full name of the user
#[query]
fn getFullName() -> String {
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
fn getRole() -> String {
    let principal_id = ic_cdk::api::caller();
    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&principal_id)
            .map(|profile| format!("{:?}", profile.role))
            .unwrap_or_default()
    })
}

#[query(name = "getSelf")]
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

// Create a new Course
#[update]
async fn createCourse(title: String) -> Course {
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
fn getCoursesByCreator() -> Vec<Course> {
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
fn getCourse(id: String) -> Course {
    COURSE_STORE.with(|el| el.borrow().get(&id).cloned().unwrap())
}

// Get all courses
#[query]
fn getAllCourses() -> Vec<Course> {
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
fn applyCourse(id: String) -> Option<Course> {
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

#[update]
fn applyJobs(id: String) -> Option<Jobs> {
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
async fn createJob(title: String) -> Jobs {
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
fn getAllJobs() -> Vec<Jobs> {
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
fn getJobsByCreator() -> Vec<Jobs> {
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
fn checkAppliedJob(job_id: String) -> bool {
    let principal_id = ic_cdk::api::caller();

    let result = JOB_STORE.with(|store| {
        store
            .borrow()
            .get(&job_id)
            .map_or(false, |job| job.applicants.contains(&principal_id))
    });

    result
}

// Check if the user has applied for the course
#[query]
fn checkAppliedCourse(course_id: String) -> bool {
    let principal_id = ic_cdk::api::caller();

    let result = COURSE_STORE.with(|store| {
        store
            .borrow()
            .get(&course_id)
            .map_or(false, |course| course.applicants.contains(&principal_id))
    });

    result
}

// Get job count for the user
#[query]
fn getJobsAppliedCount() -> u32 {
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

// Get registered courses for the user
#[update]
fn getCoursesRegisteredByUser() -> Vec<Course> {
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

ic_cdk::export_candid!();
