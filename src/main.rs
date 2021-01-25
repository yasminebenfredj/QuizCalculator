extern crate rand;
use std::time::Instant;
use std::io;
use std::str::FromStr;
use rand::thread_rng;
use rand::Rng;
use std::io::Write;

fn main() 
{
	println!("\n\n\n** Debut du jeu de Quiz Calculator **");

	'play:loop {
		let start = Instant::now();

		if game() == true 
		{
			let end = Instant::now();
			println!("Votre reflexion a durée : {:?}.", end - start);
			continue;
		}
		else {
			println!("Vous avez perdu ! \n");
			break 'play;
		}
	}

	println!("** Fin du jeu Quiz Calculator **\n\n\n");
}


fn game() -> bool 
{
	let mut attempt : i32 = 5 ;

	let a = _genrate_number();
	let b = _genrate_number();

	let _result = a + b;

	print!("\nDonnez la solution pour : {:?} + {:?} =  ",a, b );
	while attempt > 0 
	{
		io::stdout().flush();

		match _enter_result() {
			Some(nb) => 
			{
				if nb == _result 
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


fn _genrate_number() -> isize
{
	let mut rng = thread_rng();
	let random_number :isize = rng.gen_range(1,101);
	return random_number ;

}

fn _enter_result() -> Option<isize>
{
	let mut result = String::new();
	match io::stdin().read_line(&mut result)
	{
		Ok(_) => {
			match isize::from_str(&result.trim()) // trim : enleve cararctere vide
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