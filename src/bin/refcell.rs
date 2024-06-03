use std::cell::{Cell, RefCell};

struct Item {
    name: String,
    quantity: Cell<u32>,
    description: RefCell<String>
}

impl Item {
    fn new(name: &str, quantity: u32, description: &str) -> Item {
        Item {
            name: name.to_string(),
            quantity: Cell::new(quantity),
            description: RefCell::new(description.to_string())
        }
    }
}

struct Inventory {
    items: Vec<Item>
}

impl Inventory {

    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item)
    }

    fn update_quantity(&self, item_name: &str, new_quantity: u32) {
        self.items.iter()
            .find(|item| item.name == item_name)
            .unwrap()
            .quantity
            .set(new_quantity)
    }

    fn update_description(&self, item_name: &str, description: &str) {
        let mut current_desc = self.items.iter()
            .find(|item| item.name == item_name)
            .unwrap()
            .description
            .borrow_mut();

        *current_desc = description.to_string();
    }

    fn display(&self) {
        for item in &self.items {
            println!("Item: {}, quantity: {}, description {}",
                     item.name,
                     item.quantity.get(),
                     item.description.borrow())
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    // Add items to the inventory
    inventory.add_item(Item::new("Apple", 10, "Fresh red apples"));
    inventory.add_item(Item::new("Banana", 20, "Ripe yellow bananas"));

    // Display inventory
    inventory.display();

    // Update the quantity of an item
    inventory.update_quantity("Apple", 15);

    // Update the description of an item
    inventory.update_description("Banana", "Organic ripe yellow bananas");

    // Display inventory after updates
    inventory.display();
}