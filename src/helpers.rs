use regex::Regex;

pub fn get_consumer_name(prefix: String) -> String {
    let thread_id = std::thread::current().id();
    let str_thread_id = format!("{:?}", thread_id);
    let re = match Regex::new(r"\d+"){
        Ok(re) => re,
        Err(e) => {
            log::error!("Failed to create regex: {:?}", e);
            return prefix;
        }
    };
    let caps = re.captures(&str_thread_id);
    match caps {
        Some(caps) => {
            let id = match caps.get(0) {
                Some(id) => id.as_str(),
                None => {
                    log::error!("Failed to get thread id");
                    return prefix;
                }
            };
            format!("{}-{}", prefix, id)
        },
        None => prefix
    }
}
