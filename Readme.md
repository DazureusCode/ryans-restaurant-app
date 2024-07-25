This is a simple API for handling requests for adding, getting, and deleting table orders via REST endpoints.

How to run:

- You will need the following env variables:
  - `export SECRET_KEY="Zs1MQN0YhFP6nYubKS1d57Er0jgZkZLA"`
  - `export RESTAURANT_DATABASE_URL="mysql://user:thisismypass@localhost:3306/restaurant"`
- In the CLI from the root directory of the project:
  1. You need to enter `docker-compose up -d` to run docker
  2. Then enter `cargo run --bin server` to run the server
  3. Then finally enter `cargo run --bin client` to run the client

Key Points:

- MySQL used for orders with the database URL and secret key are provided as env variables.
- Rocket is used as the web server.
- Creating an order POST call can handle n number of orders.
- The project is organized by api, domain (for business), and db which is for separation of responsibilities.
- My Db setup function drops and creates the database, but in prod I would not drop the database.

Data Structure:

- I decided to make each table a Hashmap with the key being a simple 1-100 number for easy look up and the value being a Hashmap of orders.
- Orders are objects with IDs using UUIDs, ensuring unique identification, the item name, and a cooking time randomly generated between 5-15 minutes as a string.

Future implementation ideas:

- I wrote a few business function imperatively using for loops. I didn’t have to time to figure how to this more functionally and would implement a more functional style in a V2. 
- My API errors currently return simple strings, would have liked to add more detailed erroring if there was more time.