/**
 *  d95wang@uw.edu, Derek Wang
	eric95@uw.edu, Eric Eckert

	CSE 373 Fall 2015
	Instructor Kevin Quinn

	Homework - Graphs
 */

import java.util.*;

/**
 * A representation of a graph.
 * Assumes that we do not have negative cost edges in the graph.
 */
public class MyGraph implements Graph {
    // you will need some private fields to represent the graph
    // you are also likely to want some private helper methods
    private final Map<Vertex, Set<Edge>> graph;
    /**
     * Creates a MyGraph object with the given collection of vertices
     * and the given collection of edges.
     * @param v a collection of the vertices in this graph
     * @param e a collection of the edges in this graph
     */
    public MyGraph(Collection<Vertex> v, Collection<Edge> e) {
        //Check if passed in data is legal
        if (v == null || e == null) {
            throw new IllegalArgumentException();
        }
        graph = new HashMap<Vertex, Set<Edge>>();
        // Repeat vertices are ignored
        for (Vertex vertex: v) {
            Set<Edge> temp = new HashSet<Edge>();
            graph.put(vertex, temp);
        }
        for (Edge edge: e) {
            if (edge.getWeight() < 0) {
                throw new IllegalArgumentException("Edge weight is negative");
            }
            // Edges that involve invalid vertices are ignored
            if (isValid(edge.getSource()) && isValid(edge.getDestination())) {
                Set<Edge> check = graph.get(edge.getSource());
                for (Edge knownEdge: check) {
                    if (knownEdge.getDestination().equals(edge.getDestination())) {
                        if (knownEdge.getWeight() != edge.getWeight()) {
                            throw new IllegalArgumentException("Conflicting edges");
                        }
                    }
                }
                check.add(edge);
                graph.put(edge.getSource(),check);
            }
        }
    }

    /** 
     * Return the collection of vertices of this graph
     * @return the vertices as a collection (which is anything iterable)
     */
    public Collection<Vertex> vertices() {
        return graph.keySet();
    }

    /** 
     * Return the collection of edges of this graph
     * @return the edges as a collection (which is anything iterable)
     */
    public Collection<Edge> edges() {
        Collection<Edge> output = new TreeSet<Edge>();
        for (Vertex v: graph.keySet()) {
            Set<Edge> temp = graph.get(v);
            output.addAll(temp);
        }
        return output;
    }

    /**
     * Return a collection of vertices adjacent to a given vertex v.
     *   i.e., the set of all vertices w where edges v -> w exist in the graph.
     * Return an empty collection if there are no adjacent vertices.
     * @param v one of the vertices in the graph
     * @return an iterable collection of vertices adjacent to v in the graph
     * @throws IllegalArgumentException if v does not exist.
     */
    public Collection<Vertex> adjacentVertices(Vertex v) {
        if (!vertexExists(v)) {
            throw new IllegalArgumentException();
        }
        Collection<Vertex> adjacents = new TreeSet<Vertex>();
	    for (Edge edge: graph.get(v)) {
	        adjacents.add(edge.getDestination());
	    }
    	return adjacents;
    }

    /**
     * Test whether vertex b is adjacent to vertex a (i.e. a -> b) in a directed graph.
     * Assumes that we do not have negative cost edges in the graph.
     * @param a one vertex
     * @param b another vertex
     * @return cost of edge if there is a directed edge from a to b in the graph, 
     * return -1 otherwise.
     * @throws IllegalArgumentException if a or b do not exist.
     */
    public int edgeCost(Vertex a, Vertex b) {
        if (!(vertexExists(a) && vertexExists(b))) {
    		throw new IllegalArgumentException();
    	}
        Set<Edge> temp = graph.get(a);
        for (Edge edge: temp) {
            if (edge.getDestination().equals(b)) {
                return edge.getWeight();
            }
        }
    	return -1;
    }
    

    
    // Checks the collection of vertices to see if the passed in vertex exists
    // Returns true if the vertex is known to be in the graph
    private boolean vertexExists(Vertex v) {
        if (v == null) {
            throw new IllegalArgumentException();
        }
        for (Vertex vertex: graph.keySet()) {
            if (vertex.equals(v)) {
                return true;
            }
        }
        return false;
    }
    
    //Checks if the input vertex is contained in the known vertices
    //It returns false if vertex is null or if the vertex is unknown
    //Otherwise returns true
    private boolean isValid(Vertex v) {
        if (v == null) {
            return false;
        } else {
            if (!graph.keySet().contains(v)) {
                return false;
            }
        }
        return true;
    }
    
        /**
     * Returns the shortest path from a to b in the graph, or null if there is
     * no such path.  Assumes all edge weights are nonnegative.
     * Uses Dijkstra's algorithm.
     * @param a the starting vertex
     * @param b the destination vertex
     * @return a Path where the vertices indicate the path from a to b in order
     *   and contains a (first) and b (last) and the cost is the cost of 
     *   the path. Returns null if b is not reachable from a.
     * @throws IllegalArgumentException if a or b does not exist.
     */
    public Path shortestPath(Vertex a, Vertex b) {
    	if (!isValid(a) || !isValid(b)) {
    		throw new IllegalArgumentException();
    	}
    	// If the vertices given are not valid, return null
    	// Look for the shortest path
		Map<Vertex, VertexNode> known = new HashMap<Vertex, VertexNode>();
		PriorityQueue<VertexNode> heap = makeHeap(a);
		while (!heap.isEmpty()) {
			VertexNode start = heap.poll();
			known.put(start.getVertex(), start);
			heap = replace(heap, start);
		}
		// Will be null or a path object depending on validity of path
		return makePath(known, a, b);
    }
    
    //Makes a Path object from the passed-in vertex a and vertex b using
    //a passed-in map to determine the connection between nodes
    private Path makePath(Map<Vertex, VertexNode> known, Vertex a, Vertex b) {
    	List<Vertex> output = new LinkedList<Vertex>();
    	int cost = known.get(b).getCost();
    	Vertex previous = b;
    	//Add all previous verticies to the list, O(n)
    	while (previous != null) {
    		output.add(0,previous);
    		previous = known.get(previous).getPath();
    	}
    	/* If the cost is infinite that means we didn't have a route to the
    	 * target. If the list describing the path from a to b does not
    	 * contain a, there is no valid path to the target. In either case
    	 * null is returned. The path is returned if there is a valid path
    	 * from a to b.
    	 */
    	if (!output.contains(a)) {
    		return null;
    	} else {
    		return new Path(output, cost);
    	}
    }
    
    //Takes in a vertex and constructs a priority queue for dijkstra where
    //the starting node has a cost of 0 and all the other nodes have an "infinite" cost
    //where infinity is the max integer
    private PriorityQueue<VertexNode> makeHeap(Vertex a) {
    	PriorityQueue<VertexNode> heap = new PriorityQueue<VertexNode>();
    	for (Vertex v: graph.keySet()) {
    		if (!v.equals(a)) {
    			VertexNode node = new VertexNode(v);
    			heap.add(node);
    		}
    	}
    	heap.add(new VertexNode(a, null, 0));
    	return heap;
    }
    
    /* Looks for all edges adjacent to the passed-in vertex and checks if their costs should be
     * updated according to dijkstra's algorithm
     * if so, the nodes are removed, altered, and reinserted into the priority queue and
     * the resulting priority queue is returned
     */
    private PriorityQueue<VertexNode> replace(PriorityQueue<VertexNode> heap, VertexNode a) {
		//Executes n times, look at every edge O(n)
		for (Edge edge: graph.get(a.getVertex())) {
		PriorityQueue<VertexNode> holder = new PriorityQueue<VertexNode>();
		    //rewrite the heap. look at every element in the heap O(n)
			while (!heap.isEmpty()) {
				VertexNode node = heap.poll();
				if (node.getVertex().equals(edge.getDestination())) {
					if (a.getCost() + edge.getWeight() < node.getCost()) {
						node.setPath(edge.getSource(), a.getCost() + edge.getWeight());
					}
				}
				holder.add(node);
			}
			heap = holder;
		}
		return heap;
    }
}
