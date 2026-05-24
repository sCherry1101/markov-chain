import random
import re
from collections import defaultdict

class markov:
    def __init__(self):
        self.chain = defaultdict(list)
        self.start_words = []

    def clean_text(self, text):
        text = text.replace("\n", " ")
        text = re.sub(r'\s+', ' ', text)
        return text

    def train(self, text):
        text = self.clean_text(text)
        words = text.split()

        for i in range(len(words) - 3):
            w1, w2, w3, w4 = words[i], words[i+1], words[i+2], words[i+3]
            self.chain[(w1, w2, w3)].append(w4)

            if w1.endswith(".") and w2[0].isupper():
                self.start_words.append((w1, w2, w3))

    def generate(self, length=80):
        if self.start_words and random.random() > 0.2:
            w1, w2, w3 = random.choice(self.start_words)
        else:
            w1, w2, w3 = random.choice(list(self.chain.keys()))

        result = [w1, w2, w3]

        for _ in range(length):
            key = (w1, w2, w3)

            if key not in self.chain:
                break

            choices = self.chain[key]
            next_word = random.choice(choices + random.sample(choices, len(choices)))

            result.append(next_word)

            if next_word.endswith("."):
                break

            w1, w2, w3 = w2, w3, next_word

        return " ".join(result)


with open("ti.txt", "r", encoding="utf-8") as f:
    text = f.read()

ai = markov()
ai.train(text)

print(ai.generate(80))