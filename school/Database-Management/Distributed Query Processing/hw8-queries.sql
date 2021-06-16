-- Derek Wang
-- 1332178
-- CSE 414 Databases

-- HW 8

-- 1. What is the total number of RDF rows in the data?
SELECT COUNT(*) FROM fbFacts

-- This query returns one result with the number of RDF rows contained in the data
-- This number is 563,980,447

-- 2. What is the number of distinct predicates in the data?
SELECT COUNT(*) FROM (SELECT DISTINCT predicate FROM fbFacts) pred

-- This query counts the distinct predicates in the data
-- The number of distinct predicates in fbFacts is 18,944


-- 3. In the example in the description, we showed some tuples with the subject of mid /m/0284r5q. What are all the tuples with the subject of mid /m/0284r5q? (3 pts)
SELECT * FROM fbFacts WHERE subject = '/m/0284r5q'

/* 
	subject		predicate						obj					context
	/m/0284r5q	/type/object/key				/wikipedia/en_id	9,327,603
	/m/0284r5q	/type/object/key				/wikipedia/en		Flyte_$0028chocolate_bar$0029
	/m/0284r5q	/type/object/key				/wikipedia/en_title	Flyte_$0028chocolate_bar$0029
	/m/0284r5q	/common/topic/article			/m/0284r5t	
	/m/0284r5q	/type/object/type				/common/topic	
	/m/0284r5q	/type/object/type				/food/candy_bar	
	/m/0284r5q	/type/object/type				/business/brand	
	/m/0284r5q	/type/object/type				/base/tagit/concept	
	/m/0284r5q	/food/candy_bar/manufacturer	/m/01kh5q	
	/m/0284r5q	/common/topic/notable_types		/business/brand	
	/m/0284r5q	/common/topic/notable_types		/food/candy_bar	
	/m/0284r5q	/food/candy_bar/sold_in			/m/09c7w0	
	/m/0284r5q	/common/topic/notable_for							{"types":[], "id":"/food/candy_bar", "property":"/type/object/type", "name":"Candy bar"}
	/m/0284r5q	/type/object/name				/lang/en			Flyte
	/m/0284r5q	/common/topic/image				/m/04v6jtv

*/
-- This query returns the tuples where the subject is /m/0284r5q

-- 4. How many travel destinations does Freebase have? To do this, we'll use the type /travel/travel_destination. 
	-- In particular, we want to find the number of subjects that have a /type/object/type 
	-- predicate with the object equal to /travel/travel_destination. (3 pts)
SELECT COUNT(*) FROM (SELECT * FROM fbFacts WHERE predicate = '/type/object/type' AND obj = '/travel/travel_destination') table2

-- This query returns the number 295 which indicates there are 295 travel destinations on Freebase

-- 5. Building off the previous query, what 20 travel destination have the most tourist attractions? 
	-- Return the location name and count. Use the /travel/travel_destination/tourist_attractions
	-- predicate to find the tourist attractions for each destination. 
	-- Use the /type/object/name predicate and /lang/en object to get the name of each location
	-- (the name will be the context of the tuple with predicate /type/object/name and object /lang/en). 
	-- Sort your result by the number of tourist attractions from largest to smallest and then on the 
	-- destination name alphabetically and only return the top 20. (4 pts)
SELECT context AS Destination_Name, t2.count
FROM (SELECT subject, COUNT(*) AS count 
FROM fbFacts 
WHERE predicate = '/travel/travel_destination/tourist_attractions' 
GROUP BY subject 
ORDER BY count 
DESC LIMIT 20) AS t2, fbFacts AS t1
WHERE t1.subject = t2.subject
AND t1.predicate = '/type/object/name'
AND t1.obj = '/lang/en'
ORDER BY t2.count DESC, Destination_Name

/*
	Destination_Name	count
	London				108
	Norway				74
	Finland				59
	Burlington			41
	Rome				40
	Toronto				36
	Beijing				32
	Buenos Aires		28x
	San Francisco		26
	Bangkok				20
	Munich				19
	Sierra Leone		19
	Vienna				19
	Montpelier			18
	Athens				17
	Atlanta				17
	Tanzania			17
	Berlin				16
	Laos				16
	Portland			15
*/	

-- This query returns the top 20 destinations in the Freebase database ordered from the largest to smallest attractions and then by name

-- 6. Generate a histogram of the number of distinct predicates per subject. This is more than a count of the number of distinct predicates per subject. 
	-- This is asking for computing a distribution of the number of distinct predicates. For your answer,
	-- still put the query in hw8-queries.sql, but instead of copying the result as a comment, make a chart of your results in Zeppelin 
	-- (the little icons below the query allow you to toggle output modes).
	-- Take a screenshot of a barchart of your histogram and submit it as hw8-histogram.pdf/jpg/png (6 pts)

SELECT t1.subject, COUNT(*)
FROM (SELECT DISTINCT subject, predicate FROM fbFacts) t1
GROUP BY t1.subject

-- This query gets the count of the distinct predicates per subject in the fbFacts database
-- This query should be used in Zeppelin in order to create a histogram of the data
-- The data set should be greater than 1000 values so the zeppelin will automatically truncate the data.

-- Multiple Choice Questinos

/*
1. C
2. B
3. B
4. C
5.a. FALSE
5.b. FALSE
5.c FALSE
5.d TRUE


*/