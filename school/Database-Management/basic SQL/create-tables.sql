/*
	Derek Wang - 1332178
	CSE 414 Spring 16 Suciu
	HW 2

	Create the flight data tables and import the data from csv
*/

PRAGMA foreign_keys = ON;
	
-- Create the carriers table
CREATE TABLE CARRIERS (
	cid varchar(10) PRIMARY KEY,
	name varchar(255)
);

-- Create the months table
CREATE TABLE MONTHS (
	mid integer PRIMARY KEY,
	month varchar(20)
); 

-- Create the weekdays table
CREATE TABLE WEEKDAYS (
	did integer PRIMARY KEY,
	day_of_week varchar(10)
);

-- Create the flights table
CREATE TABLE FLIGHTS (
	fid integer PRIMARY KEY, 
	year integer, 
	month_id integer, 
	day_of_month integer,
	day_of_week_id integer,
	carrier_id varchar(10),
	flight_num integer,
	origin_city varchar(255),
	origin_state varchar(255),
	dest_city varchar(255),
	dest_state varchar(255),
	departure_delay integer,
	taxi_out integer,
	arrival_delay integer,
	canceled integer,
	actual_time integer,
	distance integer,
	FOREIGN KEY(carrier_id) REFERENCES CARRIERS(cid),
	FOREIGN KEY(month_id) REFERENCES MONTHS(mid),
	FOREIGN KEY(day_of_week_id) REFERENCES WEEKDAYS(did)
	);

-- Import the data from csv
.mode csv
.import carriers.csv CARRIERS
.import months.csv MONTHS
.import weekdays.csv WEEKDAYS
.import flights-small.csv FLIGHTS

