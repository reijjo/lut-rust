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

	pub fn conquer_nation(&mut self, target: &mut Country, my_country: &str) {
		if target.get_is_conquered() {
			println!("Country already conquered!");
			return;
		}

		if target.get_name() == my_country {
			println!("Even your sick desires need boundaries.");
			return;
		}

		let my_army = self.country.get_army_size();
		let target_army = target.get_army_size();

		if my_army > target_army {
			target.set_is_conquered(true);

			let new_population = self.country.get_population() + target.get_population();
			self.country.set_population(new_population);

			let new_army_size = self.country.get_army_size() + target.get_army_size();
			self.country.set_army_size(new_army_size);

			let mut conquered = self.country.get_conquered_nations().clone();
    	conquered.push(target.get_name().to_string());
    	self.country.set_conquered_nations(conquered);

			println!("You have conquered {}", target.get_name());
		} else if my_army == target_army {
			println!("Both armies were evenly matched. No one conquered anyone.");
		} else {
			println!("You have lost your war against {}. You have been conquered.", target.get_name());
			self.country.set_is_conquered(true);
		}
	}
}