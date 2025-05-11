# ngrams

recursively obtains n-gram information for a list of files and outputs the ngram and their count space-separated to stdout descending by count.

Example usage:
```bash
# get all bigrams from all files in ./src
ngrams ./src

# get all symbol only bigrams
ngrams ./src -m symbols

# get all trigrams from all files in ./src and ./api
ngrams ./src ./api -n 3

# get all trigrams and output to ./output.txt
ngrams ./src ./api -n 3 >> output.txt

# get all trigrams and page through the results
ngrams ./src ./api -n 3 | less
```