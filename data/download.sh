curl -s "https://raw.githubusercontent.com/steve-kasica/wordle-words/master/wordle.csv" |tail -n +2 | awk -F',' '{print $1}' > words.txt
