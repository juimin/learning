/**
 * d95wang@uw.edu, Derek Wang
   eric95@uw.edu, Eric Eckert
   
   CSE 373 Fall 2015
   Instructor Kevin Quinn

   Homework - Graphs
 * Representation of a directed graph edge.
 */
public class Edge implements Comparable<Edge> {
	
	//Initialize the start and destination vertices and the weight 
	private final Vertex from,to;
	private final int w;
	
	/**
	 * Construct a new edge
	 * @param from start vertex
	 * @param to end vertex
	 * @param w weight of this edge
	 */
	public Edge(Vertex from, Vertex to, int w) {
		if(from == null || to == null)
			throw new IllegalArgumentException("null");
		this.from = from;
		this.to = to;
		this.w = w;
	}

    	/**
	 * Get the source vertex
	 * @return the Vertex that is the source of the edge
	 */
	public Vertex getSource() {
		return from;
	}

    	/**
	 * Get the destination vertex
	 * @return the Vertex that is the destination of the edge
	 */
	public Vertex getDestination() {
		return to;
	}

    	/**
	 * Get the edge weight (a.k.a. cost)
	 * @return the weight of the edge
	 */
	public int getWeight() {
		return w;
	}

	/**
	 * A string representation of this object
	 * @return A string of the form <from, to, weight>
	 */
	public String toString() {
		return "<"+from+", "+to+", "+w+">";
	}

	//auto-generated: hashes on all fields
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + ((from == null) ? 0 : from.hashCode());
		result = prime * result + ((to == null) ? 0 : to.hashCode());
		result = prime * result + w;
		return result;
	}

	//auto-generated: compares all fields
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		final Edge other = (Edge) obj;
		if (from == null) {
			if (other.from != null)
				return false;
		} else if (!from.equals(other.from))
			return false;
		if (to == null) {
			if (other.to != null)
				return false;
		} else if (!to.equals(other.to))
			return false;
		return w==other.w;
	}
	
	// Compares the vertex to the given edge. If they have the
	// same hash code and are equals they should be the same
	// so 0 is returned. If not and the edge weight of the passed
	// in edge is greater, -1 is returned. If the two are not the
	// same and this edge has the greater weight, 1 is returned;
	// Throws an Illegal Argument Exception if the edge is null
	public int compareTo(Edge edge) {
		if (edge == null) {
			throw new IllegalArgumentException();
		}
		if ((hashCode() == edge.hashCode()) && equals(edge)) {
			return 0;
		} else if (edge.getWeight() > w) {
			return -1;
		} else {
			return 1;
		}
	}
}
