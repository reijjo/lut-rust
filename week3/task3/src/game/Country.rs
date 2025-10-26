#[derive(Clone)]
#[derive(PartialEq)]
pub struct Country {
	name: String,
	population: i64,
	army_size: i64,
	conquered_countries: Vec<String>,
	is_conquered: bool
}

impl Country {
	pub fn new(name: String, population: i64, army_size: i64, conquered_countries: Vec<String>, is_conquered: bool) -> Self {
		Self {
			name: name,
			population,
			army_size,
			conquered_countries,
			is_conquered
		}
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn get_population(&self) -> &i64 {
		&self.population
	}

	pub fn get_army_size(&self) -> &i64 {
		&self.army_size
	}

	pub fn get_conquered_nations(&self) -> &Vec<String> {
		&self.conquered_countries
	}

	pub fn get_is_conquered(&self) -> &bool {
		&self.is_conquered
	}

	pub fn set_population(&mut self, population: i64) {
		self.population = population
	}

	pub fn set_army_size(&mut self, army_size: i64) {
		self.army_size = army_size
	}

	pub fn set_conquered_nations(&mut self, conquered_countries: Vec<String>) {
		self.conquered_countries = conquered_countries
	}

	pub fn set_is_conquered(&mut self, is_conquered: bool) {
		self.is_conquered = is_conquered
	}
}