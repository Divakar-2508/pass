pub mod toml_reader;

use mysql::{prelude::Queryable, *};
use std::{env, io::Write};

//help function
fn help(arg: String) {
    match arg.as_str() {
        "show" => {
            println!("\n'show' - Used to display records");
            println!("\nUsage:");
            println!("\tpass show [command (optional)] [arguments (optional)]");
            println!("\nArguments:");
            println!("\tsite name, user name, password");
            println!("\nExample1:");
            println!("\tpass show google.com blaze2508 pass@123");
            println!("\nNote: \n\tfor leftout values, please use '-'\n");
            println!("Example2:\n\tpass show google.com - pass@123 \n\tpass show - blaze -");
        }
        "add" => {
            println!("\n'add' - Add new records");
            println!("\ncmd Usage:");
            println!("\tpass add [arguments (mandatory)]");
            println!("\nArguments:");
            println!("\tSite name, User name, Password");
            println!("\nExample:");
            println!("\tpass add google.com blaze pass@123");
        }
        "del" => {
            println!("\n'del' - delete existing records");
            println!("\nUsage:");
            println!("\tpass del [arguments]");
            println!("\tall the credentials with the site name will be displayed from which you can delete the records");
            println!("\tExample: pass del google");
        }
        _ => {
            println!("\nCLI Tool - pass");
            println!("Save Site, username, password");
            println!("\nUsage:");
            println!("\tpass [command] [options]");
            println!("\ncommands:");
            println!("\tadd - add new entry");
            println!("\tshow - show all/specified entries");
            println!("\nExample:");
            println!("\tpass add google.com blaze pass@2508");
            println!("\nFor more information about specific command, use 'pass help [Command]'");
        }
    }
}

//delete record from database
fn delete(arg: String, conn: Pool) {
    let mut query = String::from("delete from passwords where ");
    let result = row_with_index(conn.get_conn().unwrap(), arg);

    let result = match result {
        Some(row) => row,
        None => {
            return;
        }
    };

    let mut record_no = String::new();
    print!("\nEnter the S.No to delete: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut record_no).unwrap();

    let record_no = match record_no.trim().parse::<usize>() {
        Ok(x) => {
            if x > 0 && x <= result.len() {
                x-1
            } else {
                println!("Please enter a valid record index");
                return;
            }
        },
        Err(_e) => {
            println!("Please enter a valid record");
            return;
        }
    };

    let row = &result[record_no];
    let site_name: String = row.get("site_name").unwrap();
    let user_name: String = row.get("user_name").unwrap();
    let password: String = row.get("password").unwrap();

    println!("\nRecord:");
    println!("\n{:^13}\t{:^13}\t{:^13}", site_name, user_name, password);
    
    print!("\nYou sure you want to delete this record? (y/n): ");
    std::io::stdout().flush().unwrap();

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    if !(choice.trim().to_lowercase() == "y") {
        println!("\nAborted");
        return;
    }

    let mut conn = conn.get_conn().unwrap();
    query.push_str(&format!("site_name = '{}' and user_name = '{}' and password = '{}'",
    site_name, user_name, password));
    match conn.query_drop(query){
        Ok(_msg) => println!("\nRecord Removed!"),
        Err(err) => {
            println!("Can't remove record: {:?}", err);
            return;
        }
    };
}

//Show the records filtered with site_name and return them.
fn row_with_index(mut conn : PooledConn, arg: String) -> Option<Vec<Row>>{
    let query = format!("select * from passwords where site_name like '%{}%'", arg);
    let result: Vec<Row> = conn.query(query).unwrap();

    if result.is_empty() {
        println!("No records found with site name '{}'", arg);
        return None;
    }

    println!(
        "\n{:^5}\t{:^13}\t{:^13}\t{:^13}\n",
        "S.No", "Site Name", "User Name", "Password"
    );

    for (i,row) in result.iter().enumerate() {
        let site: String = row.get("site_name").unwrap();
        let user: String = row.get("user_name").unwrap();
        let password: String = row.get("password").unwrap();
        println!("{:^5}\t{:^13}\t{:^13}\t{:^13}", i+1, site, user, password);
    }

    Some(result)
}

//Insert new record into database
fn add(args: Vec<String>, mut conn: PooledConn) {
    let query = format!(
        "insert into passwords values('{}','{}','{}')",
        args[0], args[1], args[2]
    );
    conn.query_drop(query).unwrap();
    println!("Record added successfully!");
}

//Show all records in database, with filters if specified
fn show(mut args: Vec<String>, mut conn: PooledConn) {
    let mut query = String::from("select * from passwords");

    args.retain(|x| x != "all");

    if args.len() > 0 && args.len() < 3 {
        println!("Invalid Format");
        println!("Usage:");
        println!("'pass show [site name] [user name] [password]', Use '-' for empty fields instead");
        println!("Try 'pass show help' for more information");
        return;
    }

    if args.len() != 0 {
        query.push_str(" where ");
        if args[0] != "-" {
            query.push_str(&format!("site_name = '{}'", args[0]));
        }
        if args[1] != "-" {
            if args[0] != "-" {
                query.push_str(" and ");
            }
            query.push_str(&format!(" user_name = '{}'", args[1]));
        }
        if args[2] != "-" {
            if args[0] != "-" || args[1] != "-" {
                query.push_str(" and ");
            }
            query.push_str(&format!(" password = '{}' ", args[0]));
        }
    }

    let ans: Vec<Row> = conn.query(query).unwrap();

    if ans.is_empty() {
        println!("No Such Records Found!'");
        println!("Need help with show? try 'pass help show'");
        return;
    }
    println!(
        "\n{:^5}\t{:^13}\t{:^13}\t{:^13}\n",
        "S.No", "Site Name", "User Name", "Password"
    );
    for (i,row) in ans.iter().enumerate() {
        let site: String = row.get("site_name").unwrap();
        let user: String = row.get("user_name").unwrap();
        let password: String = row.get("password").unwrap();
        println!("{:^5}\t{:^13}\t{:^13}\t{:^13}", i+1, site, user, password);
    }

}


//Driver functions
fn main() {
    // let curr = env::current_dir().unwrap();
    // println!("{:?}", curr.display());
    let data : Option<toml_reader::Data> = toml_reader::toml_read();

    let user;
    let password;
    let host;
    let port;
    let db;
    
    if let Some(data) = data {
        user = data.database.username;
        password = data.database.password;
        host = data.database.host;
        port = data.database.port;
        db = data.database.db
    }
    else {
        println!("can't get the database information, check if config.toml file exists");
        return;
    }
    
    let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, db);
    let opt = Opts::from_url(&url).unwrap();

    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        println!("\nInvalid arguments, try 'pass help'");
        return;
    }

    let command = &args[0].to_string();

    if command == "help" {
        if args.contains(&"add".to_string()) {
            help("add".to_string());
        } else if args.contains(&"show".to_string()) {
            help("show".to_string());
        } else {
            help("general".to_string());
        }
        return;
    } else if command == "add" && args.len() != 4{
        println!("\nPlease Specify [Site Name] [User-Name] [Password] for 'add'");
        println!("Ex: pass add google blaze pass@2508");
        return;
    } else if command == "del" && args.len() != 2 {
        println!("\nPlease Specify [Site Name]");
        println!("Ex: pass del google");
        println!("Don't worry, it just displays filtered records with site name for easier deletion");
        return;
    }

    let pool = Pool::new(opt).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(
        "create table if not exists passwords (
            site_name varchar(50),
            user_name varchar(50),
            password varchar(50)
        )",
    )
    .unwrap();

    match command.as_str() {
        "show" => show(args[1..].to_vec(), conn),
        "add" => add(args[1..].to_vec(), conn),
        "del" => delete(args[1].to_string(), pool),
        _ => println!("Invalid command: {}, Try running 'pass help'", command),
    }
}
