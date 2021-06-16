# Create a function that builds a map

build_map<- function(add, lower_SAT, upper_SAT){
  
  # Create data frame for Map data
  final_data<- add %>%
    filter( upper_SAT >= SAT_AVG) %>%
    filter( lower_SAT <= SAT_AVG) %>%
    select(INSTNM, LATITUDE, LONGITUDE, ADM_RATE, hover)
  
  # Parameters for markers
  m <- list(
    colorbar = list(title = "Admission Rate"),
    size = 5, opacity = 0.5, symbol = 'circle')
  
  # Parameters for map
  g <- list(
    scope = 'usa',
    projection = list(type = 'albers usa'),
    showland = TRUE,
    landcolor = toRGB("gray95"),
    subunitcolor = toRGB("gray85"),
    countrycolor = toRGB("gray85"),
    countrywidth = 0.5,
    subunitwidth = 0.5)
  
  # plotly parameters
  plot_ly(
    final_data, 
    lat = LATITUDE,
    lon = LONGITUDE, 
    text = hover, 
    color = ADM_RATE * 100, 
    opacity = 0.75, 
    na.rm = TRUE, 
    type = 'scattergeo', 
    locationmode = 'USA-states', 
    size = ADM_RATE, 
    mode = "markers", 
    marker = m) %>%
    layout(title = "US College Cards", geo = g)
}