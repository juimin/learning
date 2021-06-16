source("scripts/summary-info.r")

build_new_map <- function(map_df, column) {
  # Remove percentage from acceptance rate
  map_df$avg_acc <- as.numeric(sub("%", "", map_df$avg_acc))
  
  # give state boundaries a white border
  l <- list(color = toRGB("white"), width = 2)
  
  # specify some map projection/options
  g <- list(
    scope = 'usa',
    projection = list(type = 'albers usa'),
    showlakes = TRUE,
    lakecolor = toRGB('white')
  )
  
  plot_ly(df, z = eval(parse(text = paste0("map_df$",column))), # text = hover,
          locations = map_df$STABBR, 
          type = 'choropleth',
          locationmode = 'USA-states', 
          color = eval(parse(text = paste0("map_df$",column))), 
          colors = 'Purples',
          marker = list(line = l), 
          colorbar = list(title = "Score/percentage")) %>%
    layout(title = 'US College Statistics', geo = g) %>% 
    return()
}