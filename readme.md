# Night City Archive
Night city archive is an in-development api/search engine for Cyberpunk: Red

The current goal of this project is to have an easy and quick way to look for specific rules and items, instead of having to skim through multiple pdfs or the dissapointing fandom wiki.

## Docker
Cheat sheet for Docker
### Starting the container
Go the the main directory where docker-compose.yml is located

To do any of these steps headless add the -d flag at the end

To build and run the container:
```bash
sudo docker-compose up --build
```

To run the container:
```bash
sudo docker-compose up
```

To stop the container:
```bash
sudo docker-compose down
```

## SeaORM
Cheat sheet for SeaORM
### Generating migrations
The official docs for writing migrations are found [here](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/)
```bash

sea-orm-cli migrate generate [migration name]

```

### Running migrations
The official docs for running migrations are found [here](https://www.sea-ql.org/SeaORM/docs/migration/running-migration/)
Currently **unapplied** migrations are executed on application startup.
```rs
Migrator::up(&database_connection, None).await?;
```

### Generating entities from database
[Official SeaORM docs](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/)

Ensure you have the following:
- a database up and running 
- the migrations have been applied
- the database is accesible to the machine executing the sea-orm-cli tool
- make sure the .env file contains the correct DATABASE_URL value for connecting.

Then run:
```bash

sea-orm-cli generate entity --with-serde both --seaography -l -o entity/src

```

The --with-serde both flag ensures entities are generated with the serde serialize and deserialize trait.
The --seaography flag ensures the entity is generated with additional flags for seaography integration.


### Removing mysql database
```
sudo docker-compose down
sudo docker volume rm night-city-archive-api_mysql-data
```