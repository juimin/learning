# This is the chart 2 file for assignment 7
# Written by Derek Wang

# This file takes in a dataset and outputs a chart concerning the passed in data

# Add relevant libraries
# library("dplyr")
# library("plotly")
# library("jsonlite")

# Set a test working directory
## setwd("C:/Users/Derek/Documents/INFO498F_HW/a7-collaborative-coding-teamhw/data/")
## xx = read.csv("intro_survey_data.csv", stringsAsFactors = FALSE)

# Function that generates a chart showing the distribution of students that have visited x number of countries
# separated by academic year

chart2 = function(dataset) {
  # Grab data relating to the year of each student, the type of computer they use, and the number of countries they have visited
  cut_down = dataset %>% select(What.is.your.current.class.standing., How.many.countries.have.you.visited.in.your.life.)
  
  # Creates the plot
  output = plot_ly(cut_down,
     x = How.many.countries.have.you.visited.in.your.life.,
     type = "box",
     color = What.is.your.current.class.standing.
  )
  
  # Set the title of the chart
  layout(
    title = "Distribution of countries traveled by academic year"
  )
  
  return(output)
}
  
  
  

