pub fn create_map() -> [[char; 5]; 5] {
	let mut map = [['*'; 5]; 5];

	map[2][2] = 'x';

	map
}

pub fn update_map(x: usize, y: usize) {
	let mut map = vec![vec!["*"; 5]; 5];

	map[x][y] = "x";

	for row in &map {
		println!("{}", row.join(" "));
	}
}
