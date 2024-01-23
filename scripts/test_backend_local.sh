dfx start --background

set -e
dfx identity use dev
echo "Running tests for backend"

# Run tests
dev=$(dfx --identity dev identity get-principal)
echo "Dev principal: $dev"

function check_cycles() {
    echo "Checking cycles..."
    CYCLES=$(dfx wallet balance)
    IDENTITY=$(dfx identity whoami)
    echo "Cycles: $CYCLES"
    echo "Identity: $IDENTITY"
}
check_cycles

CANISTER_NAME="ea_backend"
CANISTER_ID=$(dfx canister id $CANISTER_NAME)

# If the canister is not deployed, deploy it
# if [[ -z "$CANISTER_ID" ]]; then
    # Install dependencies
    echo "Installing dependencies..."
    npm install
    # Deploy the canisters locally
    dfx deploy
#  else
#     echo "Canister already deployed"
#  fi

# CheckUser
echo "Checking user..."
dfx canister call ea_backend checkUser
 
# Create a new user
echo "Creating a new user..."
dfx canister call ea_backend createUser "(\"Alice\", \"testing@testing\", \"trainer\")"

# Get full name
echo "Getting full name..."
dfx canister call ea_backend getFullName

# Get Role
echo "Getting role..."
dfx canister call ea_backend getRole

# Get the user profile record of self
echo "Getting the user profile record..."
dfx canister call ea_backend getSelf

# Get the user profile record by id
echo "Getting the user profile by id..."
dfx canister call ea_backend get "(\"57161a8133dc2f502b4cc6d5cc058e39da218eb95660d2dbe7e6043e1fadd354\")"

# Update the user profile 
echo "Updating the user profile..."
dfx canister call ea_backend update '(record {id="57161a8133dc2f502b4cc6d5cc058e39da218eb95660d2dbe7e6043e1fadd354"; occupation="Engineer"; resume=vec {0; 0; 0}; role=variant {TRAINER}; description="Experienced Dev"; email="test@test.com"; fullname="John Jacobs"; keywords=vec {"Communication"}; organization="TestOrps"; skills=vec {"Programming"}; location="California"})'

# Search for profiles
echo "Searching for profile..."
dfx canister call ea_backend search "(\"John\")"

# Create a course
echo "Creating a course..."
dfx canister call ea_backend createCourse "(\"Blockchain\")"

# Get all courses by user
echo "Getting all courses by user..."
dfx canister call ea_backend getCoursesByCreator

# Get a course 
echo "Getting a course..."
dfx canister call ea_backend getCourse "(\"055bbb72eec21996b30edab2a1b7ea7e56399d120f063640c1dc0c26eeb24808\")"

# Get all courses
echo "Getting all courses..."
dfx canister call ea_backend getAllCourses 

# Apply for a course
echo "Applying for a course..."
dfx canister call ea_backend applyCourse "(\"055bbb72eec21996b30edab2a1b7ea7e56399d120f063640c1dc0c26eeb24808\")"

# Apply for a job
echo "Applying for a job..."
dfx canister call ea_backend applyJobs "(\"056e4f166dd5a8dd6256030798c05c33d740d4762c6d9807d0b96bc070ed0e89\")"

# Create a job
echo "Creating a job..."
dfx canister call ea_backend createJob "(\"Designer\")"

# Get all jobs
echo "Getting all jobs..."
dfx canister call ea_backend getAllJobs

#Get jobs by creator
echo "Getting jobs by creator..."
dfx canister call ea_backend getJobsByCreator

# Check if applied for job
echo "Checking applied job..."
dfx canister call ea_backend checkAppliedJob "(\"056e4f166dd5a8dd6256030798c05c33d740d4762c6d9807d0b96bc070ed0e89\")"

# Check if registered for course
echo "Checking registered course..."
dfx canister call ea_backend checkAppliedCourse "(\"055bbb72eec21996b30edab2a1b7ea7e56399d120f063640c1dc0c26eeb24808\")"

# Get job applications count for a user
echo "Getting job applications count..."
dfx canister call ea_backend getJobsAppliedCount

# Get courses registered by a user
echo "Getting courses registered by a user..."
dfx canister call ea_backend getCoursesRegisteredByUser