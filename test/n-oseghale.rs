use std::io;
fn main(){
	let mut input1 = String::new();

	println!("How many siblings do you have?");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let siblings:i32 = input1.trim().parse().expect("Not a valid number");
	
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();

	println!("Enter name of sibling");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let name = input2.trim().parse().expect("Not a name");

	println!("Enter age of sibling");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let age:i32 = input3.trim().parse().expect("Not a valid age");

	println!("Enter gender of sibling");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let gender = input4.trim().parse().expect("Not a gender");

	println!("Enter country of residence of sibling");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let country = input5.trim().parse().expect("Not a country");

	for i in 1..siblings{

	}


	if age>18 && age==18
	{
		let mut relationship_status = String::new;
		println!("Are they married, single or in a relationship");
		io::stdin().read_line(&mut input3).expect("No");

		if relationship_status == married
		{
			let mut no_of_children:i32 = String::new;
			println!("Number of children");

			if no_of_children<1 && no_of {children ==0}
			{
				println!("Go and have kids jare what  are you ddoing with your life");
			}
			else{
				let mut input6 = String::new;
				println!("Enter child's name");
				io::stdin().read_line(&mut input6).expect("Not a valid name");
				let childs_name = input6.trim().parse().expect("Something");

				let mut input7:i32 = String::new;
				println!("Enter child's age");
				io::stdinn().read_line(&mut input7).expect("Not a valid  string");
				let childs_age = input7.trim().parse().expect("Something");

				let mut input8 = String::new;
				println!("Enter child's scchool");
				io::stdin().read_line(&mut input6).expect("Not a valid name");
				let childs_school = input7.trim().parse().expect("Something");}
		}
		if relationship_status ==single
		{
			println!("Are you a student or employed");
			let mut status =  String::new;

			if status == student
			{
				let mut input8 = String::new;
				println!("University");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let  university_name = input8().trim().parse().expect("Something");

				let mut input9 = String::new;
				println!("Course of study");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let  course = input9().trim().parse().expect("Something");

				let mut input10 = String::new;
				println!("Year of study");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let year_of_study = input10().trim().parse().expect("Something");

				let mut input11 = String::new;
				println!("Home or aborad?");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let study_country= input11().trim().parse().expect("Something");

				if study_country ==abroad
				{
					println!("Country is ", country);
						}

			}
			if status == employed
			{
				let mut input13 = String::new;
				println!("Is job remote, on site or hybrid?");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let job = input13().trim().parse().expect("Something");

				if job == on {site 
				{
					{
					let mut input14 = String::new;
					}
				println!("Company name");
				io::stdin().read_line(&mut input14).expect("Not a valid string");
				let company_name = input14.trim().parse().expect("Something");

				let mut input15 = String::new;
				println!("Job title");
				io::stdin().read_line(&mut input13).expect("Not a valid string");
				let job_title = input15.trim().parse().expect("Something");

				let mut input16 = String::new;
				println!("Industry sector");
				io::stdin().read_line(&mut input13).expect("Not a valid string");
				let industry_sector = input16.trim().parse().expect("Something");
				} }
}
			}
		}
	if relationship_status == relationship 
	{
		let mut input17 = String::new;
				println!("How many years have you been with your partner");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let years = input17().trim().parse().expect("Something");

				let mut input18 = String::new;
				println!("Name of partner");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let partner_name = input18().trim().parse().expect("Something");

				let mut input19 = String::new;
				println!("Do you live with your partner");
				io::stdin().read_line(&mut String).expect("Not a valid string");
				let yes,no = input19.trim().parse().expect("Something");

				if yes,no == yes 
				{
					{
					println("they reside in",country);
					}
			}	}
        
	

	if age < 18 
	{
		println!("Have you written WAEC?");
		let mut input20 = String::new;
		io::stdin().read_line(&mut input20).expect("Not a valid string");
		let waec or not = input20.trim().parse().expect("something");

		if waec or not == yes 
		{
			let mut input21 = String::new;
				println!("Seoondary school attended");
				io::stdin().read_line(&mut input21).expect("Not a valid string");
				let secondary_school = input21.trim().parse().expect("Something");

				let mut input22 = String::new;
				println!("Input fianl grade");
				io::stdin().read_line(&mut input22).expect("Not a valid string");
				let grade = input22.trim().parse().expect("Something");

				let mut input23 = String::new;
				println!("Year of completion");
				io::stdin().read_line(&mut input23).expect("Not a valid string");
				let year_of_completion = input23.trim().parse().expect("Something");
		}
		if waec,not == not
		{
			let mut input24 = String::new;
				println!("Current class level");
				io::stdin().read_line(&mut input24).expect("Not a valid string");
				let class = input24.trim().parse().expect("Something");

				let mut input25 = String::new;
				println!("Do  you plan to  take WAEC soon?");
				io::stdin().read_line(&mut input25).expect("Not a valid string");
				let are you taking it soon = input25.trim().parse().expect("Something");

				if are you taking it soon == yes 
				{
					let mut input26:i32 = String::new;
				println!("What year");
				io::stdin().read_line(&mut input26).expect("Not a valid string");
				let year_of_waec = input26.trim().parse().expect("Something");
				}

		}
	}


}
