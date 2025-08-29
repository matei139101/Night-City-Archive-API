# I actually don't know what to put here
TBD...

## Docker
Cheat sheet for Docker
### Starting the container
Go the the main directory where docker-compose.yml is located

To do any of these steps headless add the -d flag at the end

To build and run the container:
```
sudo docker-compose up --build
```

To run the container:
```
sudo docker-compose up
```

To stop the container:
```
sudo docker-compose down
```

## SeaORM
Cheat sheet for SeaORM
### Generating migrations
The official docs for writing migrations are found [here](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/)
```
sea-orm-cli migrate generate [migration name]
```

### Running migrations
The official docs for running migrations are found [here](https://www.sea-ql.org/SeaORM/docs/migration/running-migration/)
Currently **unapplied** migrations are executed on application startup.

### Generating entities from database
[Official SeaORM docs](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/)

Ensure you have a database up and running, that the migrations have been applied and that it is accessible to your host machine.
Then make sure the .env file contains the correct DATABASE_URL value for connecting.

Then run:
```
sea-orm-cli generate entity --with-serde both --seaography -l -o entity/src
```

This flag ensures entities are generated with the serde serialize and deserialize trait allowing for easy converting from and to Json.
```
--with-serde both
```

This flag ensures the entity is generated with additional flags for seaography integration.
```
--seaography
```

### Removing mysql database
```
sudo docker-compose down
sudo docker volume rm night-city-archive-api_mysql-data
```