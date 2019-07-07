
mod roman_numerals;

fn main() {
		println!("Hello, world!");	
		for n in 0..1001 {
			
			let roman: String = roman_numerals::convert_to_roman(n);
			let result = roman_numerals::convert_to_decimal(&roman);
			
			if n % 10 == 0 {
				println!()
			}
			
		
			match result {
				Ok(stuff) => {

				if n != stuff{
					panic!("Match Failure")
				}
				
				print!("{}|{}, ", stuff, roman )
				
				},
				
				Err(err) => println!("Err {}", err ),
			}
		}
	}
