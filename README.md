# Grind Time Backend

This is a small Rust API which acts as the backend for our [Grind Time App](https://github.com/AdrianColaianni/grind_time).

## Features

- Authentication
- Leader board table
- Account table

### Authentication

- Each new device will use a symmetric key to register a new account with the backend
- The device will generate a name and a secret key that the database will share
- After the account is created, the device will forget the original symmetric key
    - The device will then use the key it generated for authentication

### Leader board table

- A table of the top ten times for each location will be stored
- The table will contain a username, user id, minute duration, and location id
    - username + id avoids joins — which I am terrified of
    - The location id will be used to reference the name and location of the building

### Account table

- Will store username and id of each user that registers
- Used for authenticating client requests
- In the future will likely store other user details
