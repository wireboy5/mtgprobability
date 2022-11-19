import random
from rich import print

class Card:
    name: str
    count: int
    black_mana: int
    white_mana: int
    colorless_mana: int

    def __init__(self, name, black_mana, white_mana, colorless_mana):
        self.name = name
        self.black_mana = black_mana
        self.white_mana = white_mana
        self.colorless_mana = colorless_mana

class Deck:
    deck: list[Card]
    card_probs: dict[Card, float]
    cards: dict[Card, int]

    def __init__(self, cards: dict[Card, int]):
        self.cards = cards
        self.deck = []
        self.card_probs = {}

        for k, v in cards.items():
            self.deck += [k for _ in range(v)]

        self.recalc_probs()
        
    def recalc_probs(self):
        # Find the probability of drawing each card
        for card, count in self.cards.items():
           self.card_probs[card] = count / len(self.deck)

    def shuffle(self):
        return random.shuffle(self.deck)

    def draw(self):
        # Pop the last card off
        return self.deck.pop(-1)
    
    def draw_seven(self):
        out = self.deck[-7:]
        self.deck = self.deck[:-7]
        return out
    
    def mana_count(self):
        black = 0
        white = 0
        colorless = 0

        for card, count in self.cards.items():
            black += card.black_mana * count
            white += card.white_mana * count
            colorless += card.colorless_mana * count
        
        return (black, white, colorless)
    
    def calculate_lands_landless(self):
        mc = self.mana_count()
        total_manna = sum(mc)
        black_manna = mc[0]
        white_manna = mc[1]
        print(f"Black: {mc[0]} White: {mc[1]} Colorless: {mc[2]} Total: {total_manna}")
        print(f"Cards: {len(self.deck)}")


        # Calculate swamp probability
        pls = (black_manna / total_manna) * 2 # We do not care abut colorless mana so we multiply by two

        # Calculate plains probability
        plp = (white_manna / total_manna) * 2

        # Calculate the amount of land we would need for 4/7 cards to be land
        pland = (4*len(self.deck))/3

        # Find the number of swamps
        swamps = pls * pland
        plains = plp * pland

        print(f"Need {round(swamps)} Swamps and {round(plains)} Plains")



# Legendary cards
sheoldred = Card("Sheoldred, the Apocalypse", 2, 0, 2)
liesa = Card("Liesa, Forgotten Archangle", 1, 2, 2)
elas_ilkor = Card("Elas il-Kor, Sadistic Pilgrim", 1, 1, 0)

# Creatures
fell_stinger = Card("Fell Stinger", 1, 0, 2)
mindleech_ghoul = Card("Mindleech Ghoul", 1, 0, 1)
inspiring_overseer = Card("Inspiring Overseer", 0, 1, 2)
spirited_companion = Card("Spirited Companion", 0, 1, 1)
morbid_opportunist = Card("Morbid Opportunist", 1, 0, 2)

# Sorceries
duress = Card("Duress", 1, 0, 0)
pilfer = Card("Pilfer", 1, 0, 1)
soul_transfer = Card("Soul Transfer", 2, 0, 1)

# Lands
swamp = Card("Swamp", 0, 0, 0)
plains = Card("Plains", 0, 0, 0)

deck = Deck({
    sheoldred: 4,
    liesa: 4,
    elas_ilkor: 2,    

    fell_stinger: 4,
    mindleech_ghoul: 4,
    inspiring_overseer: 4,
    spirited_companion: 4,
    morbid_opportunist: 4,

    duress: 4,
    pilfer: 4,
    soul_transfer: 2,

    swamp: 21,
    plains: 17
})

#print(deck.calculate_lands_landless())
# Shuffle the deck
deck.shuffle()

draw = True
draw_count = 7
hand = []

while draw:
    draw = False
    hand = [deck.draw() ]
    for _ in range(draw_count):
        deck.recalc_probs()
        c = deck.draw()
        hand.append(c)
        probs_sec = f"{deck.card_probs[c] * 100:.2f}"
        print(f"[blue]Hand[/blue]: \t\t[red]{c.name}[/red]: {' ' * (40-len(c.name))}{probs_sec.zfill(5)}%")
        
    print(f"[orange3]Type [/orange3][grey]`m`[/grey][orange3] to mulligan[/orange3]")
    i = input("> ")
    if i == "m":
        draw = True
        draw_count -= 1
        if draw_count == 0:
            raise ValueError("You discarded too many cards!")
        deck.shuffle()






while True:
    print(f"[cornflower_blue]ENTER[/cornflower_blue][orange3] to draw[/orange3]")
    i = input("> ")
    deck.recalc_probs()
    c = deck.draw()
    probs_sec = f"{deck.card_probs[c] * 100:.2f}"
    print(f"\t\t[red]{c.name}[/red]: {' ' * (40-len(c.name))}{probs_sec.zfill(5)}%")