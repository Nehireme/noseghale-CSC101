use std::io;
fn main() {
    println!("WELCOME TO THE PUBLIC SERVICE APS LEVEL CHECKER\nENDORSED BY THE FEDERAL GOVERNMENT OF NIGERIA (not really it's just me)\n");
    println!("What is your field of work fromm the array before you?");
    let w = vec!["Office Administrator","Academic","Lawyer","Teacher"];
    println!("{:?}",w);

    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Not a valid string");

    if input1.to_lowercase().trim() =="office administrator"{

        let a = vec!["-","APS 1-2","APS 1-2","APS 3-5","APS 3-5","APS 5-8","APS 5-8","APS 5-8","EL1 8-10","EL1 8-10","EL2 10-13","EL2 10-13","EL2 10-13","SES"];
        let b = vec!["-","Intern","Intern","Administrator","Administrator"," Senior Administrator","Senior Administrator","Senior Administrator","Office manager","Office manager","Director","Director","Director","CEO"];        
        let mut input2 = String::new();
    println!("How many years have you worked?");
    std::io::stdin().read_line(&mut input2).expect("Invalid string");
    let years:usize = input2.trim().parse().expect("invalid");

    let thing = a[years];
    let thing1 = b[years];
    println!("you are a/an [{}], position is [{}]",thing1,thing);    
}

else if input1.to_lowercase().trim() =="academic"{

        let a = vec!["-","APS 1-2","APS 1-2","APS 3-5","APS 3-5","APS 5-8","APS 5-8","APS 5-8","EL1 8-10","EL1 8-10","EL2 10-13","EL2 10-13","EL2 10-13","SES"];
        let c = vec!["-","-","-","Research assistant","Research Assistant","PhD candidate","PhD candidate","PhD Candidate","Post- Doc Researcher","Post-Doc Researcher","Senior Lecturer","Senior Lecturer","Senior Lecturer","Dean"];        
        let mut input2 = String::new();
    println!("How many years have you worked?");
    std::io::stdin().read_line(&mut input2).expect("Invalid string");
    let years:usize = input2.trim().parse().expect("invalid");

    let thing = a[years];
    let thing2 = c[years];
    println!("you are a/an [{}], position is [{}]",thing2,thing);
}
else if input1.to_lowercase().trim() =="lawyer"{

        let a = vec!["-","APS 1-2","APS 1-2","APS 3-5","APS 3-5","APS 5-8","APS 5-8","APS 5-8","EL1 8-10","EL1 8-10","EL2 10-13","EL2 10-13","EL2 10-13","SES"];
        let d = vec!["-","Paralegal","Paralegal","Junior Associate","Junior Associate","Associate","Asssociate","Associate","Senir Associate 1-2","Senior Ssociate 1-2","Senior Asssociate 3-4","Senior Associate 3-4","Senior Associate 3-4","Partner"];        
        let mut input2 = String::new();
    println!("How many years have you worked?");
    std::io::stdin().read_line(&mut input2).expect("Invalid string");
    let years:usize = input2.trim().parse().expect("invalid");

    let thing = a[years];
    let thing3 = d[years];
    println!("you are a/an [{}], position is [{}]",thing3,thing);
}
else if input1.to_lowercase().trim() =="teacher"{

        let a = vec!["-","APS 1-2","APS 1-2","APS 3-5","APS 3-5","APS 5-8","APS 5-8","APS 5-8","EL1 8-10","EL1 8-10","EL2 10-13","EL2 10-13","EL2 10-13","SES"];
        let e = vec!["-","Placement","Placement","Classroom teacher","Classroom teacher","Snr teacher","Snr teacher","Snr teacher","Leading teacher","Leadding teacher","Deputy principal","Deputy principal","Deputy principal","Principal"];        
        let mut input2 = String::new();
    println!("How many years have you worked?");
    std::io::stdin().read_line(&mut input2).expect("Invalid string");
    let years:usize = input2.trim().parse().expect("invalid");

    let thing = a[years];
    let thing4 = e[years];
    println!("you are a/an [{}], position is [{}]",thing4,thing);
}
}
