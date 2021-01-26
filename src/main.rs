extern crate rand;
use std::time::Instant;
use std::io;
use std::str::FromStr;
use rand::thread_rng;
use rand::Rng;
use std::io::Write;
use std::convert::TryInto;
use std::string::String;


fn main() 
{
	println!("\n\n\n** Debut du jeu de Quiz Calculator **");
	let mut score =0i32;

	'play:loop {
		let start = Instant::now();
		println!("#Score: {:?}", score);


		if score < 0
		{
			println!("Vous avez perdu ! \n");
			break 'play;
		}

		if game() == true 
		{
			let end = Instant::now();
			println!("Votre reflexion a durée : {:?}.\n", end - start);
			score += 2;
			continue;
		}
		else {
			println!("Vous avez perdu 2 points .");
			score -= 2;
			continue;

		}

	}

	println!("** Fin du jeu Quiz Calculator **\n\n\n");
}


fn game() -> bool 
{
	let mut attempt : i32 = 5 ;

	let a = _genrate_number(1,101);
	let b = _genrate_number(1,a);

	let choice : usize = _genrate_number(0,4).try_into().unwrap();
	let op = _operation(choice);

	let _result : i32 = match choice  {
		0 => a + b,
		1 => {
			if((a-b)>0) 
			{a - b}
			else {b - a}},
		2 => a * b,
		_ => a /  b
	};

	print!("\nDonnez la solution pour : {:?} {} {:?} =  ",a,op, b );
	while attempt > 0 
	{
		io::stdout().flush();

		match _enter_result() {
			Some(nb) => 
			{
				if nb == _result.try_into().unwrap() 
				{
					println!("Bravo! vous avez gagner avec {} essaies.\n", 6 - attempt );
					return true;
				}
				else {
					if attempt > 1 {
						println!("C'est faut! Il vous reste {} chance(s) Réessayez...", attempt-1 );
					}
				}
			},
			None => {}
		}
		attempt -= 1;
	}

	false
}

fn _operation(i : usize  ) -> String
{
	let mut operations : Vec<String> = Vec::new();
	operations.push(String::from("+"));
	operations.push(String::from("-"));
	operations.push(String::from("*"));
	operations.push(String::from("//"));

	let op  = &operations[i] ;

	op.to_string()

}


fn _genrate_number(mini : i32, maxi : i32) -> i32
{
	let mut rng = thread_rng();
	let random_number :i32 = rng.gen_range(mini,maxi);
	return random_number ;

}

fn _enter_result() -> Option<usize>
{
	let mut result = String::new();
	match io::stdin().read_line(&mut result)
	{
		Ok(_) => {
			match usize::from_str(&result.trim()) // trim : enleve cararctere vide
			{
				Ok(number) => Some(number),
				Err(_) => {
					println!("Entez un nombre valide ! ");
					None
				}
			}
		},
		_ => 
		{
			println!("Erreur : nombre invalide");
			None
		}
	}
}