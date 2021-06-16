# UI for shiny application

# Apply the relevant packages
library("shiny")
library("dplyr")

shinyUI(fluidPage(
  # Apply the title panel
  titlePanel("Iris Data Shiny Project"),
  
  # Implement the side bar and side panel
  sidebarLayout(
    
    # Position the side bar to be on the left
    position = "left",
    
    # Create the side bar panel with widgets
    sidebarPanel(
      # Createa a check group that shows which species should be shown on the display
      checkboxGroupInput("checkGroup", label = h3("Species"), 
                         choices = list("Setosa" = "setosa", "Versicolor" = "versicolor", "Virginica" = "virginica"),
                         selected = "setosa"
      ),
      
      # Input line break
      hr(),
      
      # Select the variable that will be used on the x axis 
      selectInput("x_selector", label = h3("Select X Axis Variable"), 
                  choices = list("Sepal Length" = 1, "Sepal Width" = 2,
                                 "Petal Length" = 3, "Petal Width" = 4), 
                  selected = 1),
      
      # Select the variable that will be used on the y axis 
      selectInput("y_selector", label = h3("Select Y Axis Variable"), 
                  choices = list("Sepal Length" = 1, "Sepal Width" = 2,
                                 "Petal Length" = 3, "Petal Width" = 4), 
                  selected = 1),
      
      # Input line break
      hr()
    ),
    
    # Show a plot of the generated distribution
    mainPanel(
      plotlyOutput("iris_plot")
    )
  )
  
))