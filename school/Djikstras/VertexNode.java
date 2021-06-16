/**
 *  d95wang@uw.edu, Derek Wang
	eric95@uw.edu, Eric Eckert

	CSE 373 Fall 2015
	Instructor Kevin Quinn

	Homework - Graphs
	
	This class is used in performing dijkstra's algorithm by containing copies
	of the vertices, their cost information, and the node before it in the shortest
	path to arrive at this node. Allows for the vertex object to immutable
 */

// Import the java utilities
import java.util.*;

public class VertexNode implements Comparable<VertexNode>{
    
    //Initializes the vertex, it's best source, and cost
	private final Vertex vertex;
    private Vertex path;
    private int cost;
    
    // Constructs a vertex node using the given vertex with an "infinite" cost
    //and a null previous location (path)
    public VertexNode(Vertex vertex) {
        this(vertex, null, Integer.MAX_VALUE);
    }
    
    // Constructs a vertex node using the given vertex with a given cost
    //and a specified previous location (path)
    public VertexNode(Vertex vertex, Vertex path, int cost) {
        if (cost < 0) {
        	throw new IllegalArgumentException();
        }
    	this.vertex = vertex;
        this.path = path;
        this.cost = cost;
    }
    
    // Returns the vertex
    public Vertex getVertex() {
    	return vertex;
    }
    
    // Returns the previous location
    public Vertex getPath() {
    	return path;
    }
    
    // Returns the cost
    public int getCost() {
    	return cost;
    }
    
    //Sets the previous location and cost of the vertex to the input values
    public void setPath(Vertex source, int cost) {
    	path = source;
    	this.cost = cost;
    }
    
    //Compares the vertex node to the input vertex node where if the
    //nodes are equal, 0 is returned, if the cost of the input node
    //is greater, -1 is returned, and if the cost of this node is
    //greater, 1 is returned.
    // If the node is null, throw an Illegal Argument Exception
    public int compareTo(VertexNode node) {
    	if (node == null) {
    		throw new IllegalArgumentException();
    	}
    	if (cost == node.cost) {
    		return 0;
    	} else if (cost < node.cost) {
    		return -1;
    	} else {
    		return 1;
    	}
    }
}