import javax.xml.transform.Result;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.regex.*;
import java.util.Properties;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.io.FileInputStream;

/**
 * Runs queries against a back-end database
 */
public class Query {

	private String configFilename;
	private Properties configProps = new Properties();

	private String jSQLDriver;
	private String jSQLUrl;
	private String jSQLUser;
	private String jSQLPassword;

	// DB Connection
	private Connection conn;

	// Logged In User
	private String username;
        private int cid; // Unique customer ID

    // Store the last searched Result Set for use when booking by itinerary
    private ArrayList<Integer> lastSearchedFlights;

	// Canned queries

       // search (one hop) -- This query ignores the month and year entirely. You can change it to fix the month and year
       // to July 2015 or you can add month and year as extra, optional, arguments
	private static final String SEARCH_ONE_HOP_SQL =
			"SELECT DISTINCT TOP (?) f.*, m.month, d.day_of_week, c.name "
					+ "FROM FLIGHTS f, MONTHS m, WEEKDAYS d, CARRIERS c "
					+ "WHERE origin_city = ? AND dest_city = ? AND day_of_month = ? "
                    + "AND year = 2015 AND f.month_id = m.mid AND m.month = \'July\' "
                    + "AND actual_time > 0 AND f.carrier_id = c.cid "
                    + "AND d.did = f.day_of_week_id "
					+ "ORDER BY actual_time ASC";
	private PreparedStatement searchOneHopStatement;

    private static final String SEARCH_HOP_SQL =
            "SELECT DISTINCT TOP (?) f.*, m.month, d.day_of_week, c.name "
                    + "FROM FLIGHTS f, FLIGHTS f2, MONTHS m, WEEKDAYS d, CARRIERS c "
                    + "WHERE f.origin_city = ? "
                    + "AND f2.dest_city = ? "
                    + "AND f.day_of_month = ? "
                    + "AND f.dest_city = f2.origin_city "
                    + "AND f.year = 2015 "
                    + "AND f.actual_time > 0 AND f.carrier_id = c.cid "
                    + "AND d.did = f.day_of_week_id "
                    + "AND f.month_id = m.mid AND m.month = \'July\' "
                    + "ORDER BY f.actual_time ASC";

    private static final String SEARCH_HOP_SQL2 =
            "SELECT DISTINCT TOP (?) f2.*, m.month, d.day_of_week, c.name ,f.actual_time "
                    + "FROM FLIGHTS f, FLIGHTS f2, MONTHS m, WEEKDAYS d, CARRIERS c "
                    + "WHERE f.origin_city = ? "
                    + "AND f2.dest_city = ? "
                    + "AND f.day_of_month = ? "
                    + "AND f.dest_city = f2.origin_city "
                    + "AND f.year = 2015 "
                    + "AND f.actual_time > 0 AND f.carrier_id = c.cid "
                    + "AND d.did = f.day_of_week_id "
                    + "AND f.month_id = m.mid AND m.month = \'July\' "
                    + "ORDER BY f.actual_time ASC";

    private PreparedStatement searchHopStatement;
    private PreparedStatement searchHopStatement2;

    // TODO: Add more queries here

    // Login
    private static final String LOGIN_SQL = "SELECT * FROM Customers c WHERE c.username = ?";
    private PreparedStatement loginStatement;

    // Cancelling
    private static final String CHECK_RID = "SELECT rid FROM Reservations WHERE rid = ?";
    private static final String RESERVED =
            "SELECT rid,fid FROM Reservations WHERE rid = ? AND cid = ?";
    private static final String CANCELLER =
            "DELETE FROM Reservations WHERE rid = ? AND cid = ?";
    private PreparedStatement check_rid_Statement;
    private PreparedStatement reservedStatement;
    private PreparedStatement cancellerStatement;

    // Reservations
    private static final String GET_RESERVATIONS =
            "SELECT r.rid, f.*, m.month, d.day_of_week, c.name " +
                    "FROM FLIGHTS f, RESERVATIONS r, MONTHS m, WEEKDAYS d, CARRIERS c " +
                    "WHERE f.fid = r.fid " +
                    "AND f.carrier_id = c.cid " +
                    "AND m.mid = f.month_id " +
                    "AND d.did = f.day_of_week_id " +
                    "AND r.cid = ?";

    private PreparedStatement getReservationStatement;

    // Booking
    private static final String MAX = "SELECT max_capacity,current_capacity from FLIGHTS WHERE fid = ?";
    private PreparedStatement maxCapacityStatement;
    private static final String BOOKED = "INSERT INTO Reservations(cid, fid) VALUES(?,?)";
    private PreparedStatement bookedStatement;
    // This allows updating of the current capacity
    private static final String UPDATE_CAPACITY =
            "UPDATE FLIGHTS SET current_capacity = ? WHERE fid = ?";
    private PreparedStatement updateStatement;
    private static final String DATE_FLIGHT =
            "SELECT year, month_id, day_of_month FROM FLIGHTS f, Reservations r " +
                    "WHERE f.fid = r.fid AND r.cid = ?";
    private PreparedStatement dateFlightStatement;
    private static final String CHECK_DATE = "SELECT year, month_id, day_of_month FROM FLIGHTS f WHERE " +
            "f.fid = ?";
    private PreparedStatement checkDateStatement;

	// transactions
	private static final String BEGIN_TRANSACTION_SQL =  
			"SET TRANSACTION ISOLATION LEVEL SERIALIZABLE; BEGIN TRANSACTION;"; 
	private PreparedStatement beginTransactionStatement;

	private static final String COMMIT_SQL = "COMMIT TRANSACTION";
	private PreparedStatement commitTransactionStatement;

	private static final String ROLLBACK_SQL = "ROLLBACK TRANSACTION";
	private PreparedStatement rollbackTransactionStatement;


	public Query(String configFilename) {
		this.configFilename = configFilename;
	}

	/**********************************************************/
	/* Connection code to SQL Azure.  */
	public void openConnection() throws Exception {
		configProps.load(new FileInputStream(configFilename));
		jSQLDriver   = configProps.getProperty("flightservice.jdbc_driver");
		jSQLUrl	   = configProps.getProperty("flightservice.url");
		jSQLUser	   = configProps.getProperty("flightservice.sqlazure_username");
		jSQLPassword = configProps.getProperty("flightservice.sqlazure_password");

		/* load jdbc drivers */
		Class.forName(jSQLDriver).newInstance();

		/* open connections to the flights database */
		conn = DriverManager.getConnection(jSQLUrl, // database
				jSQLUser, // user
				jSQLPassword); // password

		conn.setAutoCommit(true); //by default automatically commit after each statement 

		/* You will also want to appropriately set the 
                   transaction's isolation level through:  
		   conn.setTransactionIsolation(...) */

	}

    // Close the connection to the database
	public void closeConnection() throws Exception {
		conn.close();
	}

	/**********************************************************/
	/* prepare all the SQL statements in this method.
      "preparing" a statement is almost like compiling it.  Note
       that the parameters (with ?) are still not filled in */

	public void prepareStatements() throws Exception {
		searchOneHopStatement = conn.prepareStatement(SEARCH_ONE_HOP_SQL);
 		beginTransactionStatement = conn.prepareStatement(BEGIN_TRANSACTION_SQL);
		commitTransactionStatement = conn.prepareStatement(COMMIT_SQL);
		rollbackTransactionStatement = conn.prepareStatement(ROLLBACK_SQL);
        loginStatement = conn.prepareStatement(LOGIN_SQL);
        check_rid_Statement = conn.prepareStatement(CHECK_RID);
        reservedStatement = conn.prepareStatement(RESERVED);
        cancellerStatement = conn.prepareStatement(CANCELLER);
        getReservationStatement = conn.prepareStatement(GET_RESERVATIONS);
        maxCapacityStatement = conn.prepareStatement(MAX);
        bookedStatement = conn.prepareStatement(BOOKED);
        updateStatement = conn.prepareStatement(UPDATE_CAPACITY);
        dateFlightStatement = conn.prepareStatement(DATE_FLIGHT);
        checkDateStatement = conn.prepareStatement(CHECK_DATE);
        searchHopStatement = conn.prepareStatement(SEARCH_HOP_SQL);
        searchHopStatement2 = conn.prepareStatement(SEARCH_HOP_SQL2);
		/* add here more prepare statements for all the other queries you need */
		/* . . . . . . */
	}

    // Login to the application
	public void transaction_login(String username, String password) throws Exception {
        // Add code here
        if (username == null || password == null) {
            throw new IllegalArgumentException();
        }
        // If the username contains spaces then the username is invalid -- Stop SQL Injections
        if (!username.contains(" ")) {
            beginTransaction();
            // Get the user from the database selecting by the input user
            loginStatement.clearParameters();
            loginStatement.setString(1, username);
            // If the user exists, then check the password and redirect if the password is invalid
            ResultSet customer = loginStatement.executeQuery();
            if (customer.next()) {
                String userPW = customer.getString("password");
                if (userPW.equals(password)) {
                    // Successful login of user
                    cid = customer.getInt("cid");
                    this.username = customer.getString("username");
                    System.out.println("Successful login of " + username);
                    commitTransaction();
                } else {
                    // Roll back the SQL if the user has entered the wrong password
                    System.out.println("You have entered an incorrect password");
                    rollbackTransaction();
                }
            } else {
                // Roll back the SQL if the user does not exist
                System.out.println("This user does not exist");
                rollbackTransaction();
            }
            customer.close();
        } else {
            // The username entered contains a space and so should not be allowed in case of injection
            System.out.println("Invalid Username");
        }
	}

	/**
	 * Searches for flights from the given origin city to the given destination
	 * city, on the given day of the month. If "directFlight" is true, it only
	 * searches for direct flights, otherwise is searches for direct flights
	 * and flights with two "hops". Only searches for up to the number of
	 * itineraries given.
	 * Prints the results found by the search.
	 */
	public void transaction_search_safe(String originCity, String destinationCity, boolean directFlight, int dayOfMonth, int numberOfItineraries) throws Exception {
        lastSearchedFlights = new ArrayList<Integer>();
        beginTransaction();
        if (directFlight) {
                // one hop itineraries
                searchOneHopStatement.clearParameters();
                searchOneHopStatement.setInt(1, numberOfItineraries);
                searchOneHopStatement.setString(2, originCity);
                searchOneHopStatement.setString(3, destinationCity);
                searchOneHopStatement.setInt(4, dayOfMonth);
                ResultSet oneHopResults = searchOneHopStatement.executeQuery();
                if (oneHopResults.next()) {
                    printResult(searchOneHopStatement.executeQuery(), false, numberOfItineraries);
                } else {
                    // No Results
                    System.out.println("The search returned no flights");
                }
                oneHopResults.close();
                commitTransaction();
            } else {
                // The user has entered in an invalid direct flight value
                searchHopStatement.clearParameters();
                searchHopStatement.setInt(1, numberOfItineraries);
                searchHopStatement.setString(2, originCity);
                searchHopStatement.setString(3, destinationCity);
                searchHopStatement.setInt(4, dayOfMonth);
                searchHopStatement2.clearParameters();
                searchHopStatement2.setInt(1, numberOfItineraries);
                searchHopStatement2.setString(2, originCity);
                searchHopStatement2.setString(3, destinationCity);
                searchHopStatement2.setInt(4, dayOfMonth);
                ResultSet HopResults = searchHopStatement.executeQuery();
                if (HopResults.next()) {
                    printResult(searchHopStatement.executeQuery(), false, numberOfItineraries);
                    System.out.println("-- TRANSFERS --");
                    printResult(searchHopStatement2.executeQuery(), true, numberOfItineraries);
                } else {
                // No Results
                    System.out.println("The search returned no flights");
                }
                HopResults.close();
                commitTransaction();
            }
	}

    private void printResult(ResultSet HopResults, boolean transfer, int size) throws Exception {
        while (HopResults.next()) {
            int result_fid = HopResults.getInt("fid");
            int result_flight_num = HopResults.getInt("flight_num");
            int result_year = HopResults.getInt("year");
            int result_day = HopResults.getInt("day_of_month");
            String result_month = HopResults.getString("month");
            String result_carrier = HopResults.getString("name");
            String result_origin_city = HopResults.getString("origin_city");
            String result_dest_city = HopResults.getString("dest_city");
            int result_dep_delay = HopResults.getInt("departure_delay");
            int result_taxi_out = HopResults.getInt("taxi_out");
            int result_arr_delay = HopResults.getInt("arrival_delay");
            int result_canceled = HopResults.getInt("canceled");
            int result_actual_time = HopResults.getInt("actual_time");
            int result_distance = HopResults.getInt("distance");
            lastSearchedFlights.add(result_fid);
            String itineraryNum;
            if (transfer) {
                itineraryNum = Integer.toString(lastSearchedFlights.size()) +
                        " Transfer for Flight " + Integer.toString(lastSearchedFlights.size() - size);
            } else {
                itineraryNum = Integer.toString(lastSearchedFlights.size());
            }
            System.out.println("Flight " + itineraryNum + ": " +
                    "Date: " + result_month + "," + result_day + "," + result_year + ", " +
                    result_carrier + ", " +
                    result_flight_num + ", " +
                    "Origin: " + result_origin_city + ", " + ", " +
                    "Destination: " + result_dest_city + ", " + ", " +
                    "Departure Delay: " + result_dep_delay + ", " +
                    "Arrival Delay: " + result_arr_delay + ", " +
                    "Taxi Out: " + result_taxi_out + ", " +
                    "Canceled: " + result_canceled + ", " +
                    "Actual Time: " + result_actual_time + ", " +
                    "Distance: " + result_distance + ", " +
                    "FLIGHT ID: " + result_fid);
        }
    }
	
	public void transaction_search_unsafe(String originCity, String destinationCity, boolean directFlight, int dayOfMonth, int numberOfItineraries) throws Exception {

            // one hop itineraries
            String unsafeSearchSQL =
                "SELECT TOP (" + numberOfItineraries +  ") year,month_id,day_of_month,carrier_id,flight_num,origin_city,actual_time "
                + "FROM Flights "
                + "WHERE origin_city = \'" + originCity + "\' AND dest_city = \'" + destinationCity +  "\' AND day_of_month =  " + dayOfMonth + " "
                + "ORDER BY actual_time ASC";

            System.out.println("Submitting query: " + unsafeSearchSQL);
            Statement searchStatement = conn.createStatement();
            ResultSet oneHopResults = searchStatement.executeQuery(unsafeSearchSQL);

            while (oneHopResults.next()) {
                int result_year = oneHopResults.getInt("year");
                int result_monthId = oneHopResults.getInt("month_id");
                int result_dayOfMonth = oneHopResults.getInt("day_of_month");
                String result_carrierId = oneHopResults.getString("carrier_id");
                String result_flightNum = oneHopResults.getString("flight_num");
                String result_originCity = oneHopResults.getString("origin_city");
                int result_time = oneHopResults.getInt("actual_time");
                System.out.println("Flight: " + result_year + "," + result_monthId + "," + result_dayOfMonth + "," + result_carrierId + "," + result_flightNum + "," + result_originCity + "," + result_time);
            }
            oneHopResults.close();
        }

	public void transaction_book(int itineraryId) throws Exception {
		// Check if the user is logged in
        if (cid != 0 && this.username != null) {
            if (lastSearchedFlights == null) {
                // There are no flights to select from
                System.out.println("Please Search Itineraries before booking");
            } else {
                // Check that the itinerary ID is within the bounds of the last searched flights
                if (lastSearchedFlights.size() >= itineraryId && itineraryId > 0) {
                    // The itinerary selection is valid and the booking can occur
                    beginTransaction();
                    int booked_fid = lastSearchedFlights.get(itineraryId - 1);
                    // Check the flight's capacity
                    maxCapacityStatement.setInt(1, booked_fid);
                    ResultSet maximum_cap = maxCapacityStatement.executeQuery();
                    maximum_cap.next();
                    int max_cap = maximum_cap.getInt("max_capacity");
                    int curr_cap = maximum_cap.getInt("current_capacity");
                    // Check the current user's flights for that day
                    boolean already_booked = double_booked(booked_fid);
                    if ((curr_cap < max_cap)) {
                        if (already_booked) {
                            // The current user is already booked for this day
                            System.out.println("You already have a flight booked on this day");
                            rollbackTransaction();
                        } else {
                            // Add the flight as a valid booked flight to reservations
                            bookedStatement.setInt(1, cid);
                            bookedStatement.setInt(2, booked_fid);
                            bookedStatement.execute();
                            // Update the flight current capacity
                            updateStatement.setInt(1, curr_cap + 1);
                            updateStatement.setInt(2, booked_fid);
                            updateStatement.execute();
                            commitTransaction();
                        }
                    } else {
                        // The flight is over capacity
                        System.out.println("Cannot reserve. Flight at capacity");
                        rollbackTransaction();
                    }
                } else {
                    // The itinerary is out of bounds
                    System.out.println("Please pick one of the Itineraries in the last searched flights");
                }
            }
        } else {
            // There is no user logged in
            System.out.println("Please log in with a valid user");
        }
	}

    // Checks if the current user already has a flight booked for the day of the given flight
    private boolean double_booked(int fid) throws Exception {
        checkDateStatement.setInt(1, fid);
        ResultSet fid_date = checkDateStatement.executeQuery();
        fid_date.next();
        int fid_year = fid_date.getInt("year");
        int fid_day = fid_date.getInt("day_of_month");
        int fid_month = fid_date.getInt("month_id");
        dateFlightStatement.setInt(1, cid);
        ResultSet dates = dateFlightStatement.executeQuery();
        while(dates.next()) {
            int year = dates.getInt("year");
            int day = dates.getInt("day_of_month");
            int month = dates.getInt("month_id");
            if ((fid_year == year) && (fid_day == day)) {
                if (fid_month == month) {
                    return true;
                }
            }
        }
        return false;
    }

	public void transaction_reservations() throws Exception {
        // Checks if there is a user logged in
        if (cid != 0 && this.username != null) {
            beginTransaction();
            // If the user exists, then get all the reservations made by that user
            getReservationStatement.setInt(1, cid);
            ResultSet reserveResult = getReservationStatement.executeQuery();
            // Write out the reservations to the console
            while(reserveResult.next()) {
                int result_rid = reserveResult.getInt("rid");
                int result_fid = reserveResult.getInt("fid");
                int result_flightnum = reserveResult.getInt("flight_num");
                int result_year = reserveResult.getInt("year");
                int result_day_m = reserveResult.getInt("day_of_month");
                String result_month = reserveResult.getString("month");
                String result_day = reserveResult.getString("day_of_week");
                String result_carrier = reserveResult.getString("name");
                String result_origin_city = reserveResult.getString("origin_city");
                String result_dest_city = reserveResult.getString("dest_city");
                int result_dep_delay = reserveResult.getInt("departure_delay");
                int result_taxi_out = reserveResult.getInt("taxi_out");
                int result_arr_delay = reserveResult.getInt("arrival_delay");
                int result_canceled = reserveResult.getInt("canceled");
                int result_actual_time = reserveResult.getInt("actual_time");
                int result_distance = reserveResult.getInt("distance");
                int result_max = reserveResult.getInt("max_capacity");
                int result_curr = reserveResult.getInt("current_capacity");
                System.out.println(
                        "Reservation: " + result_rid + ", " +
                                "Flight ID: " + result_fid + ", " +
                                "Flight Number: " + result_flightnum + ", " +
                                "Carrier: " + result_carrier + ", " +
                                "Date: " + result_month + " " + result_day_m + " " + result_year + ", " +
                                "Weekday: " + result_day + ", " +
                                "Origin: " + result_origin_city + ", " + ", " +
                                "Destination: " + result_dest_city + ", " + ", " +
                                "Departure Delay: " + result_dep_delay + ", " +
                                "Arrival Delay: " + result_arr_delay + ", " +
                                "Taxi Out: " + result_taxi_out + ", " +
                                "Canceled: " + result_canceled + ", " +
                                "Actual Time: " + result_actual_time + ", " +
                                "Distance: " + result_distance + ", " +
                                "Max Capacity: " + result_max + ", " +
                                "Current Capacity: " + result_curr
                );
            }
            reserveResult.close();
            commitTransaction();
        } else {
            // There is no user logged in
            System.out.println("Please log in with a valid user");
        }
	}

	public void transaction_cancel(int reservationId) throws Exception {
        // Check if the user is logged in
        if (cid != 0 && this.username != null) {
            // User is logged in and can cancel their reservations
            beginTransaction();
            // If the user exists, then get all the reservations made by that user
            // Removes the specified reservation for the user if this reservation exists
            check_rid_Statement.setInt(1, reservationId);
            ResultSet result_check_rid = check_rid_Statement.executeQuery();
            if (!result_check_rid.next()) {
                result_check_rid.close();
                // The rid specified was not valid
                System.out.println("The specified reservation ID was not valid");
                rollbackTransaction();
            } else {
                result_check_rid.close();
                // The reservation was found
                reservedStatement.setInt(1, reservationId);
                reservedStatement.setInt(2, cid);
                ResultSet result_check_reserved = reservedStatement.executeQuery();
                if (result_check_reserved.next()) {
                    int res_fid = result_check_reserved.getInt("fid");
                    result_check_reserved.close();
                    // Remove the reservation
                    cancellerStatement.setInt(1, reservationId);
                    cancellerStatement.setInt(2, cid);
                    cancellerStatement.execute();
                    // Get the current capacity
                    maxCapacityStatement.setInt(1, res_fid);
                    ResultSet maximum_cap = maxCapacityStatement.executeQuery();
                    maximum_cap.next();
                    int curr_cap = maximum_cap.getInt("current_capacity");
                    // Update with the new capacity
                    if (curr_cap <= 0) {
                        updateStatement.setInt(1,curr_cap);
                    } else {
                        updateStatement.setInt(1,curr_cap - 1);
                    }
                    updateStatement.setInt(2, res_fid);
                    updateStatement.execute();
                    maximum_cap.close();
                    commitTransaction();
                    System.out.println("Flight successfully canceled");
                } else {
                    // The reservation cannot be removed as this user is not associated
                    System.out.println("This flight was not reserved by this user");
                    rollbackTransaction();
                }
            }
        } else {
            // There is no user logged in
            System.out.println("Please log in with a valid user");
        }
	}

       public void beginTransaction() throws Exception {
            conn.setAutoCommit(false);
            beginTransactionStatement.executeUpdate();  
        }

       public void commitTransaction() throws Exception {
            commitTransactionStatement.executeUpdate(); 
            conn.setAutoCommit(true);
        }

       public void rollbackTransaction() throws Exception {
            rollbackTransactionStatement.executeUpdate();
            conn.setAutoCommit(true);
		}
}
