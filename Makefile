init:
	docker-compose -f docker-compose.dev.yml up --build
up:
	docker-compose -f docker-compose.dev.yml up
down:
	docker-compose -f docker-compose.dev.yml down
sh-back:
	docker-compose -f docker-compose.dev.yml exec back sh