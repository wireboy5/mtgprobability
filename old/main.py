from textual.app import App, ComposeResult
from textual.widgets import Header, Footer, Static, Input, DataTable
from textual.containers import Container
import cardload

cards = cardload.load_cards()

card_dict = dict()
for card in cards:
    card_dict[card[0]] = card


class Browser(Static):

    def compose(self) -> ComposeResult:

        # Create a search box
        yield Input(placeholder="Search for a card by name and rules text")

        # And a data table of search results
        yield DataTable()

        pass
    
    def on_mount(self) -> None:
        self.query_one(Input).focus()

        table = self.query_one(DataTable)

        table.add_columns("Name", "Index", "Mana", "Text")
    
    async def on_input_changed(self, message: Input.Changed) -> None:

        
        # Search for all cards of the value
        found = cardload.search(cards, message.value)

        # Get the datatable
        table = self.query_one(DataTable)

        
        table.row_count = 0
        table._clear_caches()
        table._y_offsets.clear()
        table.data.clear()
        table.rows.clear()
        table._line_no = 0
        table._require_update_dimensions = True
        

        table.add_rows([[str(a) for a in card_dict[c[1]]] for c in found])

        table.refresh()
        




class MTGApp(App):

    BINDINGS = [
        ("a", "add_card", "Add a card to your deck"),
        ("r", "remove_card", "Remove selected card from deck")
    ]

    def compose(self) -> ComposeResult:
        yield Header()
        yield Container(Browser())
        yield Footer()

if __name__ == "__main__":
    app = MTGApp()
    app.run()