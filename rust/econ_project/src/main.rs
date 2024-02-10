use rand::Rng;

#[derive(Debug)]
#[allow(dead_code)]
enum IndustryName {
    Mining,
    Agriculture,
    Service,
    Manufacturing,
    Research,
    Energy,
}

#[derive(Debug, Clone)]
struct Inventory {
    money: i32,
    labor: i8,
    energy: i8,
    materials: i8,
    goods: i8,
}

impl Inventory {
    fn new_empty() -> Inventory {
        Inventory {
            money: 0,
            labor: 0,
            energy: 0,
            materials: 0,
            goods: 0,
        }
    }
    fn add(&mut self, second: Inventory) {
        self.money += second.money;
        self.labor += second.labor;
        self.energy += second.energy;
        self.materials += second.materials;
        self.goods += second.goods;
    }

    fn subtract(&mut self, second: Inventory) {
        self.money -= second.money;
        self.labor -= second.labor;
        self.energy -= second.energy;
        self.materials -= second.materials;
        self.goods -= second.goods;
    }
}

#[derive(Debug)]
struct Industry {
    name: IndustryName,
    build_cost: Inventory,
    turn_cost: Inventory,
    turn_income: Inventory,
}

impl Industry {
    fn new_mining() -> Industry {
        Industry {
            name: IndustryName::Mining,
            build_cost: Inventory {
                money: 8_000,
                labor: 2,
                energy: 0,
                materials: 0,
                goods: 0,
            },
            turn_cost: Inventory::new_empty(),
            turn_income: Inventory {
                money: 1_600,
                labor: 0,
                energy: 0,
                materials: 2,
                goods: 0,
            },
        }
    }
    fn new_agriculture() -> Industry {
        Industry {
            name: IndustryName::Agriculture,
            build_cost: Inventory {
                money: 11_000,
                labor: 3,
                energy: 0,
                materials: 0,
                goods: 0,
            },
            turn_cost: Inventory::new_empty(),
            turn_income: Inventory {
                money: 2_100,
                labor: 0,
                energy: 0,
                materials: 3,
                goods: 0,
            },
        }
    }
}

#[derive(Debug)]
struct Player {
    inventory: Inventory,
    industries: Vec<Industry>,
}

fn d10() -> i8 {
    rand::thread_rng().gen_range(1..=10)
}

impl Player {
    fn new() -> Player {
        Player {
            inventory: Inventory {
                money: d10() as i32 * 2_000,
                labor: d10(),
                energy: d10(),
                materials: d10(),
                goods: 0,
            },
            industries: Vec::new(),
        }
    }

    fn add_industry(&mut self, industry: Industry) {
        self.inventory.subtract(industry.build_cost.clone());
        self.industries.push(industry);
    }

    fn turn(&mut self) {
        for industry in &self.industries {
            self.inventory.subtract(industry.turn_cost.clone());
            self.inventory.add(industry.turn_income.clone());
        }
    }
}

fn main() {
    let mut collin = Player::new();
    collin.add_industry(Industry::new_mining());
    println!("{:#?}", collin);
    collin.turn();
    println!("{:#?}", collin);
}
