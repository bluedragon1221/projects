import re

def compress_word(word):
    # keep the first letter, then remove all vowels after it
    return re.sub(r'(.)[aeiouy]*', r'\1', word)

def compress_sentence(sentence):
    return " ".join(compress_word(i) for i in sentence.split(" "))

string = "We are trying to use this image for a particular purpose and we need to decide how much lossy compression we want to use"
print(compress_sentence(string)) 
