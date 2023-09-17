docker:
	docker image build -t railway-symphony .
cu:
	docker compose up

cd:
	docker compose down

cdu:
	make cd && make cu
clean:
	docker rmi $(docker image ls | grep "<none>" | awk '{print $3}')
