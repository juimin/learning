import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.Random;

/* CSE 373 Starter Code
 * @Author Kevin Quinn
 * d95wang@uw.edu, Derek Wang
 * eric95@uw.edu, Eric Eckert
 * 
 * TextAssociator represents a collection of associations between words.
 * See write-up for implementation details and hints
 * 
 * Class methods written by us
 * 
 */
public class TextAssociator {
	private WordInfoSeparateChain[] table;
	private double counter;
	private int size;
	private final int[] PRIMES = {11, 23, 47, 97, 211, 421, 853, 1709, 3461, 6967, 
		14011, 28019, 56039, 112121, 222107, 442237, 880031, 1756687, 3500059, 
		7000061, 14000071, 28000073};

	/* INNER CLASS
	 * Represents a separate chain in your implementation of your hashing
	 * A WordInfoSeparateChain is a list of WordInfo objects that have all
	 * been hashed to the same index of the TextAssociator
	 */
	private class WordInfoSeparateChain {
		private List<WordInfo> chain;
		
		/* Creates an empty WordInfoSeparateChain without any WordInfo
		 */
		public WordInfoSeparateChain() {
			this.chain = new ArrayList<WordInfo>();
		}
		
		/* Adds a WordInfo object to the SeparateCahin
		 * Returns true if the WordInfo was successfully added, false otherwise
		 */
		public boolean add(WordInfo wi) {
			if (!chain.contains(wi)) {
				chain.add(wi);
				return true;
			}
			return false;
		}
		
		/* R	emoves the given WordInfo object from the separate chain
		 * Returns true if the WordInfo was successfully removed, false otherwise
		 */
		public boolean remove(WordInfo wi) {
			if (chain.contains(wi)) {
				chain.remove(wi);
				return true;
			}
			return false;
		}
		
		// Returns the size of this separate chain
		public int size() {
			return chain.size();
		}
		
		// Returns the String representation of this separate chain
		public String toString() {
			return chain.toString();
		}
		
		// Returns the list of WordInfo objects in this chain
		public List<WordInfo> getElements() {
			return chain;
		}
	}

	/* Creates a new TextAssociator without any associations 
	 */
	public TextAssociator() {
		counter = 0;
		size = 0;
		table = new WordInfoSeparateChain[PRIMES[size]];
	}

	/* Adds a word with no associations to the TextAssociator 
	 * Returns False if this word is already contained in your TextAssociator ,
	 * Returns True if this word is successfully added
	 */
	public boolean addNewWord(String word) {
		int hashIndex = getHashIndex(word);
		//check if there is anything at the referenced index.
		if (!hasList(hashIndex)) {
			//If there is nothing there, create a new WordInfoSeparateChain
			table[hashIndex] = new WordInfoSeparateChain();
		}
		//Check if the word is already in the list
		for (WordInfo item: table[hashIndex].getElements()) {
			if (item.getWord().equalsIgnoreCase(word)) {
				return false;
			}
		}
		// If the load factor reaches 0.75 then resize the hash table
		if (counter / table.length >= 0.75) {
			size++;
			resize();
			return addNewWord(word);
		} else {
			//Add word to list at referenced index
			table[hashIndex].add(new WordInfo(word));
			counter++;
			return true;
		}
	}
	
	/* Adds an association between the given words. Returns true if association correctly added, 
	 * returns false if first parameter does not already exist in the SpellChecker or if 
	 * the association between the two words already exists
	 */
	public boolean addAssociation(String word, String association) {
		int hashIndex = getHashIndex(word);
		//Check if referenced index is empty.
		if (!hasList(hashIndex)) {
			return false;
		}
		//Check if list at referenced index contains the word
		for(WordInfo item : table[hashIndex].getElements()) {
			if (item.getWord().equalsIgnoreCase(word)) {
				//If the word is there, add association
				item.addAssociation(association);
				return true;
			}
		}
		return false;
	}
	
	/* Remove the given word from the TextAssociator, returns false if word 
	 * was not contained, returns true if the word was successfully removed.
	 * Note that only a source word can be removed by this method, not an association.
	 */
	public boolean remove(String word) {
		int hashIndex = getHashIndex(word);
		//Check if referenced index is empty
		if (!hasList(hashIndex)) {
			return false;
		}
		//Checkif list at referenced index contains the word
		for (WordInfo item: table[hashIndex].getElements()) {
			if (item.getWord().equalsIgnoreCase(word)) {
				//Remove the word from the list
				table[hashIndex].remove(item);
				counter--;
				//If the load factor is <= 0.25 resize the table to be smaller
				if (counter / PRIMES[size] <= 0.25) {
					size--;
					resize();
				}
				return true;
			}
		}
		return false;
	}
	
	/* Returns a set of all the words associated with the given String  
	 * Returns null if the given String does not exist in the TextAssociator
	 */
	public Set<String> getAssociations(String word) {
		int hashIndex = getHashIndex(word);
		//Check if referenced index is empty
		if (!hasList(hashIndex)) {
			return null;
		}
		//Check if list at referenced index contains the word
		for(WordInfo item : table[hashIndex].getElements()) {
			if (item.getWord().equalsIgnoreCase(word)) {
				//Return set of associations
				return item.getAssociations();
			}
		}
		return null;
	}
	
	/* Prints the current associations between words being stored
	 * to System.out
	 */
	public void prettyPrint() {
		System.out.println("Current number of elements : " + size);
		System.out.println("Current table size: " + table.length);
		
		//Walk through every possible index in the table
		for (int i = 0; i < table.length; i++) {
			if (table[i] != null) {
				WordInfoSeparateChain bucket = table[i];
				
				//For each separate chain, grab each individual WordInfo
				for (WordInfo curr : bucket.getElements()) {
					System.out.println("\tin table index, " + i + ": " + curr);
				}
			}
		}
		System.out.println();
	}
	
	//Assignes an index to the passed in word
	private int getHashIndex(String word) {
		if (word == null) {
			throw new IllegalArgumentException();
		}
		int hash = Math.abs(word.hashCode() % PRIMES[size]);
		return hash;
	}
	
	//Resizes the hash table
	private void resize() {
		WordInfoSeparateChain[] temp = new WordInfoSeparateChain[PRIMES[size]];
		//Iterate through every index and its contained list
		for(WordInfoSeparateChain bucket : table) {
			if (bucket != null) {
				for (WordInfo wordItem: bucket.getElements()) {
					int hash = getHashIndex(wordItem.getWord());
					//Pull the word out of the previous table, assign a new
					//hash code, and place it in new table
					if (temp[hash] == null) {
						temp[hash] = new WordInfoSeparateChain();
					}
					temp[hash].add(wordItem);
				}
			}
		}
		table = temp;
	}
	
	
	//Checks if the word is contained in the thesaurus
	public boolean contains(String word) {
		int hash = getHashIndex(word);
		if (!hasList(hash)) {
			return false;
		}
		for (WordInfo object: table[hash].getElements()) {
			if (object.getWord().equalsIgnoreCase(word)) {
				return true;
			}
		}
		return false;
	}
	
	//Returns size of the table
	public int getSize() {
		return PRIMES[size];
	}
	
	//Checks if the value at the index is empty
	public boolean hasList(int n) {
		return table[n] != null;
	}
	
	//Returns a random WordInfo object in the TestAssociator
	public WordInfo randomWord(Random r) {
		int n = -1;
		//Continue to loop until a non null index is found.
        while (n == -1) {
            n = r.nextInt(PRIMES[size]); 
            if (table[n] != null) {
            	//if index is not null, pick a random WordInfo from the list
            	//at the index
            	return (WordInfo) table[n].getElements().toArray()[r.nextInt(table[n].size())];
            }
            n = -1;
        }
        return null;
       
	}  
}