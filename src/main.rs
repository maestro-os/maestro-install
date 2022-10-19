use std::io::BufRead;
use std::io::Write;
use std::io;

fn main() {
	println!("##     ##    ###    ########  ######  ######## ########   #######  
###   ###   ## ##   ##       ##    ##    ##    ##     ## ##     ## 
#### ####  ##   ##  ##       ##          ##    ##     ## ##     ## 
## ### ## ##     ## ######    ######     ##    ########  ##     ## 
##     ## ######### ##             ##    ##    ##   ##   ##     ## 
##     ## ##     ## ##       ##    ##    ##    ##    ##  ##     ## 
##     ## ##     ## ########  ######     ##    ##     ##  #######  ");

	println!();
	println!("Welcome to the maestro installer!");
	println!();
	println!("To begin the installation, press ENTER.");

	let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();

	let _ = lines_iter.next();
	println!();



	println!();
	println!("|> Step 1: Localization");

	// TODO Language
	// TODO Contient/Country
	// TODO Timezone



	println!();
	println!("|> Step 2: System informations");

	// TODO Add a characters limit?
	print!("Type system hostname: ");
	io::stdout().flush();
	let _hostname = lines_iter.next();

	// TODO



	println!();
	println!("|> Step 3: Creating administrator user");

	// TODO Add a characters limit?
	print!("Type admin username: ");
	io::stdout().flush();
	let _username = lines_iter.next();

	print!("Type admin/root password: ");
	io::stdout().flush();
	// TODO Disable prompting
	let _pass = lines_iter.next();
	// TODO Re-enable prompting

	print!("Confirm admin/root password: ");
	io::stdout().flush();
	// TODO Disable prompting
	let _pass_confirm = lines_iter.next();
	// TODO Re-enable prompting

	// TODO Check both passwords correspond



	println!();
	println!("|> Step 4: Disks partitions");

	// TODO Detect partitions/systems that are already present
	// TODO



	println!();
	println!("|> Step 5: Installation");

	// TODO Ask for confirmation

	// TODO Perform install
}
