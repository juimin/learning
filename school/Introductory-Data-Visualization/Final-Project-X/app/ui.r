# Require packages
library(shiny)
library(plotly)

shinyUI(navbarPage("ACT and SAT scores!",
                   
                   tabPanel("Home",
                              mainPanel(
                                includeMarkdown("scripts/index.Rmd")
                              )
                        ),
                   
                   tabPanel("Map",
                            sidebarLayout(
                              sidebarPanel(
                                
                                # Sliders for map: One for upper bound, one for lower bound
                                # IMPORTANT: Map freaks out if sliders are in certain behavior.
                                # Case 1 freakout: lower bound slider is at bottom, and upper bound slider is very close to lower bound, in other words when there are no data points in between
                                # Case 2: basically the opposite. Both upper and lower are very near the top with no data in between
                                # Case 3: when lower bound goes above upper bound
                                # Map reloads when you set the sliders reasonably. 
                                # Alternatively, use a different method of input for SAT bounds. Radio buttons or w/e
                                
                                # IMPORTANT 2: for some reason when the range of the sliders is small, the circles become larger. I don't know why that is.
                                
                                sliderInput("lower", 
                                            "Lower SAT bound", 
                                            value = 400,
                                            min = 400, 
                                            max = 1600),
                                sliderInput("upper", 
                                            "Upper SAT bound", 
                                            value = 1600,
                                            min = 400, 
                                            max = 1600),
                                
                                selectInput("ADM_Score", label = h3("State Averages"), 
                                            choices = list("Average SAT Score" = "avg_SAT", "Average Admission Rate" = "avg_acc"), 
                                            selected = 'avg_SAT')
                              ),
                              mainPanel(
                                plotlyOutput("SATmap"),
                                hr(),
                                plotlyOutput("map2")
                              )
                            )
                   ),
                   tabPanel("ACT and SAT Plots",
                            # Apply the title panel
                            titlePanel("SAT and ACT scores"),
                            
                            sidebarLayout(
                              sidebarPanel(
                                # Createa a check group that shows which species should be shown on the display
                                selectInput("actPercentile", label = h3("Select ACT Percentile"), 
                                                   choices = list("25th Percentile" = "25", "Mid Percentile" = "MID", "75th Percentile" = "75"),
                                                   selected = "MID"
                                ),
                                
                                # Select the variable that will be used on the x axis 
                                selectInput("actSubject", label = h3("Select ACT Subject"), 
                                            choices = list("English" = "ACTEN", "Writing" = "ACTWR",
                                                           "Math" = "ACTMT", "Cumulative" = "ACTCM"), 
                                            selected = "ACTEN"
                                ),
                                
                                # Input a slider to limit the range of act scores and admission rate
                                sliderInput("actAdmSlide", label = h3("ACT Admission Bound"), min = 0, max = 1,
                                            value = c(0,1)),
                                
                                # Input line break
                                hr(),
                                
                                selectInput("satPercentile", label = h3("Select SAT Percentile"), 
                                                   choices = list("25th Percentile" = "25", "Mid Percentile" = "MID", "75th Percentile" = "75"),
                                                   selected = "MID"
                                ),
                                
                                # Select the variable that will be used on the y axis 
                                selectInput("satSubject", label = h3("Select SAT Subject"), 
                                            choices = list("Reading" = "SATVR", "Writing" = "SATWR",
                                                           "Math" = "SATMT"), 
                                            selected = "SATVR"),
                                
                                # Input a slider to limit the range of act scores and admission rates
                                sliderInput("satAdmSlide", label = h3("SAT Admission Bound"), min = 0, max = 1,
                                            value = c(0,1)),
                                
                                # Input line break
                                hr()
                              ),
                              mainPanel(
                                plotlyOutput("actPlot"),
                                hr(),
                                plotlyOutput("satPlot")
                              )
                            )
                   ),
                   # FAQ panel, includes a markdown file
                   tabPanel("FAQ",
                              mainPanel(
                                includeMarkdown("scripts/FAQ.Rmd")
                              )
                            
                   ),
                   # Reference tab, includes a markdown file followed by a data table output
                   tabPanel("Reference",
                              mainPanel(
                                includeMarkdown("scripts/reference.Rmd"),
                                dataTableOutput("conversionTable")
                              )
                            
                   ),
                   # State tab, includes data table
                   tabPanel("State Data",
                            mainPanel(
                              print(h3("Averages by State: ")),
                              dataTableOutput("table")
                            )
                            ),
                   # Contact us tab, includes 
                   tabPanel("Contact Us",
                            mainPanel(
                              print(h3("Email: ")),
                              includeMarkdown("scripts/contact.Rmd")
                            )
                   )
))
