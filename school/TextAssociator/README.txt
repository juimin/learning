d95wang@uw.edu, Derek Wang
eric95@uw.edu, Eric Eckert

CSE 373 Fall 2015
Instructor Kevin Quinn

Homework - Text Associator Readme.txt
This is the write up section of the homework

1)
   - Design Decision 1: What should be the starting capacity of your Text Associator?
   		If we consider that the text associator must be capable of storing an arbitrary number of words
   		and associations (so an arbitrary number of WordInfo objects), then the storage size of our
   		hash table should be able to account for a massive number of words as well as small
  		amount of words. We knew that in order to avoid having too many words become indexed to the same
  		value in our hash table, we would need to get a hash code for each word and then mod
  		that hash code by a prime table size to get the desired index. The table could be resized later
  		and modding by the table size ensures every word could be contained within the table so 
  		we decided we could start with a smaller capacity for our Text Associator. Eventually we decided
  		on 11 as it is both prime and is reasonably small.

   - Design Decision 2: At what load factor should you expand your internal capacity?
        In class we discussed that a good load factor to resize at is 0.75, which indicates that the
        hash table is about 3/4 full. Assuming that our hashing is working correctly and that
        our decision to index by modding a word's hash function by the table size (which should be
        a prime number), then each WordInfoSeparate chain contained in the hash table should only contain
        a couple WordInfo objects at max by the time the load factor reaches 0.75. In general, a load factor
        of 0.75 should offer a good balance between space overhead costs and look up times.

   - Design Decision 3: What should the new size of your array be?
   		The new size of the array should be around twice the size of the old one. However, the table size
   		must remain a prime number, so a number that approximates twice the size of the current table
   		and is prime must be selected. As there isn't really a good way to find prime numbers, and even if there
   		were, such an operation would become exceedingly expensive, we have included an array of
   		prime numbers with each successive number being approximately double the one before it.

2)  We ended up using the String's hashcode function in order to save time. Our design process for this 
	assignment was to just write the entire thing using the inbuilt function to save time as if we wanted to
	we could go back and write our own hash function and refactor it into the program if we wished to.
	This hash function was effective because it saved us the trouble of having to write our own function
	and also completed the desired effect. In the end we modded whatever integer appeared from the hashcode
	so as long as the hash function output a decently unique hash code for each string it was fine.

	There probably are a ton of different hash functions that we could have used. We even spent some time
	looking into "best hash functions" to see the types of things people had written and we also considered
	writing our own but for the purposes of this assignment there didn't really seem to be a need to go that
	far if the String object in java contains a hashcode method. Closer analysis might yield that one
	function may be better than others but this didn't seem to be that big of a concern for this project.

3)  If we had to choose a different method with which to implement our hash table, we would probably choose
	to use the method of probing with double hashing so as to avoid clustering. What we would need in order to
	accomplish this would be two hash functions, and assuming that both are well made, the hash codes for the 
	same string from each one will be different. This method would involve changing the way we find the desired index for each word by having us use both functions (the lecture slides recommend a good way as being
	(hash1(key) + i*hash2(key) % table size) where i is the number of times we have probed. Even if we
	have changed hash methods, it would seem that it would still be a good idea to use an array as the
	primary data structure, so the changes we would need to make to our code are as follows.

	Implement a hash code function that should return values different from the hash code function currently
	in use.

	Use the (hash1(key) + i*hash2(key) % table size) equation for probing for the proper place to put our
	object to be inserted. This would replace our current hashcode % table size. The add method,
	remove method, and get/set associations methods would need to include a loop that allows us to keep probing until we find an empty spot. (so anythinything that requires searching for a specific WordInfo object).

	Change the resize so that the resize would occur when the load factor reaches about 0.5 (as this is
	open addressing now). The size would still change by the same amount (approximately double and still prime).

	We wouldn't need the WordInfoSeparateChain class anymore as we are storing each WordInfo object in one index.


4) We spent about 10 hours on this. The most challenging part of the project was actually the client writing
   portion seen in part three of the assignment. The rest of the assignment seemed rather straightforward going
   with what we learned in class about hash tables. Second to that would be thinking about whether or not we
   wanted to implement our own hash code or use the java inbuilt one. This was more thinking than anything else.  		