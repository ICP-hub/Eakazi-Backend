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
dfx canister call ea_backend get "(\"8f5f4267afed8662d960f1dc310c4becc19839c2733dfe37d7ce55c52f9a4e11\")"

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

# Minting Certificate
echo "Minting Certificate..."
dfx canister call ea_backend mint_certificate "(\"g2sm3-5rre3-ofvqj-lpsje-voulv-66ezn-kktlg-dgzdr-iiva7-zgc7e-sae\", \"6a95d846fdda8554090b813f5b3d3b341c5de7217fa121524b36fe21a836c5ba\", \"description\", \"tag\", \"a36e668f5739c5f72052a0d2ba43468f733e7c03c0d9e4cd9b91996aa0d89be1\", \"iVBORw0KGgoAAAANSUhEUgAAB9AAAAWGCAYAAADXcD66AAAAAXNSR0IArs4c6QAAAARzQklUCAgICHwIZIgAAAAJcEhZcwAAGkwAABpMARwsOZwAAASJaVRYdFhNTDpjb20uYWRvYmUueG1wAAAAAAA8P3hwYWNrZXQgYmVnaW49J++7vycgaWQ9J1c1TTBNcENlaGlIenJlU3pOVGN6a2M5ZCc/Pgo8eDp4bXBtZXRhIHhtbG5zOng9J2Fkb2JlOm5zOm1ldGEvJz4KPHJkZjpSREYgeG1sbnM6cmRmPSdodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjJz4KCiA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0nJwogIHhtbG5zOkF0dHJpYj0naHR0cDovL25zLmF0dHJpYnV0aW9uLmNvbS9hZHMvMS4wLyc+CiAgPEF0dHJpYjpBZHM+CiAgIDxyZGY6U2VxPgogICAgPHJkZjpsaSByZGY6cGFyc2VUeXBlPSdSZXNvdXJjZSc+CiAgICAgPEF0dHJpYjpDcmVhdGVkPjIwMjQtMDItMjc8L0F0dHJpYjpDcmVhdGVkPgogICAgIDxBdHRyaWI6RXh0SWQ+NTE0MGUzNzgtYjk3MS00ZmM1LWEyZmEtOGE2OWNlZTcxZjUzPC9BdHRyaWI6RXh0SWQ+CiAgICAgPEF0dHJpYjpGYklkPjUyNTI2NTkxNDE3OTU4MDwvQXR0cmliOkZiSWQ+CiAgICAgPEF0dHJpYjpUb3VjaFR5cGU+MjwvQXR0cmliOlRvdWNoVHlwZT4KICAgIDwvcmRmOmxpPgogICA8L3JkZjpTZXE+CiAgPC9BdHRyaWI6QWRzPgogPC9yZGY6RGVzY3JpcHRpb24+CgogPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9JycKICB4bWxuczpkYz0naHR0cDovL3B1cmwub3JnL2RjL2VsZW1lbnR\")"