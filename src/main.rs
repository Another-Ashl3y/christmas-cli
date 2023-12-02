use colored::*;
use std::io;
use clearscreen;
use console::Term;
use std::fs::File;
use chrono::Utc;
use std::io::prelude::*;
use arboard::Clipboard;
use rand::Rng;

fn main() {
    cls();
    println!("\n\n\n");
    print_christmas_tree();
    println!("\n\n\n     {}\n","-------------------------------".bold().black());
    println!("\t{}\n","Welcome to the workspace!".bold().cyan());
    cls();

    let stdout = Term::buffered_stdout();
    'main: loop {
        // cls();
        println!("\n\n\n");
        print_christmas_tree();
        println!("\n\n\n     {}\n","-------------------------------".bold().black());
        println!();
        display_options();
        if let Ok(character) = stdout.read_char() {
            println!("{}{}","-> ".to_string().black().bold(), character.to_string().bright_black().bold());
            match character {
                '0' => make_card(),
                '1' => {
                    let t = Utc::now();
                    println!("{}", t);
                },
                '2' => cls(),
                'e' => break 'main,
                _ => (),
            }
        }
        // println!("->");
        // let mut input = String::new();
        // let _ = io::stdin().read_line(&mut input).unwrap();
        // let input = input.trim();
        // if input == "exit" {
        //     break 'main;
        // }
        
    }
}

fn make_card() {
    cls();
    print_christmas_star();

    // To
    println!("{}","Who is this for?".to_string().cyan().bold());
    let mut name = String::new();
    let _ = io::stdin().read_line(&mut name).unwrap();
    let name =  ["Dear", name.trim(),",\n"].join(" ");

    // Message
    println!("{}","What is your special message?".to_string().cyan().bold());
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let line = ["\t", line.trim(),"\n"].join("");

    // From
    println!("{}","Who is it from?".to_string().cyan().bold());
    let mut from = String::new();
    let _ = io::stdin().read_line(&mut from).unwrap();
    let from = ["From,\n\t", from.trim()].join("");

    // Whole card
    
    let card = format_card(name, line, from);

    // File
    let mut file = File::create("./card.txt").expect("making creation failed");
    file.write_all(card.as_bytes()).expect("write failed");

    // Copy to clipboard
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(&card).expect("Copy failed for some reason");

    // Outro
    println!("{}{}","Card file made and copied to clipboard".bold().cyan(), card.as_str().bold().black());
}

fn format_card(name: String, line: String, from: String) -> String {let stdout = Term::buffered_stdout();
    println!("Is it a personal card? (y/n)");
    if let Ok(character) = stdout.read_char() {
        println!("{}{}","-> ".to_string().black().bold(), character.to_string().bright_black().bold());
        match character {
            'y' => {
                let card = [
                    get_random_cover(),
                    // "```".to_string(),
                    name, 
                    "\tMerry Christmas!".to_string(),
                    random_message(),
                    // "\tThis was made with a program I made.".to_string(),
                    line,
                    "\tLove you lots!".to_string(),
                    from,
                    // "```".to_string()
                ].join("\n");
                return card;
            },
            _ => {
                let card = [
                    get_random_cover(),
                    // "```".to_string(),
                    name, 
                    "\tMerry Christmas!".to_string(),
                    random_message(),
                    // "\tThis was made with a program I made.".to_string(),
                    line,
                    from,
                    // "```".to_string()
                ].join("\n");
                return card;
            },
        }
    }
    return "Failed".to_string()
}

fn random_message() -> String {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0..16);
    match y {
        0 => "\tWishing you a white Christmas! (And when you run out of white, just open a bottle of red).".to_string(),
        1 => "\tA Christmas reminder: Don\'t try to borrow any money from elves ... they're always a little short. Have a Merry Christmas!".to_string(),
        2 => "\tThey say the best Christmas gifts come from the heart … but cash and gift cards do wonders too! Happy Holidays!".to_string(),
        3 => "\tRemember, Santa is watching. Everything. Yes, even that. Anyway, Merry Christmas!".to_string(),
        4 => "\tEat. Drink. Be Merry. Have a wonderful Christmas!".to_string(),
        5 => "\tPlease note: Christmas is canceled. Apparently, you told Santa you have been good this year … he died laughing.".to_string(),
        6 => "\tMerry Christmas! I put so much thought into your gift that now it's too late to get it.".to_string(),
        7 => "\tThis Christmas, may your family be functional and all your batteries be included.".to_string(),
        8 => "\tI told Santa you were good this year and sent him a link to your Pinterest board. Merry Christmas to you!".to_string(),
        _ => "\tThis holiday season, let\'s make it a point to cherish what\'s truly important in our lives: cookies.".to_string(),
    }

}

fn display_options() {
    let options = [
        ["0","Make Card"],
        ["1","Get date"],
        ["2","Clear screen"],
        ["3","Nothing at the moment"],
        ["4","Nothing at the moment"],
        ["5","Nothing at the moment"],
        ["e","Exit"],
    ];
    for i in options {
        println!("[{}] - {}", i[0].to_string().red().bold(), i[1].to_string().green().bold());
    }
}

fn print_christmas_tree() {    
    println!("\t{}", "         |".green().bold());
    println!("\t{}", "        -+-".yellow().bold());
    println!("\t{}", "         A".green().bold());
    println!("\t{}{}", "        /=\\".green().bold(),"               /\\  /\\    ___  _ __  _ __ __    __".red().bold());
    println!("\t{}{}", "      i/ O \\i".green().bold(),"            /  \\/  \\  / _ \\| '__|| '__|\\ \\  / /".red().bold());
    println!("\t{}{}", "      /=====\\".green().bold(),"           / /\\  /\\ \\|  __/| |   | |    \\ \\/ /".red().bold());
    println!("\t{}{}", "      /  i  \\".green().bold(),"           \\ \\ \\/ / / \\___/|_|   |_|     \\  /".red().bold());
    println!("\t{}{}", "    i/ O * O \\i".green().bold(),"                                       / /".red().bold());
    println!("\t{}{}", "    /=========\\".green().bold(),"        __  __                        /_/    _".red().bold());
    println!("\t{}{}", "    /  *   *  \\".green().bold(),"        \\ \\/ /        /\\  /\\    __ _  ____  | |".red().bold());
    println!("\t{}{}", "  i/ O   i   O \\i".green().bold(),"       \\  /   __   /  \\/  \\  / _` |/ ___\\ |_|".red().bold());
    println!("\t{}{}", "  /=============\\".green().bold(),"       /  \\  |__| / /\\  /\\ \\| (_| |\\___ \\  _".red().bold());
    println!("\t{}{}", "  /  O   i   O  \\".green().bold(),"      /_/\\_\\      \\ \\ \\/ / / \\__,_|\\____/ |_|".red().bold());
    println!("\t{}", "i/ *   O   O   * \\i".green().bold());
    println!("\t{}", "/=================\\".green().bold());
    println!("\t{}", "       |___|".black().bold());
}

fn print_christmas_star() {
    println!("\n\n\t{}","                   ,.".to_string().yellow().bold());
    println!("\t{}","                 ,'  `.".to_string().yellow().bold());
    println!("\t{}","               ,' _<>_ `.".to_string().yellow().bold());
    println!("\t{}","             ,'.-'____`-.`.".to_string().yellow().bold());
    println!("\t{}","           ,'_.-''    ``-._`.".to_string().yellow().bold());
    println!("\t{}","         ,','      /\\      `.`.".to_string().yellow().bold());
    println!("\t{}","       ,' /.._  O /  \\ O  _.,\\ `.".to_string().yellow().bold());
    println!("\t{}","     ,'/ /  \\ ``-;.--.:-'' /  \\ \\`.".to_string().yellow().bold());
    println!("\t{}","   ,' : :    \\  /\\`.,'/\\  /    : : `.".to_string().yellow().bold());
    println!("\t{}","  < <>| |   O >(< (  ) >)< O   | |<> >".to_string().yellow().bold());
    println!("\t{}","   `. : :    /  \\/,'`.\\/  \\    ; ; ,'".to_string().yellow().bold());
    println!("\t{}","     `.\\ \\  /_..-:`--';-.._\\  / /,'".to_string().yellow().bold());
    println!("\t{}","       `. \\`'   O \\  / O   `'/ ,'".to_string().yellow().bold());
    println!("\t{}","         `.`._     \\/     _,','".to_string().yellow().bold());
    println!("\t{}","           `..``-.____.-'',,'".to_string().yellow().bold());
    println!("\t{}","             `.`-.____.-','".to_string().yellow().bold());
    println!("\t{}","               `.  <>  ,'".to_string().yellow().bold());
    println!("\t{}","                 `.  ,' ".to_string().yellow().bold());
    println!("\t{}\n\n","                   `'".to_string().yellow().bold());
}

fn get_random_cover() -> String {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0..16);
    match y {
        0 => {
"---------------------------------------------------------

\t\t        __
\t\t      .-'  |
\t\t     /   <\\|
\t\t    /     \\'
\t\t    |_.- o-o
\t\t    / C  -._)\\
\t\t   /',        |
\t\t  |   `-,_,__,'
\t\t  (,,)====[_]=|
\t\t    '.   ____/
\t\t     | -|-|_
\t\t     |____)_) 

---------------------------------------------------------
".to_string()
        },
        1 => {
"---------------------------------------------------------
                 ,,'''''',,
               ;;;;;,,,,   '',
              ;''''''':;;;,,  ',
             ``` ''''  '';;;,  ;
            .-'  `      _ `;;',;
           (    ,   '  ',) '; ;;
            `;'`',    , /  ;  ,:,
           ,' --' ',-' ;  (  ;::::
          ;   '         ;,;;--;;'
          ; ,       , `;;;;     `,
          ;,  ,   ,  ,;;;'        ;
           ; , ,   ,;;;'           ;
            `;`'`;;;;'              ;
           ,'    ;;;'               jv
                 ;;;

---------------------------------------------------------
".to_string()
        },
        2 => {
"---------------------------------------------------------
\t\t   ,--.
\t\t  ()   \\
\t\t   /    \\
\t\t _/______\\_
\t\t(__________)
\t\t(/  @  @  \\)
\t\t(`._,()._,')
\t\t(  `-'`-'  )
\t\t \\        /
\t\t  \\,,,,,,/



---------------------------------------------------------
".to_string()
        },
        _ => {
"
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WW         ___ __     ____      ___    ___   _   _          WW
WW        |   V  \\   / _  )    / __|  / __| | | | |         WW
WW        | /\\ /\\ | ( (/ /_   | /    | /    \\ \\_| |         WW
WW        |_||_||_|  \\_____|  |_|    |_|     \\__  |         WW
WW                                              | |         WW
WW                          _.-\"\"\"-.          _/ /          WW
WW                        _/_______ `.       |__/           WW
WW                       / _______ \\  \\                     WW
WW                       \\/,-. ,-.\\/   \\                    WW
WW                       ()>=   =<()`._ \\_                  WW
WW                     ,-(.--(_)--.)`-.`(_)                 WW
WW                   ,'  /.-'\\_/`-.\\   `.                   WW
WW                  /   /    `-'    \\    \\                  WW
WW                ,'    \\           /     `.                WW
WW               /     _ `.       ,'  _     \\               WW
WW              /     _/   `-._.-'    \\_     \\              WW
WW             /_______|     -|O      |_______\\             WW
WW            {________}______|_______{________}            WW
WW            ,'   _ \\(_____[|_=]______)  / _   `.          WW
WW           /    / `'--------------------`' \\    \\         WW
WW           `---'  |_____________________|   `---'         WW
WW                    |_____|_____|_____|                   WW
WW                    |__|_____|_____|__|                   WW
WW        _           |_____|_____|_____|  hjw              WW
WW       | |           _                                    WW
WW       | |          (_)        _                          WW
WW  ___  | |__    ___  _   ___  | |_  ___  _    ____   ___  WW
WW /  _| |  _ \\  / __|| | / __| |  _||   V  \\  / _  | / __| WW
WW(  (_  | | \\ || /   | | \\__ \\ | |  | /\\ /\\ |( (_| | \\__ \\ WW
WW \\___| |_| |_||_|   |_| |___/ |_|  |_||_||_| \\____| |___/ WW
WW                                                          WW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW


".to_string()
                    },
        // _ => {"".to_string()},
    }


}

fn cls() {

    clearscreen::clear().expect("Failed to clear screen");

}
