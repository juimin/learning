# This file is for data analysis

# Extract data from the csv file.
newdata <- read.csv('data/Summarized data.csv')
score_chart <- read.csv('data/SAT score conversion chart.csv')

# Convert factor variables to numerical variables

newdata$ADM_RATE <- as.numeric(as.character(newdata$ADM_RATE))
newdata$SAT_AVG <- as.numeric(as.character(newdata$SAT_AVG))
newdata$COSTT4_A <- as.numeric(as.character(newdata$COSTT4_A))

# Create a variable that stores the average admission rate for all the universities  
Ave_rate <- mean(newdata$ADM_RATE, na.rm = TRUE) %>% 
            round(digit = 3)


# Create a variable that stores the average cost for attendance for all the universities
Ave_cost <- mean(newdata$COSTT4_A, na.rm = TRUE) %>% 
            round(digit = 2)

# Create a variable that stores the average SAT score for all the universities 
Ave_sat <- mean(newdata$SAT_AVG, na.rm = TRUE) %>% 
           round(digit = 0)

# Create a summary table that shows the average cost of attendance, and admission rate by state
data_by_state <-  newdata %>% 
                    group_by(STABBR) %>% 
                    summarise(avg_SAT = mean(SAT_AVG, na.rm = TRUE) %>% round(digit = 0),
                              avg_att = mean(COSTT4_A, na.rm = TRUE) %>% round(digit = 2),
                              avg_acc = paste0(mean(ADM_RATE, na.rm = TRUE) %>% round(digit = 3) * 100, '%')) %>% 
                    na.omit()

# create a different data frome to be use for map
map_df <- data_by_state

names <- c("State", "Average SAT Score", "Average Cost of Attendance", "Average Acceptance Rate")

# change column names
names(data_by_state) <- names

# Create a graph

                  






