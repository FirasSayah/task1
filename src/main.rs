use std::io;
use rand::Rng;


fn main() {
     // display title
     println!("\n*********************************************************************************");
     println!("Welcome to vending/change machine !!");
     println!("**********************************************************************************\n");
    
    'main: loop {

        let rand_num: f32 = rand::thread_rng().gen_range(0.00..10.00); // generates number
        let price: f32 = (rand_num * 100.0).round() / 100.0; // formats the number as price
        let mut user_inputf: f32;

        // display price & user guide
        println!("\n****************");
        println!("Price: {}", price);
        println!("****************\n");
        
        
        loop {
            println!("Please enter payment below in \"000.00\" format!");
            //----------------------------------------------------------------------
            // get user input in string format
            let mut user_input: String = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
    
            // see if user wants to quit
            if user_input.trim().to_lowercase().contains("q"){

                println!("Are you sure do quit ?");
                println!("\n0: No");
                println!("1: Yes  \n");

                let mut user_input: String = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
                if user_input.trim().to_lowercase().contains("1"){
                    println!("quitting...");
                    break 'main;
                }
                else if user_input.trim().to_lowercase().contains("0") {
                    user_input.clear();
                    continue;
                }
                else {
                    println!("This is not a Option ! try again Please..");
                    continue;

                }
            }

            // check user_input format 
            if user_input.trim().len() != 6 || user_input.chars().any(char::is_alphabetic) {
                println!("Wrong format!");
                user_input.clear();
                continue;
            }

            // try to parse user_input to f32 to be able to compare with price
            user_inputf = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if user_inputf < price{
                println!("Not enough cash!");
                user_input.clear();
                continue;
            }

            let difference = ((user_inputf - price) * 100.00 ).round() / 100.00;
            if difference > 0.00f32 {
                println!(" Here is your change: {}", difference);
                break;
            } 
            if difference == 0.00f32 {
                println!("Thank you! Payment accepted!");
                break;
            }

        }

    }


}
    




