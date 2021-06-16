/**
 *  d95wang@uw.edu, Derek Wang
	eric95@uw.edu, Eric Eckert

	CSE 373 Fall 2015
	Instructor Kevin Quinn

	Homework - Graphs
 */

import java.util.List;

public class Path {
    // we use public fields fields here since this very simple class is
    // used only for returning multiple results from shortestPath
    public final List<Vertex> vertices;
    public final int cost;
    
    //Constructs a path object using the given vertices and cost
    public Path(List<Vertex> vertices, int cost) {
		this.vertices = vertices;
		this.cost = cost;
    }
}
