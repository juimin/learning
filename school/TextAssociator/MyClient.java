import java.util.*;
import java.io.*;

/* CSE 373 
 * @Author 
 * d95wang@uw.edu, Derek Wang
 * eric95@uw.edu, Eric Eckert
 * 
 * 
 * Client code written in order to implement the TestAssociator. Two small
 * programs are included: Degrees of Nonsense, and Guess the Association.
 * Degrees of nonsense takes a random association for a user input word, then
 * if possible, plugs it back into the thesaurus to find another association.
 * The process iterates as many times as indicated by the user, or until the next
 * word is not in the thesaurus.
 * Guess the Association picks 4 random words, and picks a random association from
 * each. One of the words is printed, and then the 4 random associated words are
 * printed, then the program asks the user to guess which of the associate words
 * is related to the chosen word.
 *
 */
 
public class MyClient {
    
    public static void main (String[] args) throws IOException {
        TextAssociator associator = new TextAssociator();
        
        //Input thesaurus
        Scanner scan = new Scanner(System.in);
        System.out.println("Please input the name of the thesaurus you want to use");
        String input = scan.nextLine();
        
        File file = new File(input);
        BufferedReader reader = new BufferedReader(new FileReader(file));
        String line = null;
        while ((line = reader.readLine()) != null) {
            String[] elements = line.split(",");
            associator.addNewWord(elements[0].trim());
            for (int i = 1; i < elements.length; i++) {
                associator.addAssociation(elements[0].trim(),elements[i].trim());
            }
        }

        Random rand = new Random();
        Boolean running = true;
        
        while(running) {
            //User selection
        	System.out.println();
            System.out.println("Select-a-program (Insert a number)");
            System.out.println("1) Varying Degrees of Nonsense");
            System.out.println("2) Guess the Association");
            System.out.println("3) I want to leave");
            int selection = scan.nextInt();
            if (selection == 1) {
                degreesOfNonsense(associator, scan, rand);
            } else if (selection == 2) {
            	guessAssociation(rand, scan, associator);
            } else if (selection == 3) {
                running = false;
            } else {
                System.out.println("That's not an option.");
            }
        }
        reader.close();
    }
    
    /* Uses the passed in scanner and random number object to generate a word 
     * found in the thesaurus with random words associated to the original word.
     * This action is performed for a user specified amount of time or until the
     * randomly selected word is not in the thesaurus, or has reached the iterations
     * desired.
     */
    public static void degreesOfNonsense(TextAssociator associator, Scanner scan, Random r) {
        //Input the word and how many degrees of association
        System.out.print("Please input the word you want to use.");
        System.out.println();
        String word = scan.next();
        System.out.println("How many degrees of nonsense do you want?");
        int degrees = scan.nextInt();
        String output = "";
        System.out.println("\"" + word + "\"");
        String tempWord = word; 
        //Find associations for the number of degrees if possible
        for (int i = 0; i <= degrees; i++) {
        	//if the most recent associated word is not in the thesaurus, end
            if (!associator.contains(tempWord.trim()) || (i == degrees)) {
                output += " " + tempWord;
                i = degrees + 1;
            } 
            //otherwise choose a random association and re-input it into the thesaurus
            else {
                Set<String> associatedWords = associator.getAssociations(tempWord.toLowerCase());
                int select = r.nextInt(associatedWords.size());
                tempWord = "" + associatedWords.toArray()[select];
                System.out.println("which is associated with the word: ");
                System.out.println(tempWord);
            }			
        }
        System.out.println();
    }
    
    /*
     * Use passed in scanner and random object in order to find 4 random words in the
     * thesaurus, one of which will be the correct answer. Then list a random association
     * for each word, and the user will guess which associated word is related to the
     * chosen word.
     */
    public static void guessAssociation(Random r, Scanner scan, TextAssociator associator) {
        //Fills an array with 4 words and their associations
        WordInfo[] wordList = new WordInfo[4];
        for (int i = 0; i < 4; i++) {
            wordList[i] = associator.randomWord(r);
        }
        //Choose 1 as the correct option
        int selection = r.nextInt(4);
        
        System.out.println("Which of the following words is associated to the word \""
             + wordList[selection].getWord() + "\"?");
        System.out.println();
        
        //Choose a random association from each option and print it
        for (int i = 0; i < 4; i++) {
            Set<String> options = wordList[i].getAssociations();
            System.out.println(i + 1 + ") " + options.toArray()[r.nextInt(options.size())]);
        }
        System.out.println();
        boolean go = true;
        while(go) {
            int choice = scan.nextInt();
            //user makes a choice
            if (choice == selection + 1) {
                System.out.println("You got it right!");
                go = false;
            } else if (choice > 4) {
                System.out.println("That is not an option.");
            } else {
                System.out.println("You were wrong!");
                go = false;
            }
            System.out.println();
        }
    }   
}