--Derek Wang 1332178
--CSE 414 Databases Sp16
--HW 1 - SQLITE

--1. First, create a simple table using the following steps:
	--Create a table Edges(Source,Destination) where both Source and Destination are integers.

		create table edges(source int, destination int);
	--Insert the tuples (10,5), (6,25), (1,3), (4, 4)
		insert into edges values(10,5);
		insert into edges values(6,25);
		insert into edges values(1,3);
		insert into edges values(4,4);
	--Write a SQL statement that returns all tuples
		select * from edges;
	--Write a SQL statement that returns only column Source for all tuples
		select source from edges;
	--Write a SQL statement that returns all tuples where Source > Destination
		select * from edges where source > destination;
	--Tricky question (check the documentation here: http://www.sqlite.org/datatype3.html): Now insert the tuple ('-1','2000'). Do you get an error? Why?
		insert into edges values('-1', '2000');

		-- According to the documentation, a column with numeric affinity, which includes all 5 data types (text, numeric, integer, real and blob) can contain
		-- include data in all 5 data types. In this case, text which is entered into this column is converted to integer or real data assuming that the conversion
		-- can be completed in a lossless manner and is also reversible. Sqlite considers integer columns to be numeric columns and so this insertion
		-- does not result in an error because the two values indicated, although they are strings, can be losslessly and reversible inserted into the table;

--2. [3 points] Create a table called MyRestaurants with the following attributes: 
	--Name of the restaurant: a varchar field
	--Type of food they make: a varchar field
	--Distance (in minutes) from your house: an integer
	--Date of your last visit: a varchar field, interpreted as date
	--Whether you like it or not: an integer, interpreted as a Boolean

		create table MyRestaurants (name varchar primary key, food_type varchar, distance int, last_visit varchar(10), liked int);

--3. [3 points] Insert at least five tuples using the SQL INSERT command five (or more) times. You should insert at least one restaurant you liked, at least one restaurant you did not like,		and at least one restaurant where you leave the iLike field NULL.

		insert into MyRestaurants values('Burger King', 'American', 25, '2001-03-02', NULL);
		insert into MyRestaurants values('McDonalds', 'American', 15, '1999-09-09', 0);
		insert into MyRestaurants values('Dicks', 'American', 35, '2008-07-02', 1);
		insert into MyRestaurants values('Mee Sum', 'Chinese', 20, '2003-10-12', 1);
		insert into MyRestaurants values('Udon', 'Japanese', 5, '2011-05-07', 0);

--4. [3 points] Write a SQL query that returns all restaurants in your table. Experiment with a few of SQLite's output formats:
	--print the results in comma-separated form
	--print the results in list form, delimited by " | "
	--print the results in column form, and make each column have width 15
	--for each of the formats above, try printing/not printing the column headers with the results

		-- Print the comma separated form with header off
		.mode csv;
		.header off;
		select * from  MyRestaurants;

		-- Print the comma separated form with header on
		.header on;
		select * from  MyRestaurants;

		-- Print the list with header on
		.mode list;
		.separator "| ";
		select * from  MyRestaurants;

		-- Print the list with header off
		.header off;
		select * from  MyRestaurants;

		-- Print the columns with header off
		.mode column;
		.width 15 15 15 15 15;
		select * from  MyRestaurants;

		-- Print the columns with header on
		.header on;
		select * from  MyRestaurants;


--5. [3 points] Write a SQL query that returns only the name and distance of all restaurants within and including 20 minutes of your house. The query should list the restaurants in 
     --alphabetical order of names.

    	select name, distance from MyRestaurants where distance <= 20 order by name asc;

--6. [3 points] Write a SQL query that returns all restaurants that you like, but have not visited since more than 3 months ago.

		select * from MyRestaurants where liked = 1 and date(last_visit) < date('now', '-3 months');   






