# Require packages
library(shiny)
library(dplyr)
library(plotly)
library(rsconnect)

setwd("/Users/Potato/Documents/info498/Final-Project-X/app")
# Link script files
source('scripts/build_map.r')
source('scripts/plotScript.R')
source("scripts/summary-info.r")
source("scripts/map2.R")

# Read in data
df<- read.csv("data/Summarized data.csv")

shinyServer(function(input, output, session) {
  
  # Create the output for the SAT Plot data
  output$satPlot = renderPlotly({
    dotPlotOutput(df,"SAT",paste0(input$satSubject, input$satPercentile), input$satAdmSlide)
  })
  
  # Create the output the ACT Plot data
  output$actPlot = renderPlotly({
    dotPlotOutput(df,"ACT",paste0(input$actSubject, input$actPercentile), input$actAdmSlide)
  })
  
  # Data averages per state
  output$table = renderDataTable(
    data_by_state
  )
  
  # Conversion table for scores
  output$conversionTable = renderDataTable(
    score_chart
  )
  
  # output map
  output$SATmap <- renderPlotly({
    build_map(df, input$lower, input$upper)
  })
  
  # secont output map
  output$map2 <- renderPlotly({
    build_new_map(map_df, input$ADM_Score)
  })
  
})
