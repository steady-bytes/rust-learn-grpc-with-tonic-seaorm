.PHONY: db db-image start-db exec-db migrate-db clean-db db-env-setup

#######################
### SQL DEVELOPMENT ### 
#######################

# db configuration variables
DB_NAME=users
DB_USER=services
DB_NETWORK=sql
MASTER_NODE=sql-db-master-node
WORKER_NODES = \
	sql-db-worker-node-1 \
	sql-db-worker-node-2 \
	sql-db-worker-node-3

#################################
### One time database local setup
.PHONY: db-env-setup

# create local db configuration files
db-env-setup:
	echo "DATABASE_URL=postgres://$(DB_USER)@localhost:26257/users" > .env && \
	echo "# For documentation on how to configure this file, \n# see diesel.rs/guides/configuring-diesel-cli \n\n[print_schema] \nfile = \"./src/dao/schema.rs\"" > diesel.toml

#####################
### Local development
.PHONY: db-run db-local-terminate

# todo -> add back in the start-worker-node target
# start a local database
db-run: db-image db-start-network db-start-master db-init db-migrate

# stop db and clean all db files that have been written to the local system 
db-local-terminate:

######################
### database utilities 
.PHONY: db-image db-start-network db-start-master db-start-worker-nodes db-data-clean
#
# todo -> move to sql deployment directory

# build docker images for database
db-image:
	cd ./deployments/sql && docker build -t db .

### Run - The whole database stack, docker and diesel-cli binarys are requred in the environment path

# create a network for the db cluster to communicate on
db-start-network:
	docker network create -d bridge $(DB_NETWORK)

# start master node
db-start-master:
	docker run -d \
	--name=$(MASTER_NODE) \
	--hostname=$(MASTER_NODE) \
	--network=$(DB_NETWORK) \
	-p 26257:26257 -p 9000:8080 \
	-v "$(PWD)/deployments/sql/data/$(MASTER_NODE):/cockroach/cockroach-data" \
	db start --insecure
	sleep 10

db-start-worker-nodes:
	$(foreach n, $(WORKER_NODES), $(call start_worker_nodes,$(n)))

define start_worker_nodes
	docker run -d \
	--name=$(1) \
	--hostname=$(1) \
	--network=$(DB_NETWORK) \
	-v "$(PWD)/deployments/sql/data/$(1):/cockroach/cockroach-data" \
	db start --insecure --join=$(MASTER_NODE)
endef

# setup our services user, create a database, and give permissions to the new users on the new
# database
db-init:
	docker exec -it $(MASTER_NODE) ./cockroach sql --insecure \
		--execute "CREATE USER IF NOT EXISTS $(DB_USER);" \
		--execute "CREATE DATABASE $(DB_NAME);" \
		--execute "GRANT ALL ON DATABASE $(DB_NAME) TO $(DB_USER);"

# run the diesel migration tool
db-migrate:
	diesel migration run

### Terminate - the whole db stack, it's configuration, and data
db-clean-master-node:
	docker stop $(MASTER_NODE)
	docker rm $(MASTER_NODE)

db-clean-worker-nodes: $(foreach n, $(WORKER_NODES), $(call clean_node $(n)))

define clean_node
	docker kill $(1) && docker rm $(1)
endef

db-stop-network:
	docker network rm $(DB_NETWORK)

db-data-clean:
	sudo rm -rf $(PWD)/deployments/sql/data

db-terminate: db-clean-master-node db-stop-network db-data-clean

### utilities

# utility to shell into master db node
db-shell:
	docker exec -it $(MASTER_NODE) ./cockroach sql --insecure

wait:
	sleep 15