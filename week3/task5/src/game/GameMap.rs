use crate::Country;

pub struct GameMap {
	countries: Vec<Country>
}

impl GameMap {

	pub fn new() -> Self {
		let mut countries = Vec::new();

		countries.push(Country::new(String::from("Denmark"), 6000000, 50000, vec![], false));
		countries.push(Country::new(String::from("Finland"), 5600000, 900000, vec![], false));
		countries.push(Country::new(String::from("Norway"), 5500000, 100000, vec![], false));
		countries.push(Country::new(String::from("Sweden"), 10000000, 200000, vec![], false));

		Self {
			countries
		}
	}

	pub fn list_countries(&self) {
		let mut i: u8 = 1;

		for country in &self.countries {
			println!("{}) {}", i, country.get_name());
			i += 1;
		}
	}

	pub fn get_country_by_index(&mut self, index: usize) -> &mut Country {
		&mut self.countries[index]
	}

	pub fn get_countries(&self) -> &Vec<Country> {
		&self.countries
	}

	pub fn set_countries(&mut self, countries: Vec<Country>) {
		self.countries = countries
	}

	pub fn other_countries_turn(&mut self, player_country_name: &str) {
    for country in self.countries.iter_mut() {
      if !country.get_is_conquered() && country.get_name() != player_country_name {
        country.extra_army();
      }
    }
  }
}
