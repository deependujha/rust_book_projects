use std::collections::HashMap;

#[derive(Debug)]
enum CodingDomain {
    CoreRustDeveloper,
    QuantitativeFinance,
    ReinforcementLearning,
}

#[derive(Debug)]
struct Coder {
    unique_id: i32,
    name: String,
    uses_aws: bool,
    coding_domain: CodingDomain,
}

impl Coder {
    fn from(unique_id: i32, name: String, coding_domain: CodingDomain, uses_aws: bool) -> Self {
        return Coder {
            unique_id,
            name,
            coding_domain,
            uses_aws,
        };
    }

    fn can_work_with_aws(&self) -> bool {
        return self.uses_aws;
    }
}

fn main() {
    // Directly initialize Coder struct
    let nitesh = Coder {
        unique_id: 101,
        name: String::from("Nitesh"),
        uses_aws: true,
        coding_domain: CodingDomain::CoreRustDeveloper,
    };

    // uses helper function in impl block for struct object
    let deependu = Coder::from(
        102,
        String::from("Deependu"),
        CodingDomain::ReinforcementLearning,
        true,
    );
    let akash = Coder::from(
        103,
        String::from("Akash"),
        CodingDomain::QuantitativeFinance,
        false,
    );

    // println!("{:?}", nitesh);
    // println!("{:?}", deependu);
    // println!("{:?}", akash);
    // dbg!(nitesh.can_work_with_aws());
    // dbg!(deependu.can_work_with_aws());
    // dbg!(akash.can_work_with_aws());

    let mut my_coders: HashMap<i32, Coder> = HashMap::new();

    my_coders.insert(101, nitesh);
    my_coders.insert(102, deependu);
    my_coders.insert(103, akash);

    for (key, value) in &my_coders {
        println!("{key}: {:?}", value);
    }

    my_separator_printer();

    let my_101_coder = get_coder_details(101, &my_coders);
    let my_105_coder = get_coder_details(105, &my_coders);

    match my_101_coder {
        Some(cdr) => {
            println!("main fn==> 101 coder exists");
            println!("main fn==> {:?}", cdr);
        }
        None => {
            println!("main fn==> id: 101 coder doesn't exists");
        }
    }
    my_separator_printer();

    match my_105_coder {
        Some(cdr) => {
            println!("main fn==> 105 coder exists");
            println!("main fn==> {:?}", cdr);
        }
        None => {
            println!("main fn==> id: 105 coder doesn't exists");
        }
    }
}

fn get_coder_details(user_id: i32, my_coders: &HashMap<i32, Coder>) -> Option<&Coder> {
    let my_cdr = my_coders.get(&user_id); // it will return us option: either coder exists, or none

    if let Some(cdr) = my_cdr {
        println!("get_coder_details function==> Coder exists with id: {user_id}");
        println!(
            "get_coder_details function==> Coder details in get_coder_details function:  {:?}",
            cdr
        );
    } else {
        println!("get_coder_details function==> Coder doesn't exists with id: {user_id}");
    }
    my_separator_printer();

    return my_cdr;
}

fn my_separator_printer() {
    let my_separator = String::from("=").repeat(70);
    println!("{my_separator}");
}
