export DOCKER_BUILDKIT=1
args:=
composer:=podman-compose

dev-compose:
	$(composer) -f docker-compose.yml -f docker-compose.dev.yml $(args)

prod-compose:
	$(composer) -f docker-compose.yml -f docker-compose.prod.yml $(args)

dev:
	make dev-compose args="build"
	make dev-compose args="up -d"

prod:
	make prod-compose args="build"
	make prod-compose args="up -d"
	
