struct RomanDigit {

	decimal: i32,
	symbol: char,
}

impl RomanDigit {
	const ROMAN_DIGITS: [RomanDigit;7] = [ 
										RomanDigit{decimal:1, symbol: 'I'},
										RomanDigit{decimal:5, symbol: 'V'},
										RomanDigit{decimal:10, symbol: 'X'},
										RomanDigit{decimal:50, symbol: 'L'},
										RomanDigit{decimal:100, symbol: 'C'},
										RomanDigit{decimal:500, symbol: 'D'},
										RomanDigit{decimal:1000, symbol: 'M'},
										
									];
	
	fn new(num: i32, sym: char) -> RomanDigit {
		RomanDigit {
			decimal: num,
			symbol: sym,
		}
	}
	
	fn from_decimal<'a>(num: i32) -> Option<&'a RomanDigit> {

		for digit in &RomanDigit::ROMAN_DIGITS {
			if digit.decimal == num && digit.decimal == num {
				return Some(digit);
			}
		}
		
		None
	}

	fn from_symbol<'a>(num: char) -> Option<&'a RomanDigit> {

		for digit in &RomanDigit::ROMAN_DIGITS {
			if digit.symbol == num{
				return Some(digit);
			}
		}
		
		None
	}
}


fn convert_to_roman(number: i32) -> String {
	let mut builder: String = String::new();
	let mut  value = number;
	let mut num_index = RomanDigit::ROMAN_DIGITS.len() - 1;

	while value > 0 {
		let num = &RomanDigit::ROMAN_DIGITS[num_index];
		
		if value >= num.decimal {
			value -= num.decimal;
			builder.push(num.symbol);
		} else {
			
			for n in 0..num_index {
				let nextDigit = &RomanDigit::ROMAN_DIGITS[n];
				let mut combinedValue = num.decimal - nextDigit.decimal;
				
				 //Prevent Stuff like VX = 5 from happening
				if combinedValue == nextDigit.decimal {
					continue;
				}
				
				if value >= combinedValue {
		
					value -= combinedValue;
					builder.push(nextDigit.symbol);
					builder.push(num.symbol);
				}
			
			}
			
			//Move on to smaller number
			num_index -= 1;
			
		}
	}
	
	return builder;
}


fn convert_to_decimal(roman: &String) -> Result<i32,String> {

	let mut value = 0;
	let mut lastValue = 0;
	//println!("Converting {}", roman);
	
	for digit in roman.chars() {
		let num: Option<&RomanDigit> = RomanDigit::from_symbol(digit);
		
		//println!("on step {}", digit);
		match num {
			Some(rDigit) => { 
			
				if (rDigit.decimal > lastValue) {
					value -= lastValue * 2;
					//println!("Reversing value {}", lastValue);
				}
				value += rDigit.decimal;
				lastValue = rDigit.decimal;
			},
			
			None => return Err("Error".to_string())
		}
	}	
	Ok(value)
}

fn main() {
    println!("Hello, world!");	
	for n in (0..1001) {
		
		let roman: String = convert_to_roman(n);
		let result = convert_to_decimal(&roman);
		
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
