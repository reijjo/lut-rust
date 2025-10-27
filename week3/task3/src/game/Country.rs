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
}