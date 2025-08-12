use serde_json::Value;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

//trait that defines fetching and saving prices
trait Pricing {
    fn name(&self) -> &str;
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, price: f64) -> Result<(), String>;
}
//helper function to check if year is leap year (for time check)
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
//helper function to ensure correct number of days in a year.
fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) { 366 } else { 365 }
}
//helper function to match each month to its number of days
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}
//Helper function to convert a Unix timestamp into month/date/year format.
fn timestamp_to_date(ts: u64) -> String {
    let mut seconds = ts; //ts = timestamp in seconds
    //unix epoch starts at 1970-01-01 00:00:00 UTC
    let mut year = 1970;
    //number of seconds in a day
    let secs_per_day = 86400;
    //calculate the year
    loop {
        let days = days_in_year(year) as u64;
        let secs_in_year = days * secs_per_day;
        if seconds >= secs_in_year {
            seconds -= secs_in_year;
            year += 1;
        } else {
            break;
        }
    }
    //calculate month
    let mut month = 1;
    loop {
        let dim = days_in_month(year, month) as u64;
        let secs_in_month = dim * secs_per_day;
        if seconds >= secs_in_month {
            seconds -= secs_in_month;
            month += 1;
        } else {
            break;
        }
    }
    //calculate day
    let day = (seconds / secs_per_day) + 1;
    seconds %= secs_per_day;
    //calculate hour, minute, second
    let hour = seconds / 3600;
    seconds %= 3600;

    let minute = seconds / 60;
    let second = seconds % 60;
    format!("{:02}/{:02} {:02}:{:02}:{:02}", month, day, hour, minute, second)
}
//Helper function: appends a line to a file by reading existing content
//and writing it back with the new line appended.
fn append_line_via_read_write(filename: &str, line: &str) -> Result<(), String> {
    use std::io::BufWriter;

    //read existing content from file
    let mut contents = match read_to_string(filename) {
        Ok(s) => s,
        Err(_) => String::new(), // If file missing, start fresh
    };
    //make sure the file ends with a newline if it has content
    if !contents.is_empty() && !contents.ends_with('\n') {
        contents.push('\n');
    }
    //append the new line
    contents.push_str(line);
    contents.push('\n');
    //open file for writing/overwriting
    let file = File::create(filename).map_err(|e| format!("Create file error: {}", e))?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(contents.as_bytes()).map_err(|e| format!("Write file error: {}", e))?;

    writer.flush().map_err(|e| format!("Flush error: {}", e))?;
    Ok(()) //verify
}
//define all structs (bitcoin, ethereum, sp500) that will use pricing trait
struct Bitcoin {
    name: String,
    coin_id: String,
}

struct Ethereum {
    name: String,
    coin_id: String,
}

struct SP500 {
    name: String,
    url: String,
}

//use pricing trait for each struct

impl Pricing for Bitcoin {
    fn name(&self) -> &str {
        &self.name
    }
    //customize the trait for bitcoin using CoinGecko api endpoint
    fn fetch_price(&self) -> Result<f64, String> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            self.coin_id
        );
        //make http request using ureq 
        let resp = ureq::get(&url)
            .call()
            .map_err(|e| format!("HTTP request error for {}: {}", &self.name, e))?
            .into_string()
            .map_err(|e| format!("Read body error for {}: {}", &self.name, e))?;
        //parse the response as JSON
        let v: Value =
            serde_json::from_str(&resp).map_err(|e| format!("JSON parse error for {}: {}", &self.name, e))?;

        v.get(&self.coin_id)
            .and_then(|m| m.get("usd"))
            .and_then(|p| p.as_f64())
            .ok_or_else(|| format!("usd price not found in CoinGecko response for {}", &self.coin_id))
    }
    //save the pricne to file named bitcoin.txt
    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let filename = format!("{}.txt", self.name.to_lowercase());
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();
        let real_date = timestamp_to_date(ts);
        let line = format!("{}, ${}", real_date, price);
        append_line_via_read_write(&filename, &line)
    }
}

impl Pricing for Ethereum {
    fn name(&self) -> &str {
        &self.name
    }
    //customize the trait for ethereum using CoinGecko api endpoint
    fn fetch_price(&self) -> Result<f64, String> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            self.coin_id
        );
        //make http request using ureq 
        let resp = ureq::get(&url)
            .call()
            .map_err(|e| format!("HTTP request error for {}: {}", &self.name, e))?
            .into_string()
            .map_err(|e| format!("Read body error for {}: {}", &self.name, e))?;
        //parse the response as JSON
        let v: Value =
            serde_json::from_str(&resp).map_err(|e| format!("JSON parse error for {}: {}", &self.name, e))?;

        v.get(&self.coin_id)
            .and_then(|m| m.get("usd"))
            .and_then(|p| p.as_f64())
            .ok_or_else(|| format!("usd price not found in CoinGecko response for {}", &self.coin_id))
    }
    //save the price to file named ehtereum.txt
    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let filename = format!("{}.txt", self.name.to_lowercase());
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();
        let real_date = timestamp_to_date(ts);
        let line = format!("{}, ${}", real_date, price);
        append_line_via_read_write(&filename, &line)
    }
}

impl Pricing for SP500 {
    fn name(&self) -> &str {
        &self.name
    }
    //customize the trait for sp500 using yahoo finance api endpoint
    //Note: This endpoint failed to return expected data. Could not find a stable public endpoint for SP500.
    fn fetch_price(&self) -> Result<f64, String> {
         //make http request using ureq
        let resp = ureq::get(&self.url)
            .call()
            .map_err(|e| format!("HTTP request error for {}: {}", &self.name, e))?
            .into_string()
            .map_err(|e| format!("Read body error for {}: {}", &self.name, e))?;
        let v: Value = serde_json::from_str(&resp)
            .map_err(|e| format!("JSON parse error for {}: {}", &self.name, e))?;
        //parse price from Yahoo Finance JSON response
        if let Some(price) = v.pointer("/quoteResponse/result/0/regularMarketPrice").and_then(|p| p.as_f64()) {
            Ok(price)
        } else {
            Err(format!("Price not found in Yahoo Finance response"))
        }
    }
    //save the price to file named sp500.txt
    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let filename = format!("{}.txt", self.name.to_lowercase());
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();
        let real_date = timestamp_to_date(ts);
        let line = format!("{}, ${}", real_date, price);
        append_line_via_read_write(&filename, &line)
    }
}


fn main() {
    //initialize the assets with their respective structs
    let btc = Bitcoin {
        name: "Bitcoin".to_string(),
        coin_id: "bitcoin".to_string(),
    };

    let eth = Ethereum {
        name: "Ethereum".to_string(),
        coin_id: "ethereum".to_string(),
    };

    let sp500 = SP500 {
        name: "SP500".to_string(),
        url: "https://query1.finance.yahoo.com/v7/finance/quote?symbols=%5EGSPC".to_string(),
    };
    //create a vector of assets that implement the pricing trait.
    let assets: Vec<Box<dyn Pricing>> = vec![Box::new(btc), Box::new(eth), Box::new(sp500)];

    println!("Data fetch started. Press Ctrl+C to stop.");

    loop {
        //go through the assets to fetch and save the prices of each trait.
        for a in assets.iter() {
            match a.fetch_price() {
                Ok(price) => match a.save_to_file(price) {
                    Ok(_) => println!("{} saved: ${}", a.name(), price),
                    Err(e) => eprintln!("{} save error: {}", a.name(), e),
                },
                Err(e) => eprintln!("{} fetch error: {}", a.name(), e),
            }
        }
        //wait 10 seconds
        println!("Data fetched. Waiting for next fetch...");
        sleep(Duration::from_secs(10));
    }
}
