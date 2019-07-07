


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


	pub fn convert_to_roman(number: i32) -> String {
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
					let next_digit = &RomanDigit::ROMAN_DIGITS[n];
					let combined_value = num.decimal - next_digit.decimal;
					
					 //Prevent Stuff like VX = 5 from happening
					if combined_value == next_digit.decimal {
						continue;
					}
					
					if value >= combined_value {
			
						value -= combined_value;
						builder.push(next_digit.symbol);
						builder.push(num.symbol);
					}
				
				}
				
				//Move on to smaller number
				num_index -= 1;
				
			}
		}
		
		return builder;
	}


	pub fn convert_to_decimal(roman: &String) -> Result<i32,String> {

		let mut value = 0;
		let mut last_value = 0;
		//println!("Converting {}", roman);
		
		for digit in roman.chars() {
			let num: Option<&RomanDigit> = RomanDigit::from_symbol(digit);
			
			//println!("on step {}", digit);
			match num {
				Some(r_digit) => { 
				
					if r_digit.decimal > last_value {
						value -= last_value * 2;
						//println!("Reversing value {}", lastValue);
					}
					value += r_digit.decimal;
					last_value = r_digit.decimal;
				},
				
				None => return Err("Error".to_string())
			}
		}	
		Ok(value)
	}

