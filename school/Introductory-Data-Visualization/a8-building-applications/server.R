# Server for shiny application

# Apply the relevant packages
library("shiny")
library("dplyr")
library("plotly")

Species_Filter = function(name) {
  return(filter(iris, Species %in% name))
}

shinyServer(function(input, output) {

  # Create the output function for rendering the iris data
  output$iris_plot = renderPlotly({
    
    # Get the iris data for only the selected species
    iris_data = filter(iris, Species %in% input$checkGroup)
    names(iris_data) = c("Sepal Length", "Sepal Width", "Petal Length", "Petal Width", "Species")
    
    # Get the x and y axis variables
    xCol = as.numeric(input$x_selector)
    yCol = as.numeric(input$y_selector)
    
    # Get the Column names
    xtitle = colnames(iris_data)[xCol]
    ytitle = colnames(iris_data)[yCol]
    
    # Set titles of Columns
    fontSet <- list(
      family = "Courier New, monospace",
      size = 18,
      color = "#7f7f7f"
    )
    xLabel <- list(
      title = xtitle,
      titlefont = fontSet
    )
    yLabel <- list(
      title = ytitle,
      titlefont = fontSet
    )
    
    # Set the plot title
    titleText = paste("Iris Plot Showing Relationship between",xtitle,"and",ytitle)
    
    # Plot the scatter plot of the x and y variables
    plot = plot_ly(data = iris_data,
                   title = titleText,
                   x = iris_data[[xCol]],
                   y = iris_data[[yCol]], 
                   mode = "markers",
                   color = Species)
    
    # Set the layout for the plot
    layout(plot, xaxis = xLabel, yaxis = yLabel, title = titleText)
  
    })
})
  
            