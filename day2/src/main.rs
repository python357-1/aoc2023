#[derive(Debug)]
struct Round {
    red_count: i32,
    blue_count: i32,
    green_count: i32
}

impl Round {
    fn from_str(input: &str) -> Round {
        //input should look like "x red, y green, z blue"
        let mut r = Round{red_count: 0, blue_count: 0, green_count: 0};
        input
            .split(", ")
            .for_each(|x| {
                if x.contains("green") {
                    r.green_count = x.split(' ').collect::<Vec<_>>()[0].parse::<i32>().expect("Could not convert string to i32");
                }
                if x.contains("red") {
                    r.red_count = x.split(' ').collect::<Vec<_>>()[0].parse::<i32>().expect("Could not convert string to i32");
                }
                if x.contains("blue") {
                    r.blue_count = x.split(' ').collect::<Vec<_>>()[0].parse::<i32>().expect("Could not convert string to i32");
                }
            });

        r
    }

    fn is_valid(&self) -> bool {
        if self.red_count <= 12 && self.green_count <= 13 && self.blue_count <= 14 {
            return true
        }

        false
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
/*    max_red: Option<i32>,
    max_green: Option<i32>,
    max_blue: Option<i32>*/
}

impl Game {
    fn new(id: i32, rounds: Vec<Round>) -> Game {
        Game{id, rounds, /*max_red: None, max_blue: None, max_green: None*/}
    }

    fn game_from_str(input: &str) -> Result<Game, &'static str> {
        let game_details = input.split("").take_while(|x| *x != ":").collect::<String>();
        let game_details_vec = game_details.split(' ').collect::<Vec<_>>();
        let id = game_details_vec.get(1).ok_or("Could not get game details");
        let id = id.unwrap().parse::<i32>().expect("could not convert game details id to i32");
        let round_details = input.split(": ").skip(1).collect::<String>();
        let rounds = round_details.split("; ").map(Round::from_str).collect::<Vec<Round>>();
        Ok(Game::new(id, rounds))
    }

    /*fn find_max(&mut self) {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        self.rounds.iter().for_each(|x| {
            if x.red_count > max_red { max_red = x.red_count }
            if x.green_count > max_green { max_green = x.green_count }
            if x.blue_count > max_blue { max_blue = x.blue_count }
        });

        self.max_red = Some(max_red);
        self.max_blue = Some(max_blue);
        self.max_green = Some(max_green);
    }*/
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let games = input
        .lines()
        .map(|x| Game::game_from_str(x).expect("from_str did not return a Game"))
        .collect::<Vec<_>>();

    let mut valid_game_ids: Vec<i32> = vec![];

    for game in games {
        if game.rounds.iter().all(|x| x.is_valid()) {
            valid_game_ids.push(game.id)
        }
    }

    println!("{:?}", valid_game_ids);
    println!("{:?}", valid_game_ids.iter().sum::<i32>());
    println!("{:?}", valid_game_ids.len());

}
