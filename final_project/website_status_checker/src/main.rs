use chrono::{Utc, DateTime}; //timestamps
use serde::Serialize; //allow serializing to JSON
use std::sync::{
    atomic::{AtomicBool, Ordering}, //atomic boolean for graceful shutdown
    mpsc, Arc, //channels and share state between threads
};
use std::thread; 
use std::time::{Duration, Instant}; //timing and sleeps

//data structure returned by each check
#[derive(Debug, Serialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>, // Ok(status_code) or Err(error_message)
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

impl WebsiteStatus {
    // Convert to JSON string for convenience
    pub fn to_json(&self) -> String {
        let status_value = match &self.status {
            Ok(code) => serde_json::json!({ "code": code }),
            Err(e) => serde_json::json!({ "error": e }),
        };
        serde_json::json!({
            "url": self.url,
            "status": status_value,
            "response_time": self.response_time.as_millis(),
            "timestamp": self.timestamp.to_rfc3339(),
        })
        .to_string()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Configuration 
    let timeout = Duration::from_secs(5); //how long before treating each timeout per request
    let max_retries: usize = 2; //how many times to retry each request
    let check_interval = Duration::from_secs(5); //how often to check
    //50 urls to check
    let urls = vec![
        "https://www.youtube.com/".to_string(),
        "https://www.facebook.com/".to_string(),
        "https://discord.com/".to_string(),
        "https://www.roblox.com/".to_string(),
        "https://www.chase.com/".to_string(),
        "https://www.wellsfargo.com/".to_string(),
        "https://brightspace.utrgv.edu/d2l/home".to_string(),
        "https://my.utrgv.edu/group/myutrgv/home".to_string(),
        "https://www.crunchyroll.com/".to_string(),
        "https://www.netflix.com/browse".to_string(),
        "https://store.steampowered.com/".to_string(),
        "https://www.tiktok.com/".to_string(),
        "https://www.instagram.com/".to_string(),
        "https://www.reddit.com/".to_string(),
        "https://github.com/".to_string(),
        "https://open.spotify.com/".to_string(),
        "https://www.icloud.com/".to_string(),
        "https://www.google.com/".to_string(),
        "https://www.target.com/".to_string(),
        "https://www.heb.com/".to_string(),
        "https://www.walmart.com/".to_string(),
        "https://www.xbox.com/".to_string(),
        "https://www.microsoft.com/".to_string(),
        "https://www.vans.com/".to_string(),
        "https://www.adidas.com/".to_string(),
        "https://www.nike.com/".to_string(),
        "https://www.hulu.com/".to_string(),
        "https://www.disneyplus.com/".to_string(),
        "https://www.riotgames.com/".to_string(),
        "https://twitter.com/".to_string(),
        "https://www.amazon.com/".to_string(),
        "https://www.ebay.com/".to_string(),
        "https://www.twitch.tv/".to_string(),
        "https://www.linkedin.com/".to_string(),
        "https://www.pinterest.com/".to_string(),
        "https://www.snapchat.com/".to_string(),
        "https://www.udemy.com/".to_string(),
        "https://www.coursera.org/".to_string(),
        "https://www.khanacademy.org/".to_string(),
        "https://stackoverflow.com/".to_string(),
        "https://www.quora.com/".to_string(),
        "https://www.medium.com/".to_string(),
        "https://www.paypal.com/".to_string(),
        "https://www.venmo.com/".to_string(),
        "https://cash.app/".to_string(),
        "https://www.trello.com/".to_string(),
        "https://slack.com/".to_string(),
        "https://zoom.us/".to_string(),
        "https://www.dropbox.com/".to_string(),
        "https://drive.google.com/".to_string(),
    ];

    // Ctrl-C for graceful shutdown flag to stop all threads
    let running = Arc::new(AtomicBool::new(true));
    {
        let run = running.clone();
        ctrlc::set_handler(move || {
            eprintln!("\nCtrl+C received. Shutting down gracefully...");
            run.store(false, Ordering::Relaxed);
        })?;
    }

    //Channel to collect results
    let (tx, rx) = mpsc::channel::<WebsiteStatus>();
    //Worker threads (one per URL):
    // - repeatedly attempts to fetch URL
    // - measures response time
    // - sends WebsiteStatus back to main thread using tx
    // - sleeps between checks
    let mut handles = Vec::with_capacity(urls.len());
    for url in urls {
        let tx = tx.clone();
        let running = running.clone();

        let handle = thread::spawn(move || {
            //each worker repeatedly checks its URL until running==false (shutdown requested)
            while running.load(Ordering::Relaxed) {
                let mut response_time = Duration::ZERO;
                let mut status_res: Result<u16, String> = Err("no attempt".into());

                //retry until max_retries reached or success
                for attempt in 0..=max_retries {
                    let start = Instant::now();

                    //create inner channel so the short-lived inner thread can return its result
                    //the inner thread will block on ureq call
                    let (resp_tx, resp_rx) = mpsc::channel::<Result<u16, String>>();
                    let url_clone = url.clone();
                    
                    //spawn the inner thread to perform the blocking request
                    //it's intentionally small and separated in case it blocks, the timout will finish later
                    //and the worker wont wait for it
                    thread::spawn(move || {
                        // Blocking ureq call inside inner thread
                        let result: Result<u16, String> = match ureq::get(&url_clone).call() {
                            Ok(resp) => Ok(u16::from(resp.status())), //convert status code to u16 (for return type)
                            Err(e) => Err(format!("{}", e)),
                        };
                        //ignore send errors (receiver might have timed out/been dropped)
                        let _ = resp_tx.send(result);
                    });

                    //wait for the inner thread result or timeout
                    match resp_rx.recv_timeout(timeout) {
                        Ok(Ok(code)) => {
                            response_time = start.elapsed();
                            status_res = Ok(code);
                            break; // success
                        }
                        Ok(Err(err_str)) => {
                            response_time = start.elapsed();
                            status_res = Err(err_str);
                            // will retry if attempts remain
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            // timed out waiting for the request result
                            response_time = start.elapsed();
                            let to_str = format!("timeout after {} ms", timeout.as_millis());
                            status_res = Err(to_str);
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            response_time = start.elapsed();
                            let to_str = "internal channel disconnected".to_string();
                            status_res = Err(to_str);
                        }
                    }

                    //simple backoff before next retry (if failing and more attempts remain)
                    if attempt < max_retries {
                        let backoff = Duration::from_millis(100 * (attempt as u64 + 1));
                        thread::sleep(backoff);
                    }
                } // end attempts loop

                //build the WebsiteStatus and send to main thread (ignore send errors)
                let report = WebsiteStatus {
                    url: url.clone(),
                    status: status_res,
                    response_time,
                    timestamp: Utc::now(),
                };
                if tx.send(report).is_err() {
                    //main thread gone so exit
                    break;
                }

                //sleep until next check or until shutdown requested
                let mut slept = Duration::ZERO;
                while slept < check_interval && running.load(Ordering::Relaxed) {
                    let step = Duration::from_millis(200);
                    thread::sleep(step);
                    slept += step;
                }
            } // end worker while
        });
        handles.push(handle);
    }

    //drop the original sender in main so channel closes once all workers finish
    drop(tx);

    //collect and print results until channel closes
    while let Ok(status) = rx.recv() {
        println!(
            "[{}] {} -> {:?} ({} ms)",
            status.timestamp.to_rfc3339(),
            status.url,
            status.status,
            status.response_time.as_millis()
        );
    }

    //join worker threads for cleanup
    for h in handles {
        let _ = h.join();
    }

    println!("Exit complete.");
    Ok(())
}
