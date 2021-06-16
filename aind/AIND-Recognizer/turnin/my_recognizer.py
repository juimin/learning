import warnings
from asl_data import SinglesData


def recognize(models: dict, test_set: SinglesData):
    """ Recognize test word sequences from word models set

   :param models: dict of trained models
       {'SOMEWORD': GaussianHMM model object, 'SOMEOTHERWORD': GaussianHMM model object, ...}
   :param test_set: SinglesData object
   :return: (list, list)  as probabilities, guesses
       both lists are ordered by the test set word_id
       probabilities is a list of dictionaries where each key a word and value is Log Liklihood
           [{SOMEWORD': LogLvalue, 'SOMEOTHERWORD' LogLvalue, ... },
            {SOMEWORD': LogLvalue, 'SOMEOTHERWORD' LogLvalue, ... },
            ]
       guesses is a list of the best guess words ordered by the test set word_id
           ['WORDGUESS0', 'WORDGUESS1', 'WORDGUESS2',...]
   """
    warnings.filterwarnings("ignore", category=DeprecationWarning)
    probabilities = []
    guesses = []

    # Loop through all the words in the test set
    for key, wordData in test_set.get_all_Xlengths().items():
        # Init best guess to empty and best score to lowest possible value
        best_guess = ""
        best_score = float("-inf")
        log_likelihoods = {}
        
        # Get the values for this word
        X, lengths = wordData
        # Loop through the model's items
        for w, model in models.items():
            try:
                # Get the score for this word and add it to the log likelihood list
                score = model.score(X, lengths)
                log_likelihoods[w] = score
                # See if the score is better than best
                if score > best_score:
                    # Set the new best score and word
                    best_score = score
                    best_guess = w
            except:
                # If we fail to get a likelihood for the word, set it to the lowest value
                log_likelihoods[w] = float("-inf")
       
        # Once we are done checking for the best guess
        guesses.append(best_guess)
        probabilities.append(log_likelihoods)
        
    # Return our guesses and probabilities for each word
    return probabilities, guesses
        
        
