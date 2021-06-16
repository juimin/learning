##install.packages("plotrix")
# library(jsonlite)
# library(plotly)
# library(dplyr)

#setwd("/Users/Edward/Documents/a7-collaborative-coding-teamhw/data")

# survey<- read.csv(file = 'intro_survey_data.csv')

map<- function(data){
  data = data$How.many.countries.have.you.visited.in.your.life.
  histogram<- hist(
              data, 
              breaks=12, col="grey65", 
              main= "Histogram from Survey Data", 
              border="blue")
  return(histogram)
}


