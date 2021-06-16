# This provides code that produces the various SAT and ACT plots
library("plotly")
library("dplyr")

dotPlotOutput = function(dataFrame, test, target,limitAdm) {
  
  # Generate vectors detailing the columns that should be selected from the data set
  nameIndex = which(colnames(dataFrame) == "INSTNM")
  rateIndex = which(colnames(dataFrame) == "ADM_RATE")
  hoverIndex = which(colnames(dataFrame) == "hover")
  # Set the plot title
  titleText = test
  
  # Narrow down the data
  testData = which(colnames(dataFrame) == target)
  selectedCol = c(nameIndex, testData, rateIndex, hoverIndex)
  PlotData = dataFrame[selectedCol]
  # Reset the column name of the test data for easier use
  colnames(PlotData)[colnames(PlotData) == target] = "testScore"
  # Cut down the plot data
  PlotData = PlotData %>% filter(!is.na(ADM_RATE)) %>% filter(testScore != "NULL") %>%
             filter(ADM_RATE > limitAdm[1], ADM_RATE < limitAdm[2])
             
  
  
  # Set the font type for labels
  fontSet <- list(
    family = "Courier New, monospace",
    size = 18,
    color = "#7f7f7f"
  )
  
  # Set the x and y axis labels
  xList = list(title = "Score", titlefont = fontSet)
  yList = list(title = "Admittance Rate", titlefont = fontSet)
  
  # Plot the data concerning the SAT selection
  plot = plot_ly(data = PlotData,
                 title = paste0(test, " Quartile Distribution"),
                 x = PlotData[[2]],
                 y = PlotData[[3]], 
                 mode = "markers",
                 text = PlotData[[4]],
                 color = ADM_RATE)
  
  # Set the title text for the plots
  titleText = paste(test, " Score vs Admittance statistics")
  
  # Apply the layout parameters
  layout(plot, xaxis = xList, yaxis = yList, title = titleText)
}