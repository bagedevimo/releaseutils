CREATE TABLE deploys (
	id INTEGER PRIMARY KEY NOT NULL,
	environment_name TEXT NOT NULL,
	build_number INTEGER NOT NULL,
	version_name TEXT NOT NULL
);
