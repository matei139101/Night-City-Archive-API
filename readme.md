# I actually don't know what to put here
TBD...

## Docker
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


### Removing mysql database
```
sudo docker-compose down
sudo docker volume rm night-city-archive-api_mysql-data
```