-- Derek Wang
-- CSE 414 SPRING 2016
-- HW 3

-- Advanced SQL Queries

	-- 1. This finds the origin cities and and destinations for each of the longest
	-- flights from each origin city. This counts direct flights only with no repeated
	-- origin/destination pairs. The output is ordered by origin then destination

	 --SQL Server Execution Times:
	 --  CPU time = 1406 ms,  elapsed time = 2261 ms.

	-- Output : 333 Rows
		SELECT DISTINCT origin_city, dest_city, actual_time
		FROM FLIGHTS AS f1
		WHERE actual_time = (SELECT max(actual_time) FROM FLIGHTS AS f2 WHERE f1.origin_city = f2.origin_city)
		ORDER BY origin_city, dest_city;

	/*
		Aberdeen SD	Minneapolis MN	106
		Abilene TX	Dallas/Fort Worth TX	111
		Adak Island AK	Anchorage AK	165
		Aguadilla PR	Newark NJ	272
		Akron OH	Denver CO	224
		Albany GA	Atlanta GA	111
		Albany NY	Las Vegas NV	360
		Albuquerque NM	Baltimore MD	297
		Alexandria LA	Atlanta GA	179
		Allentown/Bethlehem/Easton PA	Atlanta GA	199
		Alpena MI	Detroit MI	80
		Amarillo TX	Houston TX	176
		Anchorage AK	Houston TX	448
		Appleton WI	Atlanta GA	180
		Arcata/Eureka CA	San Francisco CA	136
		Asheville NC	Newark NJ	189
		Ashland WV	Cincinnati OH	84
		Aspen CO	Chicago IL	183
		Atlanta GA	Honolulu HI	649
		Atlantic City NJ	Fort Lauderdale FL	212
	*/

	-- 2. Find all the origin cities that ONLY serve flights shorter than 3 hours
	-- You can assume that NULL values are not over 3 hours.
	-- Returned Cities are sorted by name and listed only once

	-- SQL Server Execution Times:
	--   CPU time = 3141 ms,  elapsed time = 3155 ms

	-- Output should be 147 rows
		SELECT DISTINCT f1.origin_city
		FROM FLIGHTS AS f1
		WHERE f1.origin_city NOT IN 
			(SELECT f2.origin_city FROM FLIGHTS AS f2 WHERE f2.actual_time >= 180)
		ORDER BY f1.origin_city;	

	/*
		Aberdeen SD
		Abilene TX
		Adak Island AK
		Albany GA
		Alexandria LA
		Alpena MI
		Amarillo TX
		Arcata/Eureka CA
		Ashland WV
		Augusta GA
		Barrow AK
		Beaumont/Port Arthur TX
		Bemidji MN
		Bethel AK
		Binghamton NY
		Bloomington/Normal IL
		Brainerd MN
		Bristol/Johnson City/Kingsport TN
		Brownsville TX
		Brunswick GA
	*/


	-- 3. For each origin city, find the percentage of departing flights shorter than 3 hours
	-- NULL actual times are considered longer than three hours now
	-- Return the City and the Percentage values and order by percentage values
	-- 0 or Null is acceptable for cities who don't have any of these flights

	--SQL Server Execution Times:
	--CPU time = 1484 ms,  elapsed time = 1493 ms.

	-- Output should be 327 rows
		SELECT f2.origin_city, (f1.short * 100.00 / f2.total) AS percentage
		FROM (SELECT origin_city, count(*) AS short FROM FLIGHTS WHERE actual_time < 180 GROUP BY origin_city) AS f1
		RIGHT JOIN (SELECT origin_city, count(*) AS total FROM FLIGHTS GROUP BY origin_city) AS f2 
			ON f2.origin_city = f1.origin_city
		ORDER BY percentage DESC;

	/*
		Aberdeen SD	100.0000000000000
		Cedar City UT	100.0000000000000
		Cordova AK	100.0000000000000
		Dickinson ND	100.0000000000000
		Dillingham AK	100.0000000000000
		Garden City KS	100.0000000000000
		Gillette WY	100.0000000000000
		Hattiesburg/Laurel MS	100.0000000000000
		Hyannis MA	100.0000000000000
		Laramie WY	100.0000000000000
		Marthas Vineyard MA	100.0000000000000
		New Bern/Morehead/Beaufort NC	100.0000000000000
		Niagara Falls NY	100.0000000000000
		Sault Ste. Marie MI	100.0000000000000
		Sioux City IA	100.0000000000000
		Victoria TX	100.0000000000000
		Waterloo IA	100.0000000000000
		West Yellowstone MT	100.0000000000000
		Wrangell AK	100.0000000000000
		Yakutat AK	100.0000000000000
	*/

	-- 4. Find all the cities where you can get to with one transfer after starting from seattle
	-- All these destinations should have an origin that is not seattle but those origin cities can be reached from seattle
	-- Does not include seattle as a destination and orders alphabetically

	--  SQL Server Execution Times:
	--   CPU time = 2328 ms,  elapsed time = 2593 ms.

	-- Output should be 256 Rows
		SELECT DISTINCT f1.dest_city
		FROM FLIGHTS f1, (SELECT DISTINCT dest_city FROM FLIGHTS WHERE origin_city = 'Seattle WA') AS f2
		WHERE f1.dest_city NOT IN (SELECT DISTINCT dest_city FROM FLIGHTS WHERE origin_city = 'Seattle WA')
		AND f1.dest_city != 'Seattle WA'
		AND f1.origin_city = f2.dest_city
		ORDER BY f1.dest_city ASC; -- This ensures that the origin of each flight is a place you can reach from seattle

	/*
	Aberdeen SD
	Abilene TX
	Adak Island AK
	Aguadilla PR
	Akron OH
	Albany GA
	Albany NY
	Alexandria LA
	Allentown/Bethlehem/Easton PA
	Alpena MI
	Amarillo TX
	Appleton WI
	Arcata/Eureka CA
	Asheville NC
	Ashland WV
	Aspen CO
	Atlantic City NJ
	Augusta GA
	Bakersfield CA
	Bangor ME
	*/

	-- 5. Find all the cities which you cannot reach from seattle and cannot reach with only one transfer from seattle.
	-- So pretty much anywhere that requires three flights starting at seattle.

	-- SQL Server Execution Times:
	--   CPU time = 2828 ms,  elapsed time = 3386 ms.

	-- Output should be 3 or 4 cities (Actually outputs 3)
		SELECT DISTINCT f1.dest_city
		FROM FLIGHTS AS f1
		WHERE f1.origin_city != 'Seattle WA'
		AND f1.dest_city != 'Seattle WA'
		AND f1.dest_city NOT IN (SELECT DISTINCT f3.dest_city 
		    FROM FLIGHTS AS f2, FLIGHTS AS f3 
		    WHERE f2.origin_city = 'Seattle WA'
		    AND f3.origin_city = f2.dest_city)
		ORDER BY f1.dest_city ASC;

	/*
		Devils Lake ND
		Hattiesburg/Laurel MS
		St. Augustine FL
	*/

	-- PHYSICAL TUNING
	/*
		i. SELECT DISTINCT carrier_id
		  FROM Flights
		  WHERE origin_city = 'Seattle WA' AND actual_time <= 180;
		  
		ii.  SELECT DISTINCT carrier_id
		  FROM Flights
		  WHERE origin_city = 'Gunnison CO' AND actual_time <= 180;
		  
		iii. SELECT DISTINCT carrier_id
		  FROM Flights
		  WHERE origin_city = 'Seattle WA' AND actual_time <= 30;

		1.a.
			The attribute to index on should be the origin_city as indexing
			would reduce the time to look for each city. The subset of specific
			cities in the origin_cities attribute is small enough that indexing
			on it would allow the query to call up origin_cities faster. This would
			allow it to quickly go through the subset of the desired city to find the
			ones that match the actual time. The alternative would be to index on the
			actual_time which would be detrimental as the times searched increased in
			interval. As you increase the range for which you search actual times,
			the less the indexing matters. On the other hand, indexing by city names
			will lead to better projected run times overall.
	*/	 
			CREATE INDEX origin_index ON FLIGHTS(origin_city);

	/*
			b.

			i. Index Not Used
				In this case, the database prefers to use the clustered index.
				The most likely reason for this is that there are approximately
				22,000 flights originating in seattle and the index on origin_cities
				does not significantly help the query for this city.

			ii. Index Used - Gunnison CO has 50 flights outgoing so the index heled.

			iii. Index Not Used
				In this case, the database prefers to use the clustered index most
				likely for the same reason seen in i. where seattle has so many 
				flights originating from it that the indexing does not help
				significantly.
				Interestingly if you index this query on the actual time, 
				you will see that index being used as the restraint of less than
				or equal to 30 for actual time creates a small enough subset
				to matter. Only about 2000 flights are in this subset and so
				indexing for time here would speed up the query.

		2. The ideal index in this case should be the origin city but as we picked
		that for the last question, the ideal should be the actual time. For this query
		in particular, the result of indexing on time would allow us to narrow down the
		results significantly as there are very few flights with actual times less than
		or equal to 30. Indexing on the destination city does make the join faster
		but it would only make the join faster. Indexing on time in this query
		would result in a very fast search as the subset contained by the <=30 restraint
		is small and locating it would be fast.
	
	*/
		CREATE INDEX time_index ON FLIGHTS(actual_time);

	/*
		3. Azure doesn't actually use the time index as the origin city index is still
		considerably better than it. As mentioned above, Gunnison CO only has 50 flights
		associated with it where it is the origin city vs the approximate 2000 that are
		narrowed down with the time index.


		4.

		Query 1
			No Indexes
				 --SQL Server Execution Times:
				 --  CPU time = 1406 ms,  elapsed time = 2261 ms.
			With Indexes
			 	SQL Server Execution Times:
   				CPU time = 1546 ms,  elapsed time = 1586 ms.
		Query 2
			No Indexes
				-- SQL Server Execution Times:
				--   CPU time = 3141 ms,  elapsed time = 3155 ms
			With Indexes
				SQL Server Execution Times:
	 			CPU time = 2375 ms,  elapsed time = 2452 ms.
		Query 3
			No Indexes
				--SQL Server Execution Times:
				--CPU time = 1484 ms,  elapsed time = 1493 ms.
			With Indexes
				 SQL Server Execution Times:
			     CPU time = 1094 ms,  elapsed time = 1086 ms.
		Query 4
			No Indexes
				--  SQL Server Execution Times:
				--   CPU time = 2328 ms,  elapsed time = 2593 ms.
			With Indexes
				SQL Server Execution Times:
   				CPU time = 2437 ms,  elapsed time = 2546 ms.
		Query 5
			No Indexes
				-- SQL Server Execution Times:
				--   CPU time = 2828 ms,  elapsed time = 3386 ms.
			With Indexes
				 SQL Server Execution Times:
  				 CPU time = 2750 ms,  elapsed time = 2857 ms.

	E. I was actually pretty pleased using this service. For the most part various controls are simple to use
	and controls are rather easy to find. I have spent a lot of time on phpmyadmin to do similar work
	but my experience on Azure was pretty smooth. The only issue I ran into was ingesting the extra credit data
	where my data types for the attributes didn't work for some reason and I had to make the integers into
	real numbers. I didn't really understand why this was an issue.

	The analysis tools were the most interesting to use and they seemed really convenient for determining
	how good your queries are. After using it I would definitely want to use it again.

	Offering a DBMS on a public cloud seems like a really good idea to me as it is incredibly accessible 
	and convenient. The only issue that I might see is internet security but if you don't have that much
	sensitive information then this could be pretty good to use. Otherwise a privately operated database
	would be much more secure and would offer most of the same functions.

	*/
