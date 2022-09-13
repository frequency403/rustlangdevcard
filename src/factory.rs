#[derive(Debug, PartialEq, Clone)]
pub struct PersonalInformation {
    firstname: String,
    lastname: String,
    githubname: String,
    email: String,
    country: String,
    city: String,
}

impl PersonalInformation {
    pub fn initialize(firstname: String, lastname: String, githubname: String, email: String, country: String, city: String) -> Self{
        PersonalInformation {
            firstname,
            lastname,
            githubname,
            email,
            country,
            city,
        }
    }
    pub fn get_info(&self, taip: &str) -> &String {
        let thaip = match taip {
            "f" => &self.firstname,
            "l" => &self.lastname,
            "g" => &self.githubname,
            "e" => &self.email,
            "co" => &self.country,
            "ci" => &self.city,
            _ => &self.firstname,
        };
        return thaip;
    }
    pub fn change_info(&mut self, field: &str, value: String) -> bool {
        return match field {
            "firstname" => {
                self.firstname = value;
                true
            },
            "lastname" => {
                self.lastname = value;
                true
            },
            "githubname" => {
                self.githubname = value;
                true
            },
            "email" => {
                self.email = value;
                true
            }
            "country" => {
                self.country = value;
                true
            },
            "city" => {
                self.city = value;
                true
            },
            _ => false,
        }
    }

    pub fn gen_card(&self) {
        fn create_separator(item: usize) -> String {
            let mut s = String::new();
            match item {
                0 => {for _ in 0..55 {s.push(' ')}s.push('|')},
                _ => {for _ in 0..56-item {s.push(' ')}s.push('|')}
            }
            return s;
        }
        fn print_a_line(length: usize) {
            let mut s = String::new();
            if length == 0 {return;}
            for _ in 0..length {
                s.push('-')
            }
            println!("{s}")
        }
        let logo = [
            "...............N8.7DD.OO................",
            "...........NDN8D888OOOZO8ZOO............",
            "........MNDDD88888...OZOOOZZZO$.........",
            ".....MM.NDDDDOD..88=8O..OZOZOOO.D8......",
            ".....8DDD8DD......,O.......OOOOD8D......",
            "...NNMNNNN...................OO8DODO....",
            "...DMMNNNNDD8O888DO88O88D8.....88DDO....",
            ".NNNMMNDNDDD88888D88OND88DDN:...DOZZOO..",
            "..MN8NN88DDDD88O8DODDO88DND88...DNDD8...",
            ".MDD..MN..8DN888.......DDND8DN.8:..OOZ..",
            "DDNNNMDN..NN8D8D.......D8DDDO..D8O88OZ8.",
            ".8NMM.....DDN8NDNNDNNNDD88DD......O8ZO..",
            "DDMMN.....NM88NNMNMNDN8D8DND......$Z$ZZO",
            ".8NNM.....DD8DDD.....NDDDDDD8...ZD8888..",
            "DDNMM.....8DDDMN......DD888Z$...8D88ODD.",
            ".NDMMM88NNDNDDNMNNNN...ODZ8ZDN88DD8888..",
            "..MMMMDNDMNMNNNNNNDD...8NDDNNDDO8888O...",
            ".DMDND8NNNNNNNMNNNDD....DDND8N88DDDZON..",
            "...MNNNN.......................D8ODN....",
            "...NNNNNNMNMD.............D888DD8N88....",
            ".....8N8M$..N.............N..DMNNN......",
            ".....NM.NDOON8NN.......DD888NND.MN......",
            "........NNONNN8ND888DODNNNDMNNN.........",
            "...........8NO8DO88DMDNNNMNM............",
            "...............MD.ZNN.NN................"
        ];
        let info = [
            format!("|   Name: {} {}", &self.firstname, &self.lastname),
            format!("|   Email: {}", &self.email),
            format!("|   GitHub Account: https://github.com/{}", &self.githubname),
            format!("|   City: {}", &self.city),
            format!("|   Country: {}", &self.country)
        ];
        let mut longest_info = 0;
        for item in &info {
            if item.len() < longest_info {longest_info = item.len()}
        }

        print_a_line(logo[0].len() + 60);
        for i in 0..logo.len() {
            let repline = logo[i].replace(".", " ");
            match i {
                1 => println!("| {repline} | Developer with the Rust Programming Language          |"),
                4 => println!("| {repline} {}{}", info[0], create_separator(info[0].len())),
                9 => println!("| {repline} {}{}", info[1], create_separator(info[1].len())),
                14 => println!("| {repline} {}{}", info[2], create_separator(info[2].len())),
                19 => println!("| {repline} {}{}", info[3], create_separator(info[3].len())),
                24 => println!("| {repline} {}{}", info[4], create_separator(info[4].len())),
                _ => println!("| {repline} |{}", create_separator(0))
            }
        }
        print_a_line(logo[0].len() + 60);
    }

}
