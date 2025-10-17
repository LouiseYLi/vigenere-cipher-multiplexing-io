use std::net::IpAddr;

pub fn validate_args(args: &[String]) -> Result<String, String> {
    validate_length(args.len() as i32)?;
    validate_ip(args)?;
    validate_port(args)?;
    validate_delays(args)?;
    let formatted_ip_at_port = format_ip_port(args);
    #[allow(clippy::needless_return)]
    return Ok(formatted_ip_at_port);
}

fn validate_length(vector_length: i32) -> Result<(), String> {
    const MAX_ARGS: i32 = 5;
    if vector_length != MAX_ARGS {
        return Err(format!(
            "Invalid number of args... Expected {}, actual {}",
            MAX_ARGS, vector_length
        ));
    }

    Ok(())
}

fn validate_ip(args: &[String]) -> Result<(), String> {
    // ::1 ipv6 loopback addr
    if args[1].parse::<IpAddr>().is_err() && args[1] != "::1" {
        return Err(format!(
            "Invalid IP address... Neither IPv4 or IPv6: {}",
            args[1]
        ));
    }
    Ok(())
}

fn validate_port(args: &[String]) -> Result<(), String> {
    if args[2] < 0.to_string() || args[2] > u16::MAX.to_string() {
        return Err(format!(
            "Invalid port... must be between 0-65535: {}",
            args[2]
        ));
    }
    Ok(())
}

fn format_ip_port(args: &[String]) -> String {
    if args[1].contains(':') {
        return format!("[{}]:{}", args[1], args[2]);
    }
    #[allow(clippy::needless_return)]
    return format!("{}:{}", args[1], args[2]);
}

fn validate_delays(args: &[String]) -> Result<(), String> {
    let max_delay = 30;
    if args[3].parse::<u16>().is_err() || args[4].parse::<u16>().is_err() {
        return Err(format!(
            "Invalid delay provided... must be between 1-10: actual min: {} actual max: {}",
            args[3], args[4]
        ));
    }
    let min = args[3].parse::<u16>().unwrap();
    let max = args[4].parse::<u16>().unwrap();
    if (min < 1 || min > max_delay) || (max <= min || max > max_delay) {
        return Err(format!(
            "Invalid delay provided... must be between 1-30: actual min: {} actual max: {}",
            args[3], args[4]
        ));
    }
    Ok(())
}

pub fn get_delay(args: &[String], index: usize) -> u16 {
    #[allow(clippy::needless_return)]
    return args[index].parse::<u16>().unwrap();
}
