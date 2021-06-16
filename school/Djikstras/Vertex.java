/**
 *  d95wang@uw.edu, Derek Wang
	eric95@uw.edu, Eric Eckert

	CSE 373 Fall 2015
	Instructor Kevin Quinn

	Homework - Graphs
 
 * Representation of a graph vertex
 */
public class Vertex implements Comparable<Vertex>{
	
	private final String label;   // label attached to this vertex

	/**
	 * Construct a new vertex
	 * @param label the label attached to this vertex
	 */
	public Vertex(String label) {
		if(label == null)
			throw new IllegalArgumentException("null");
		this.label = label;
	}

	/**
	 * Get a vertex label
	 * @return the label attached to this vertex
	 */
	public String getLabel() {
		return label;
	}
	
	/**
	 * A string representation of this object
	 * @return the label attached to this vertex
	 */
	public String toString() {
		return label;
	}

	//auto-generated: hashes on label
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + ((label == null) ? 0 : label.hashCode());
		return result;
	}

	//auto-generated: compares labels
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		final Vertex other = (Vertex) obj;
		if (label == null) {
            return other.label == null;
		} else {
		    return label.equals(other.label);
		}
	}

	// Allows this vertex to be compared to the given vertex
	// Where if the vertices are equal if they have the same label
	// Returns 0 if they are equal, -1 if they are not
	// If the passed in vertex is null, throws an Illegal Argument Exception
	public int compareTo(Vertex vertex) {
		if (vertex == null) {
			throw new IllegalArgumentException();
		}
		if (equals(vertex)) {
			return 0;
		} else {
			return -1;
		}
	}
}
