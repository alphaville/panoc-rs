set -euxo pipefail

main() {
    if [ -v ${DO_DOCKER+set} ]
    then
        #rustup toolchain remove stable && rustup toolchain install stable
        #sudo pip install --upgrade pip
        #sudo pip install virtualenv --upgrade --ignore-installed six

    fi
}

main
