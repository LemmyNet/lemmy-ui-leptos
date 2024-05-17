# This script assumes docker can be run as a non-root user. See the following docs if the script fails because you need to use sudo:
# https://docs.docker.com/engine/install/linux-postinstall/#manage-docker-as-a-non-root-user

CONTANIER_NAME='end2end'
DOCKERFILE_PATH='end2end/docker-compose.yml'

# define some colors to use for output
RED='\033[0;31m'
NC='\033[0m'

# kill and remove any running containers
cleanup () {
  docker compose -p $CONTANIER_NAME kill
  docker compose -p $CONTANIER_NAME rm -f --all
}

# catch unexpected failures, do cleanup and output an error message
trap 'cleanup ; printf "${RED}Tests Failed For Unexpected Reasons${NC}\n"'\
  HUP INT QUIT PIPE TERM

# build and run the composed services
docker compose -p $CONTANIER_NAME -f $DOCKERFILE_PATH build
docker compose -p $CONTANIER_NAME -f $DOCKERFILE_PATH run $CONTANIER_NAME

cleanup