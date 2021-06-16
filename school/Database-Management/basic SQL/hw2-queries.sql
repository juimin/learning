/*
	Derek Wang - 1332178
	CSE 414 Spring 16 Suciu
	HW 2

	Write the queries that will display the answers to the HW questions

List the distinct flight numbers of all flights from Seattle to Boston by Alaska Airlines Inc. on Mondays. Also notice that, in the database, the city names include the state. So Seattle appears as Seattle WA.

[Output relation cardinality: 3 rows]
Find all flights from Seattle to Boston on July 15th 2015. Search only for itineraries that have one stop. Both legs of the flight must have occurred on the same day and must be with the same carrier. The total flight time (actual_time) of the entire itinerary should be less than 7 hours (but notice that actual_time is in minutes). For each itinerary, the query should return the name of the carrier, the first flight number, the origin and destination of that first flight, the flight time, the second flight number, the origin and destination of the second flight, the second flight time, and finally the total flight time.

Put the first 20 rows of your result right after your query as a comment.

[Output relation cardinality: 488 rows]
Find the day of the week with the longest average arrival delay. Return the name of the day and the average delay.

[Output relation cardinality: 1 row]
Find the names of all airlines that ever flew more than 1000 flights in one day. Return only the names. Do not return any duplicates.

[Output relation cardinality: 11 rows]
Find all airlines that had more than 0.5 percent of their flights out of Seattle be canceled. Return the name of the airline and the percentage of canceled flight out of Seattle. Order the results by the percentage of canceled flights in ascending order.

[Output relation cardinality: 6 rows]

*/

-- Question 1
-- The quesry answers question 1, listing the flight numbers of all flights from Seattle to 
-- Boston on Monday for Alaska Airlines
-- Outputs 3 rows
SELECT DISTINCT f.flight_num
FROM FLIGHTS AS f, CARRIERS AS c, WEEKDAYS AS w
WHERE f.carrier_id = c.cid
AND f.day_of_week_id = w.did
AND f.origin_city = 'Seattle WA'
AND f.dest_city = 'Boston MA'
AND c.name = 'Alaska Airlines Inc.'
AND w.day_of_week = 'Monday';

-- Question 2
-- Displays the flight destinations and travel time for individual flights in a two flight transfer as well as
-- the carrier for the flight and total travel time
-- Outputs 488 rows unless the limit contraint is applied
SELECT c.name,f1.flight_num, f1.origin_city, f1.dest_city, f1.actual_time, f2.flight_num, f2.origin_city,
	f2.dest_city, f2.actual_time, f1.actual_time + f2.actual_time
FROM FLIGHTS AS f1, FLIGHTS AS f2, CARRIERS as c, MONTHS as m
WHERE f2.carrier_id = c.cid
AND f1.carrier_id = c.cid
AND f1.month_id = m.mid
-- Specify origin and final destination and that the transfer was at the same city
AND f1.origin_city = 'Seattle WA'
AND f2.dest_city = 'Boston MA'
AND f1.dest_city = f2.origin_city
-- Ensure the carriers are the same between transfers
AND f1.carrier_id = f2.carrier_id
-- Specify the date As July 15 2015
AND f2.year = 2015
AND f1.year = 2015
AND m.month = 'July'
AND f1.day_of_month = 15
AND f2.day_of_month = 15
-- Flight time less than 7 hours
AND f1.actual_time + f2.actual_time < 420;
-- This can be used to find the first twenty
-- LIMIT 20;

/*

The first twenty rows of the 488 results

name                    flight_num  origin_city  dest_city   actual_time  flight_num  origin_city  dest_city   actual_time  f1.actual_time + f2.actual_time
----------------------  ----------  -----------  ----------  -----------  ----------  -----------  ----------  -----------  -------------------------------
American Airlines Inc.  42          Seattle WA   Chicago IL  228          26          Chicago IL   Boston MA   150          378
American Airlines Inc.  42          Seattle WA   Chicago IL  228          186         Chicago IL   Boston MA   137          365
American Airlines Inc.  42          Seattle WA   Chicago IL  228          288         Chicago IL   Boston MA   137          365
American Airlines Inc.  42          Seattle WA   Chicago IL  228          366         Chicago IL   Boston MA   150          378
American Airlines Inc.  42          Seattle WA   Chicago IL  228          1205        Chicago IL   Boston MA   128          356
American Airlines Inc.  42          Seattle WA   Chicago IL  228          1240        Chicago IL   Boston MA   130          358
American Airlines Inc.  42          Seattle WA   Chicago IL  228          1299        Chicago IL   Boston MA   133          361
American Airlines Inc.  42          Seattle WA   Chicago IL  228          1435        Chicago IL   Boston MA   133          361
American Airlines Inc.  42          Seattle WA   Chicago IL  228          1557        Chicago IL   Boston MA   122          350
American Airlines Inc.  42          Seattle WA   Chicago IL  228          2503        Chicago IL   Boston MA   127          355
American Airlines Inc.  44          Seattle WA   New York N  322          84          New York NY  Boston MA   74           396
American Airlines Inc.  44          Seattle WA   New York N  322          199         New York NY  Boston MA   80           402
American Airlines Inc.  44          Seattle WA   New York N  322          235         New York NY  Boston MA   91           413
American Airlines Inc.  44          Seattle WA   New York N  322          1443        New York NY  Boston MA   80           402
American Airlines Inc.  44          Seattle WA   New York N  322          2118        New York NY  Boston MA                322
American Airlines Inc.  44          Seattle WA   New York N  322          2121        New York NY  Boston MA   74           396
American Airlines Inc.  44          Seattle WA   New York N  322          2122        New York NY  Boston MA   65           387
American Airlines Inc.  44          Seattle WA   New York N  322          2126        New York NY  Boston MA   60           382
American Airlines Inc.  44          Seattle WA   New York N  322          2128        New York NY  Boston MA   83           405
American Airlines Inc.  44          Seattle WA   New York N  322          2131        New York NY  Boston MA   70           392

*/

-- Question 3
-- Outputs the day of the week with the highest average arrival delay
-- Output 1 row
SELECT w.day_of_week, avg(f.arrival_delay)
FROM WEEKDAYS AS w, FLIGHTS AS f
WHERE f.day_of_week_id = w.did
GROUP BY w.day_of_week
ORDER BY avg(f.arrival_delay) DESC
LIMIT 1;

-- Question 4
-- Outputs the carrier names of the airlines who flew more than 1000 flights in one day
-- Outputs 11 rows
SELECT DISTINCT c.name
FROM CARRIERS AS c, FLIGHTS AS f
WHERE f.carrier_id = c.cid
GROUP BY c.name, f.day_of_month, f.year, f.month_id
HAVING count(*) > 1000;

-- Question 5
-- Outputs all the airlines and the percentage of flights canceled of all airlines who had flights 
-- outgoing from seattle canceled more than half of 1%
-- Outputs 6 rows
SELECT c.name, (sum(f.canceled) * 1.0) / (count(*) * 1.0)
FROM CARRIERS AS c, FLIGHTS AS f
WHERE f.carrier_id = c.cid
AND f.origin_city = 'Seattle WA'
GROUP BY c.name
HAVING (sum(f.canceled) * 1.0) / (count(*) * 1.0) > .005
ORDER BY (sum(f.canceled) * 1.0) / (count(*) * 1.0) ASC;