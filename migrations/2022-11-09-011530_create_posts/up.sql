-- Your SQL goes here
CREATE TABLE "entries" (
	id SERIAL PRIMARY KEY,
	user_name VARCHAR NOT NULL,
	location INT NOT NULL,
	minutes INT NOT NULL
)
