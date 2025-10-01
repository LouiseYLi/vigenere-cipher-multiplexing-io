use std::net::IpAddr;

pub fn validate_args(args: &[String]) -> Result<String, String> {
    validate_length(args.len() as i32)?;
    validate_shift_value(args)?;
    validate_ip(args)?;
    validate_port(args)?;
    let formatted_ip_at_port = format_ip_port(args); 
    return Ok(formatted_ip_at_port);
}

fn validate_length(vector_length: i32) -> Result<(), String> {
    const MAX_ARGS: i32 = 5;
    if vector_length != MAX_ARGS {
        return Err(format!(
            "Invalid number of args... Expected 4, actual {}",
            vector_length
        ));
    }
    Ok(())
}

fn validate_shift_value(args: &[String]) -> Result<(), String> {
    if args[2].trim().parse::<i32>().is_err() {
        return Err(format!(
            "Invalid shift value... Expected integer, actual {}",
            args[2]
        ));
    }
    Ok(())
}

fn validate_ip(args: &[String]) -> Result<(), String> {
    // ::1 ipv6 loopback addr
    if args[3].parse::<IpAddr>().is_err() && args[3] != "::1" {
        return Err(format!(
            "Invalid IP address... Neither IPv4 or IPv6: {}",
            args[3]
        ));
    }
    Ok(())
}

fn validate_port(args: &[String]) -> Result<(), String> {
    if args[4] < 0.to_string() || args[4] > u16::MAX.to_string() {
        return Err(format!(
            "Invalid port... must be between 0-65535: {}",
            args[4]
        ));
    }
    Ok(())
}

fn format_ip_port(args: &[String]) -> String {
    if args[3].contains(':') {
        return format!("[{}]:{}", args[3], args[4])
    }
    return format!("{}:{}", args[3], args[4])
}
