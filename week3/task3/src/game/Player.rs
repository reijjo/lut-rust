use crate::Country;

pub struct Player {
	country: Country
}

impl Player {
	pub fn new(country: Country) -> Self {
		Self { country }
	}

	pub fn inspect(&self) {
		println!("An inspection has been completed..");
		println!("Country information:");
		println!("Name: {}", self.country.get_name());
		println!("Population: {}", self.country.get_population());
		println!("Army size: {}", self.country.get_army_size());
	}

	pub fn get_country(&mut self) -> &mut Country {
		&mut self.country
	}

	pub fn spy(&self, country: &Country) {
		if country == &self.country {
			println!("You can't spy on your own nation!");
		} else {
			println!("Espionage successful.");
			println!("Country information:");
			println!("Name: {}", country.get_name());
			println!("Population: {}", country.get_population());
			println!("Army size: {}", country.get_army_size());
		}
	}

	pub fn conquer_nation(&mut self, target: Country, my_country: &Country) {

	}
}