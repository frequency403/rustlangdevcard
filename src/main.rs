use crate::factory::PersonalInformation;

mod factory;

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}: ", prompt);
    std::io::stdin().read_line(&mut input).expect("Something went wrong");
    return input.trim().to_string();
}
fn question_for_change() -> bool {
    match get_user_input("Do you want to change the information? [Y/n]").as_str().trim() {
        "Y" | "y" | "j" | "J" => true,
        _ => { false },
    }

}
fn print_all_object_values(obj: &PersonalInformation) {
    println!("\n\nYour Information:\n\n\
    Firstname: {}\n
    Lastname: {}\n
    Email: {}\n
    Github: https:/github.com/{}\n
    Country: {}\n
    City: {}\n\n",
             obj.get_info("f"),
             obj.get_info("l"),
             obj.get_info("g"),
             obj.get_info("z"),
             obj.get_info("co"),
             obj.get_info("ci"));


}
fn change_object_property(info: &mut PersonalInformation) -> bool {
    let field_value = get_user_input("Enter the property you want to change");
    return if field_value.as_str().contains("first") | field_value.as_str().contains("First") {
        info.change_info("firstname", get_user_input("New entry for Firstname"));
        true
    } else if field_value.as_str().contains("last") | field_value.as_str().contains("Last") {
        info.change_info("lastname", get_user_input("New entry for Lastname"));
        true
    } else if field_value.as_str().contains("github") | field_value.as_str().contains("githubuser") {
        info.change_info("github", get_user_input("New entry for Github Username"));
        true
    } else if field_value.as_str().contains("mail") | field_value.as_str().contains("email") {
        info.change_info("email", get_user_input("New entry for Email"));
        true
    }else if field_value.as_str().contains("country") | field_value.as_str().contains("Country") {
        info.change_info("country", get_user_input("New entry for Country"));
        true
    } else if field_value.as_str().contains("city") | field_value.as_str().contains("City") {
        info.change_info("city", get_user_input("New entry for City"));
        true
    } else { false }
}

fn init_object() -> PersonalInformation {

    PersonalInformation::initialize(
        get_user_input("Enter your Firstname"),
        get_user_input("Enter your Lastname"),
        get_user_input("Enter the Github Username"),
        get_user_input("Enter your Email"),
        get_user_input("Enter the Country"),
        get_user_input("Enter your City")
    )
}
fn loopit(info: &mut PersonalInformation) {
    print_all_object_values(&info);
    if question_for_change() {
        if change_object_property(info) {
            println!("Success!"); loopit(info);
        } else { println!("Something failed"); }
    } else { info.gen_card() }
}
fn main() {
    let mut info = init_object();
    loopit(&mut info);
}
