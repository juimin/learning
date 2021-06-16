# Summary Information 

# Require 'dplyr' package for data wrangling.
# library(dplyr)

# Set the working directory
# setwd('C:/Users/chenn/Documents/Info498F/a7-collaborative-coding-teamhw')

# Read the 'intro_survey_data.csv' file into a data frame
# survey_data <- read.csv('data/intro_survey_data.csv')

# Create a function that takes in a dataset as a parameter and returns a sentence that
# tells you the number of MAC users in the classroom.

mac_user <- function(dataset) {
  num_of_user <- dataset %>% 
                  filter(What.operating.system.do.you.typically.use. == 'Mac') %>% 
                  nrow()
  
  sentence <- paste('There are', num_of_user, 'Mac users in the classroom.', sep = ' ')
  return (sentence)
}


# Create a function that takes in a dataset as a parameter and returns a sentence that
# tells you the number of people who are interested in applying for informatics major. 

informatics_applicant <- function(dataset) {
  num_of_applicants <- dataset %>% 
    filter(Are.you.interested.in.applying.to.the.Informatics.major. == 'Yes') %>% 
    nrow()
  
  sentence <- paste('There are', num_of_applicants, 'people who are interested in applying for informatics major.', sep = ' ')
  return (sentence)
}

# Create a function that takes in a dataset as parameter and returns a sentence that tells
# you the number of people who have never used R programming language upon entering the class. 

never_used_r <- function(dataset) {
  num_of_people <- dataset %>% 
    filter(What.is.your.familiarity.with..Using.the.R.programming.language == 'Never used it') %>% 
    nrow()
  
  sentence <- paste('There are', num_of_people, 'people in this class who has never learned R before.', sep = ' ')
  return (sentence)
}   
  


