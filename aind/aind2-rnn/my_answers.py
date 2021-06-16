import numpy as np

from keras.models import Sequential
from keras.layers import Dense
from keras.layers import LSTM
from keras.layers import Activation
import keras


# TODO: fill out the function below that transforms the input series 
# and window-size into a set of input/output pairs for use with our RNN model
def window_transform_series(series, window_size):
    # containers for input/output pairs
    X = []
    y = []
    
    # Loop through the series and isolating inputs with the given window size
    for start in range(np.size(series) - window_size):
        # Append inputs and outputs to the collections
        X.append(series[start:start + window_size])
        y.append(series[start + window_size])
        
    # reshape each 
    X = np.asarray(X)
    X.shape = (np.shape(X)[0:2])
    y = np.asarray(y)
    y.shape = (len(y),1)

    return X,y

# TODO: build an RNN to perform regression on our time series input/output data
def build_part1_RNN(window_size):
    # Define a sequential model
    model = Sequential()
    # Add an LSTM layer taking in the inputs from each window
    model.add(LSTM(5, input_shape = (window_size,1)))
    # Output the single unit fully connected layer
    model.add(Dense(1))
    return model


### TODO: return the text input with only ascii lowercase and the punctuation given below included.
def cleaned_text(text):
    punctuation = ['!', ',', '.', ':', ';', '?', ' ']
    output = ''
    # Loop through the text and only extract characters we want from the input
    for idx in range(len(text)):
        if text[idx].isalpha() or text[idx] in punctuation:
            output += text[idx]
        else:
            output += ''
    return output

### TODO: fill out the function below that transforms the input text and window-size into a set of input/output pairs for use with our RNN model
def window_transform_text(text, window_size, step_size):
    # containers for input/output pairs
    inputs = []
    outputs = []
    
    pairs = len(text) - window_size
    # Loop through the text and separate the inputs of window_size separated in increments of step_size
    for idx in range(0, pairs, step_size):
        inputs.append(text[idx:idx+window_size])
        outputs.append(text[idx+window_size])
    return inputs,outputs

# TODO build the required RNN model: 
# a single LSTM hidden layer with softmax activation, categorical_crossentropy loss 
def build_part2_RNN(window_size, num_chars):
    # Create a sequential NN
    model = Sequential()
    # Add the 200 unit LSTM taking in inputs with the given window size and number of characters per window
    model.add(LSTM(200, input_shape = (window_size,num_chars)))
    # Set output layer for probability of each possible character using softmax activation
    model.add(Dense(num_chars))
    model.add(Activation('softmax'))
    return model
