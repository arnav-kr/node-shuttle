# Addition dependencies or actions to do before the actual container starts

# install nodejs (v23)
curl -fsSL https://deb.nodesource.com/setup_23.x -o nodesource_setup.sh
bash nodesource_setup.sh
apt-get update
apt-get install -y nodejs