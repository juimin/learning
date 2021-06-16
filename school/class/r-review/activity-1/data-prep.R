# Data Prep
library(dplyr)

# LOAD DATA: read in the `wa-county-data.csv` and `wa-voter-turnout.csv` files
raw.county.data <- read.csv('./data/wa-county-data.csv', stringsAsFactors = FALSE)
raw.voter.data <- read.csv('./data/wa-voter-turnout.csv', stringsAsFactors = FALSE)

# Fix encoding error
colnames(raw.county.data)[1] <- "County.name"
# DATA WRANGLING: clean and join the data frames

# County data: remove commas and percent signs from numeric columns
FormatNumbers <- function(col, data) {
  new.col <- gsub(",", "", data[,col])
  new.col <- gsub("%", "", new.col)
  return(new.col)
}

# Format the numeric columns
county.data <- data.frame(lapply(colnames(raw.county.data), FormatNumbers, data=raw.county.data), stringsAsFactors = FALSE)
colnames(county.data) <- tolower(colnames(raw.county.data))
county.data <- county.data %>% 
  mutate(county = gsub(" County", "", county.name)) %>% 
  select(-county.name) %>% 
  select(county, everything())

# Voter turnout data: remove blank rows and "total" rows
voter.data <- raw.voter.data %>% 
  filter(county != "", 
         county != "Total")

voter.data <- data.frame(lapply(colnames(voter.data), FormatNumbers, data=voter.data), stringsAsFactors = FALSE)
colnames(voter.data) <- tolower(colnames(raw.voter.data))

# Join the data frames together, set as numeric
all.data <- left_join(county.data, voter.data, by='county') 
all.data[,2:ncol(all.data)] <- sapply(all.data[,2:ncol(all.data)], as.numeric)
