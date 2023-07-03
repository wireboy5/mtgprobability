import json
import codecs
import pandas as pd
import numpy as np
from thefuzz import fuzz
from thefuzz import process
from rich import print


def load_cards():
    # Load all of the magic cards
    with codecs.open("StandardAtomic.json", encoding="utf-8") as f:
        cards = json.loads(f.read())

    card_list = []

    # Turn them into a list of cards
    # Name - Subname Index - Mana Cost - Rules Text
    for k, v in cards["data"].items():
        for i,c in enumerate(v):
            card_list.append((k, i, c.get("manaCost", ""), c.get("text", "")))

    return card_list



# Calculate the search ratio with every card's name
def search_name(cards, search):
    # A list of (ratio, name, index)
    ratios = []

    for c in cards:
        ratios.append((fuzz.token_set_ratio(search, c[0]), c[0], c[1]))
    
    # Sort ratios by the first element
    ratios.sort()
    ratios.reverse()

    return ratios

# Calculate the search ratio with every card's text
def search_text(cards, search):
    # A list of (ratio, name, index)
    ratios = []

    for c in cards:
        ratios.append((fuzz.token_set_ratio(search, c[3]), c[0], c[1]))
    
    # Sort ratios by the first element
    ratios.sort()
    ratios.reverse()

    return ratios
def search(card_list, search):
    found = search_name(card_list, search)
    found.extend(search_text(card_list, search))

    # Sort by distance and reverse so largest distance is forst
    found.sort()
    found.reverse()

    return found