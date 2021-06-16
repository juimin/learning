-- Derek Wang 1332178
-- CSE 414 Databases Spring 2016
-- Homework 7

--Create the customers table and insert the a small set of data

CREATE TABLE Customers(
    cid int PRIMARY KEY,
    username varchar(255),
    name varchar(255),
    zip int,
    password varchar(255),
    CONSTRAINT userID UNIQUE(username)
);

INSERT INTO Customers(cid, username, name, zip, password) VALUES(1, 'd95wang', 'Derek Wang',98105,'potato');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(2, 'e95eckert',  'Eric Eckert', 98111, 'tomato');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(3, 'a95gilbert',  'Alex Gilbert', 98105, 'weaboo');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(4, 'n95pong',  'Nick Pong', 98011, 'gamer');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(5, 'm95wang',  'Morgan Wang', 98072, 'pharmacy');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(6, 'w95zhao',  'Willa Zhao', 10932, 'california');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(7, 'l95chen',  'Leo Chen', 20232,'missing');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(8, 'r95lay',  'Raymond Lay', 37232, 'therapy');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(9, 't95pham',  'Tracy Pham', 23212,'publichealth');
INSERT INTO Customers(cid, username,  name, zip, password) VALUES(10, 'd95lee',  'Davin Lee', 98111, 'helep');

-- Create the Reservations(cid, fid table containing each customer and their reserved flight
-- Insert sample data

CREATE TABLE Reservations(
    rid int IDENTITY(1,1) PRIMARY KEY,
    cid int REFERENCES Customers(cid),
    fid int REFERENCES FLIGHTS(fid),
    CONSTRAINT reservation UNIQUE(cid, fid)
);

-- CREATE AN INDEX on fid
CREATE INDEX flights ON Reservations(fid);

INSERT INTO Reservations(cid, fid) VALUES(1,2);
INSERT INTO Reservations(cid, fid) VALUES(2,3);
INSERT INTO Reservations(cid, fid) VALUES(3,4);
INSERT INTO Reservations(cid, fid) VALUES(4,5);
INSERT INTO Reservations(cid, fid) VALUES(5,2);
INSERT INTO Reservations(cid, fid) VALUES(6,3);
INSERT INTO Reservations(cid, fid) VALUES(7,2);
INSERT INTO Reservations(cid, fid) VALUES(8,1);
INSERT INTO Reservations(cid, fid) VALUES(9,34);
INSERT INTO Reservations(cid, fid) VALUES(10,23);
INSERT INTO Reservations(cid, fid) VALUES(1,23);
INSERT INTO Reservations(cid, fid) VALUES(2,34);
INSERT INTO Reservations(cid, fid) VALUES(3,34);
INSERT INTO Reservations(cid, fid) VALUES(4,34);
INSERT INTO Reservations(cid, fid) VALUES(5,34);
INSERT INTO Reservations(cid, fid) VALUES(6,33);
INSERT INTO Reservations(cid, fid) VALUES(7,55);
INSERT INTO Reservations(cid, fid) VALUES(8,11);
INSERT INTO Reservations(cid, fid) VALUES(9,22);
INSERT INTO Reservations(cid, fid) VALUES(10,33);
INSERT INTO Reservations(cid, fid) VALUES(1,67);
INSERT INTO Reservations(cid, fid) VALUES(2,87);
INSERT INTO Reservations(cid, fid) VALUES(3,67);
INSERT INTO Reservations(cid, fid) VALUES(4,103);
INSERT INTO Reservations(cid, fid) VALUES(5,190);
INSERT INTO Reservations(cid, fid) VALUES(6,199);
INSERT INTO Reservations(cid, fid) VALUES(7,15);
INSERT INTO Reservations(cid, fid) VALUES(8,177);
INSERT INTO Reservations(cid, fid) VALUES(9,156);
INSERT INTO Reservations(cid, fid) VALUES(10,153);

-- Alter the flights table to have a maximum capacity
ALTER TABLE FLIGHTS ADD max_capacity int;
UPDATE FLIGHTS SET max_capacity = 3;

ALTER TABLE FLIGHTS ADD current_capacity int;
UPDATE FLIGHTS SET current_capacity = 0;


