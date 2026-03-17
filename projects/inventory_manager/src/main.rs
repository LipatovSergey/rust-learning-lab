#[derive(Debug)]
enum ItemType {
    Weapon,
    Armor,
    Potion,
    Misc,
}

#[derive(Debug)]
enum ItemState {
    InInventory,
    Equipped,
}

#[derive(Debug)]
struct Item {
    id: u32,
    name: String,
    item_type: ItemType,
    weight: f32,
    state: ItemState,
}

impl Item {
    fn new(id: u32, name: String, item_type: ItemType, weight: f32) -> Self {
        Self {
            id,
            name,
            item_type,
            weight,
            state: ItemState::InInventory,
        }
    }
}

struct Inventory {
    items: Vec<Item>,
    next_id: u32,
}

impl Inventory {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 0,
        }
    }

    fn add(&mut self, name: String, item_type: ItemType, weight: f32) {
        let new_item = Item::new(self.next_id, name, item_type, weight);
        self.items.push(new_item);
        self.next_id += 1;
    }

    fn remove(&mut self, id: u32) -> bool {
        let mut found_index: Option<usize> = None;
        for (i, el) in self.items.iter().enumerate() {
            if el.id == id {
                found_index = Some(i);
                break;
            }
        }

        if let Some(index) = found_index {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    fn list_items(&self) -> &[Item] {
        &self.items[..]
    }
}

fn show_inventory(items: &[Item]) {
    for item in items {
        println!("{:?}", item)
    }
}

fn main() {
    let mut inventory = Inventory::new();
    inventory.add(String::from("Arrow"), ItemType::Weapon, 0.3);
    inventory.add(String::from("Sword"), ItemType::Weapon, 1.3);
    inventory.remove(1);
    let items = inventory.list_items();
    show_inventory(items);
}
