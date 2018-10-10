fn main(){
	let x = "Abhishek";
	println!("Hello {}", x);

	loop{
		if x == "Abhishek" {
			println!("{} is the admin.", x);}
		else {
			println!("{} is intruder!", x);
		}
		break;
	}

}

